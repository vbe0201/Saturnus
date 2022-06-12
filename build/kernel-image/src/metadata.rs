use std::{io, mem::size_of};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

/// The magic value of the Kernel Loader.
pub const KERNEL_LOADER_MAGIC: &[u8; 4] = b"SLD0";

/// The magic value of the Kernel.
pub const KERNEL_MAGIC: &[u8; 4] = b"SKN0";

/// Representation of the Kernel metadata map.
///
/// This must be kept in sync with actual kernel code
/// at all times. Data type changes between different
/// architectures must also be accounted for here.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct KernelMeta {
    /// The 4 bytes kernel magic value.
    pub magic: u32,
    /// The offset to the serialized INI1 blob which
    /// holds all the Kernel Initial Processes.
    pub ini1_base: u64,
    /// The base address of the Kernel Loader blob.
    pub loader_base: u64,
    /// The current kernel version.
    pub version: u32,
    /// The current layout of the kernel binary.
    pub layout: KernelLayout,
}

impl KernelMeta {
    /// Deserializes a meta object from a given reader.
    pub fn read(mut data: &[u8]) -> io::Result<Self> {
        Ok(Self {
            magic: data.read_u32::<LE>()?,
            ini1_base: data.read_u64::<LE>()?,
            loader_base: data.read_u64::<LE>()?,
            version: data.read_u32::<LE>()?,
            layout: KernelLayout::read(data)?,
        })
    }

    /// Gets the binary size of the meta object.
    pub fn size(&self) -> usize {
        size_of::<u32>() * 2 + size_of::<u64>() * 2 + size_of::<KernelLayout>()
    }

    /// Serializes the meta to a given writer.
    pub fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u32::<LE>(self.magic)?;
        writer.write_u64::<LE>(self.ini1_base)?;
        writer.write_u64::<LE>(self.loader_base)?;
        writer.write_u32::<LE>(self.version)?;
        self.layout.write(writer)?;

        Ok(())
    }
}

/// Representation of the kernel's binary section layout.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct KernelLayout {
    /// End of the kernel .text section.
    pub text_start: u32,
    /// End of the kernel .text section.
    pub text_end: u32,
    /// Start of the kernel .rodata section.
    pub rodata_start: u32,
    /// End of the kernel .rodata section.
    pub rodata_end: u32,
    /// Start of the kernel .data section.
    pub data_start: u32,
    /// End of the kernel .data section.
    pub data_end: u32,
    /// Start of the kernel .bss section.
    pub bss_start: u32,
    /// End of the kernel .bss section.
    pub bss_end: u32,
    /// End of the kernel blob.
    pub kernel_end: u32,
    /// Start of the _DYNAMIC section.
    pub dynamic_start: u32,
    // XXX: Nintendo tracks the start and end of init_array here.
    // But since it's against Rust's philosophy to have code run
    // before main, we can assume we won't need to do any work here.
}

impl KernelLayout {
    /// Deserializes a layout object from a given reader.
    pub fn read(mut data: &[u8]) -> io::Result<Self> {
        Ok(Self {
            text_start: data.read_u32::<LE>()?,
            text_end: data.read_u32::<LE>()?,
            rodata_start: data.read_u32::<LE>()?,
            rodata_end: data.read_u32::<LE>()?,
            data_start: data.read_u32::<LE>()?,
            data_end: data.read_u32::<LE>()?,
            bss_start: data.read_u32::<LE>()?,
            bss_end: data.read_u32::<LE>()?,
            kernel_end: data.read_u32::<LE>()?,
            dynamic_start: data.read_u32::<LE>()?,
        })
    }

    /// Serializes the layout to a given writer.
    pub fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u32::<LE>(self.text_start)?;
        writer.write_u32::<LE>(self.text_end)?;
        writer.write_u32::<LE>(self.rodata_start)?;
        writer.write_u32::<LE>(self.rodata_end)?;
        writer.write_u32::<LE>(self.data_start)?;
        writer.write_u32::<LE>(self.data_end)?;
        writer.write_u32::<LE>(self.bss_start)?;
        writer.write_u32::<LE>(self.bss_end)?;
        writer.write_u32::<LE>(self.kernel_end)?;
        writer.write_u32::<LE>(self.dynamic_start)?;

        Ok(())
    }
}
