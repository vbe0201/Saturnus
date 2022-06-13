//! This crate provides the facilities to build full Saturnus
//! kernel images.
//!
//! This involves stitching the Kernel itself, the Kernel Loader
//! and a number of Kernel Initial Process binaries together into
//! a structured, executable binary format.
//!
//! It is used by the `xtask` crate for the build command.

use std::{
    fs,
    io::{Seek, SeekFrom, Write},
    iter,
    path::Path,
};

use anyhow::{bail, Result};
use memchr::memmem;

mod kip;
pub use self::kip::*;

mod metadata;
pub use self::metadata::*;

const PAGE_SIZE: usize = 0x1000;

/// The builder for the final Kernel Image.
#[derive(Default)]
pub struct ImageBuilder {
    kernel: Vec<u8>,
    kernel_meta: (usize, KernelMeta),

    loader: Vec<u8>,
    loader_meta: (usize, KernelLoaderMeta),

    kips: Vec<u8>,
    kip_count: u8,

    version: u32,
}

impl ImageBuilder {
    /// Loads a raw Kernel binary from the given path and
    /// stores it.
    pub fn with_kernel<P: AsRef<Path>>(mut self, path: P) -> Result<Self> {
        let mut kernel = fs::read(path)?;

        // We try to find the metadata offset for the kernel first.
        // However, it must not be at 0 because the image needs to
        // begin with executable code. At the same time, it is fair
        // to assume it's a logic bug when metadata are *too* far in.
        let finder = memmem::Finder::new(KERNEL_MAGIC);
        let meta_offset = match finder.find(&kernel) {
            Some(off) if off == 0 || off > 0x10 => {
                bail!("suspicious metadata offset found; please confirm")
            }
            Some(off) => off,
            None => panic!("Malformed kernel binary!"),
        };

        // Now deserialize the full kernel meta blob.
        let meta = KernelMeta::read(&kernel[meta_offset..])?;
        assert_eq!(meta.magic, u32::from_le_bytes(*KERNEL_MAGIC));
        assert!(meta.layout.text_start <= meta.layout.text_end);
        assert!(meta.layout.rodata_start <= meta.layout.rodata_end);
        assert!(meta.layout.data_start <= meta.layout.data_end);
        assert!(meta.layout.bss_start <= meta.layout.bss_end);
        assert!(kernel.len() <= meta.layout.kernel_end as usize);

        // If the kernel is not the expected length, pad it.
        let required_padding = meta.layout.kernel_end as usize - kernel.len();
        if required_padding > 0 {
            kernel.extend(iter::repeat(0).take(required_padding));
        }

        // Store the kernel along with its meta.
        self.kernel = kernel;
        self.kernel_meta = (meta_offset, meta);

        Ok(self)
    }

    /// Loads a raw Kernel Loader binary from the given path and
    /// stores it.
    pub fn with_loader<P: AsRef<Path>>(mut self, path: P) -> Result<Self> {
        let loader = fs::read(path)?;

        // We try to find the metadata offset for the loader first.
        // However, it must not be at 0 because the image needs to
        // begin with executable code. At the same time, it is fair
        // to assume it's a logic bug when metadata are *too* far in.
        let finder = memmem::Finder::new(KERNEL_LOADER_MAGIC);
        let meta_offset = match finder.find(&loader) {
            Some(off) if off == 0 || off > 0x10 => {
                bail!("suspicious metadata offset found; please confirm")
            }
            Some(off) => off,
            None => panic!("Malformed kernel binary!"),
        };

        // Now deserialize the full kernel loader meta blob.
        let meta = KernelLoaderMeta::read(&loader[meta_offset..])?;
        assert_eq!(meta.magic, u32::from_le_bytes(*KERNEL_LOADER_MAGIC));
        assert_eq!(meta.marker, 0xCCCCCCCC);

        self.loader = loader;
        self.loader_meta = (meta_offset, meta);

        Ok(self)
    }

