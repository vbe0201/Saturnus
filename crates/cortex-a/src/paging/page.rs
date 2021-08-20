//! Abstractions for virtual memory pages.

use super::{PhysAddr, VirtAddr};

mod sealed {
    pub trait Sealed {}
}

macro_rules! define_page_sizes {
    ($(#[$doc:meta] $name:ident = $size:expr),*$(,)?) => {
        $(
            #[$doc]
            pub const $name: usize = $size;
            impl sealed::Sealed for PageSize<$name> {}
            impl SupportedPageSize for PageSize<$name> {}
        )*
    };
}

#[rustfmt::skip]
define_page_sizes![
    /// 4 KiB large virtual memory page.
    _4K = 4 << 10,
    /// 2 MiB large virtual memory page.
    _2M = 2 << 20,
    /// 1 GiB large virtual memory page.
    _1G = 1 << 30,

    /// 16 KiB large virtual memory page.
    _16K = 16 << 10,
    /// 32 MiB large virtual memory page.
    _32M = 32 << 20,

    /// 512 MiB large virtual memory page.
    _512M = 512 << 20,
    /// 64 KiB large virtual memory page.
    _64K = 64 << 10,
];

/// A struct which is used in combination with the [`SupportedPageSize`] trait
/// to ensure that the const generic represents a valid page size.
pub struct PageSize<const SIZE: usize>;

/// Trait that is used to ensure a [`PageSize`] object is valid.
pub trait SupportedPageSize: sealed::Sealed {}

/// Error type used to indicate that a virtual address is not aligned
/// to the page size.
#[derive(Debug)]
pub struct UnalignedVirtAddr(VirtAddr);

/// Error type used to indicate that a physical address is not aligned
/// to the page size.
#[derive(Debug)]
pub struct UnalignedPhysAddr(PhysAddr);

/// A virtual memory page, which size is specified by the const generic argument.
///
/// A page is guaranteed to be aligned to it's page size.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Page<const SIZE: usize>
where
    PageSize<SIZE>: SupportedPageSize,
{
    start: VirtAddr,
}

impl<const SIZE: usize> Page<SIZE>
where
    PageSize<SIZE>: SupportedPageSize,
{
    /// Creates a new page that starts at the given address.
    ///
    /// # Returns
    ///
    /// An error if the given address is not aligned to the page size.
    #[inline]
    pub fn from_start_address(start: VirtAddr) -> Result<Self, UnalignedVirtAddr> {
        match start.is_aligned(SIZE) {
            true => Ok(Self { start }),
            false => Err(UnalignedVirtAddr(start)),
        }
    }

    /// Creates a new page that starts at the given address, without checking if
    /// the address is aligned.
    #[inline]
    pub unsafe fn from_start_address_unchecked(start: VirtAddr) -> Self {
        Self { start }
    }

    /// Creates the page that contains the given virtual address, by aligning the
    /// given address downwards.
    #[inline]
    pub fn containing_address(addr: VirtAddr) -> Self {
        Self {
            start: addr.align_down(SIZE),
        }
    }

    /// Returns the starting address of this page.
    #[inline]
    pub fn start(self) -> VirtAddr {
        self.start
    }
}

/// A physical memory frame, which size is specified by the const generic argument.
///
/// A physical frame is guaranteed to be aligned to it's page size.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PhysFrame<const SIZE: usize>
where
    PageSize<SIZE>: SupportedPageSize,
{
    start: PhysAddr,
}

impl<const SIZE: usize> PhysFrame<SIZE>
where
    PageSize<SIZE>: SupportedPageSize,
{
    /// Creates a new page that starts at the given address.
    ///
    /// # Returns
    ///
    /// An error if the given address is not aligned to the page size.
    #[inline]
    pub fn from_start_address(start: PhysAddr) -> Result<Self, UnalignedPhysAddr> {
        match start.is_aligned(SIZE) {
            true => Ok(Self { start }),
            false => Err(UnalignedPhysAddr(start)),
        }
    }

    /// Creates a new page that starts at the given address, without checking if
    /// the address is aligned.
    #[inline]
    pub unsafe fn from_start_address_unchecked(start: PhysAddr) -> Self {
        Self { start }
    }

    /// Creates the page that contains the given virtual address, by aligning the
    /// given address downwards.
    #[inline]
    pub fn containing_address(addr: PhysAddr) -> Self {
        Self {
            start: addr.align_down(SIZE),
        }
    }

    /// Returns the starting address of this page.
    #[inline]
    pub fn start(self) -> PhysAddr {
        self.start
    }
}