    /// Loads a Kernel Initial Process binary from the given path
    /// and stores it.
    ///
    /// KIPs are expected to start with [`KIP_MAGIC`] and the total
    /// number of allowed KIPs is [`MAX_KIP_COUNT`].
    pub fn add_kip<P: AsRef<Path>>(mut self, path: P) -> Result<Self> {
        if self.kip_count > MAX_KIP_COUNT {
            bail!("Number of allowed KIPs exceeded");
        }

        let kip = fs::read(path)?;
        if &kip[..KIP_MAGIC.len()] != KIP_MAGIC {
            bail!("Invalid KIP binary supplied: no header magic found");
        }

        self.kips.extend(kip);
        self.kip_count += 1;

        Ok(self)
    }

    /// Sets the version for the Kernel Image.
    pub fn with_version(mut self, major: u8, minor: u8, micro: u8) -> Self {
        self.version = ((major as u32) << 24) | ((minor as u32) << 16) | ((micro as u32) << 8);
        self
    }

    /// Finalizes the build and writes the resulting Kernel Image
    /// to `outfile`.
    pub fn finalize<P: AsRef<Path>>(mut self, outfile: P) -> Result<()> {
        if self.kernel_meta.0 == 0 || self.loader.is_empty() {
            bail!("Cannot build Kernel Image without at least Kernel and Loader");
        }

        // Build the INI1 header if necessary and determine its length.
        let ini1_header = build_ini1_header(self.kips.len(), self.kip_count);
        let ini1_header_len = ini1_header.as_ref().map(|h| h.len()).unwrap_or(0);

        // Calculate the start and end offsets of the INI1 segment.
        let ini1_start = align_up(self.kernel_meta.1.layout.kernel_end as usize, PAGE_SIZE);
        let ini1_end = ini1_start + ini1_header_len + self.kips.len();

        // Calculate the start and end offsets of the Kernel Loader.
        let loader_start =
            align_up(ini1_end, PAGE_SIZE) + if ini1_header_len == 0 { PAGE_SIZE } else { 0 };
        let loader_end = loader_start + self.loader.len();

        // Update our headers accordingly.
        self.kernel_meta.1.ini1_base = ini1_start as u64;
        self.kernel_meta.1.loader_base = loader_start as u64;
        self.kernel_meta.1.version = self.version;
        self.loader_meta.1.version = self.version;

        // Now build the resulting output binary.
        let mut output = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(outfile)?;
        {
            // Write the initial bits of kernel code.
            output.write_all(&self.kernel[..self.kernel_meta.0])?;

            // Re-serialize the kernel metadata.
            self.kernel_meta.1.write(&mut output)?;

            // Write the remaining bits of kernel code.
            output.write_all(&self.kernel[(self.kernel_meta.0 + self.kernel_meta.1.size())..])?;

            // Write the INI1 record of Kernel Initial Processes.
            output.seek(SeekFrom::Start(ini1_start as u64))?;
            output.write_all(&ini1_header.unwrap_or_default())?;
            output.write_all(&self.kips)?;

            // Write the initial bits of loader code.
            output.seek(SeekFrom::Start(loader_start as u64))?;
            output.write_all(&self.loader[..self.loader_meta.0])?;

            // Re-serialize the loader metadata.
            self.loader_meta.1.write(&mut output)?;

            // Write the remaining bits of loader code.
            output.write_all(&self.loader[(self.loader_meta.0 + self.loader_meta.1.size())..])?;

            // Append trailing padding at an aligned image end.
            output.seek(SeekFrom::Start(align_up(loader_end, PAGE_SIZE) as u64))?;
            output.write_all(&vec![0; PAGE_SIZE])?;
        }

        Ok(())
    }
}

#[inline]
const fn align_up(value: usize, align: usize) -> usize {
    assert!(align.is_power_of_two());
    (value + align - 1) & !(align - 1)
}
