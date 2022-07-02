//! Implementation details of [`crate::addr`].

use core::{fmt, mem::size_of};

use utils::align;

pub const PHYS_ADDR_MASK: usize = 0xFFF0_0000_0000_0000;
pub const VIRT_ADDR_MASK: usize = 0xFFFF_0000_0000_0000;

macro_rules! impl_fmt_traits {
    (for $for:ident) => {
        impl fmt::Debug for $for {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple(stringify!($for))
                    .field(&format_args!("{:#X}", self.0.addr()))
                    .finish()
            }
        }

        impl fmt::Binary for $for {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Binary::fmt(&self.0.addr(), f)
            }
        }

        impl fmt::LowerHex for $for {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::LowerHex::fmt(&self.0.addr(), f)
            }
        }

        impl fmt::Octal for $for {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Octal::fmt(&self.0.addr(), f)
            }
        }

        impl fmt::UpperHex for $for {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::UpperHex::fmt(&self.0.addr(), f)
            }
        }

        impl fmt::Pointer for $for {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Pointer::fmt(&self.0, f)
            }
        }
    };
}

/// A [`PhysAddr`]/[`VirtAddr`] object was attempted to be built
/// from an invalid pointer.
#[derive(Debug)]
pub struct InvalidAddress(usize);

/// A physical memory address.
///
/// This has the memory layout of a pointer and mostly also
/// acts like a wrapper around one.
///
/// It ensures that the high 12 bits of its contained address
/// are always zeroed.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysAddr(*mut ());

/// A virtual memory address.
///
/// This has the memory layout of a pointer and mostly also
/// acts like a wrapper around one.
///
/// It ensures that the high 16 bits of its contained address
/// are either all zeroes or all ones.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtAddr(*mut ());

const _: () = assert!(size_of::<PhysAddr>() == size_of::<*mut ()>());
const _: () = assert!(size_of::<VirtAddr>() == size_of::<*mut ()>());

impl PhysAddr {
    /// Attempts to create a new physical address from the
    /// supplied pointer.
    ///
    /// This will return [`InvalidAddress`] when the highest
    /// 12 bits of the pointed-to memory address are not all
    /// zeroes.
    #[inline(always)]
    pub fn try_new<T>(ptr: *mut T) -> Result<Self, InvalidAddress> {
        let addr = ptr.addr();
        match addr & PHYS_ADDR_MASK {
            0 => Ok(Self(ptr as *mut ())),
            _ => Err(InvalidAddress(addr)),
        }
    }

    /// Creates a new physical address from the supplied pointer.
    ///
    /// # Panics
    ///
    /// See the error conditions of [`PhysAddr::try_new`].
    #[inline(always)]
    pub fn new<T>(ptr: *mut T) -> Self {
        Self::try_new(ptr).unwrap()
    }

    /// Creates a new physical address from the supplied pointer.
    ///
    /// # Safety
    ///
    /// The pointer is not validated, the caller is responsible
    /// for making sure it actually points into physical memory.
    #[inline(always)]
    pub const unsafe fn new_unchecked<T>(ptr: *mut T) -> Self {
        Self(ptr as *mut ())
    }

    /// Gets the referenced memory address as [`usize`].
    #[inline(always)]
    pub fn addr(self) -> usize {
        self.0.addr()
    }

    /// Gets this address as an immutable pointer to a value
    /// of type `T`.
    ///
    /// # Safety
    ///
    /// While this method in itself is safe, special care must
    /// be applied when *using* the resulting pointer:
    ///
    /// When dereferencing or otherwise using said pointer, it is
    /// up to the caller to ensure that the invariants are upheld
    /// for the pointer this [`PhysAddr`] was constructed with as
    /// well as all the transformations (e.g. alignment) applied
    /// to it up until now.
    #[inline(always)]
    pub const fn as_ptr<T>(self) -> *const T {
        self.as_mut_ptr::<T>() as *const T
    }

    /// Gets this address as a mutable pointer to a value of
    /// type `T`.
    ///
    /// # Safety
    ///
    /// While this method in itself is safe, special care must
    /// be applied when *using* the resulting pointer:
    ///
    /// When dereferencing or otherwise using said pointer, it is
    /// up to the caller to ensure that the invariants are upheld
    /// for the pointer this [`PhysAddr`] was constructed with as
    /// well as all the transformations (e.g. alignment) applied
    /// to it up until now.
    #[inline(always)]
    pub const fn as_mut_ptr<T>(self) -> *mut T {
        self.0.cast::<T>()
    }

    /// Aligns the address up to the next multiple of `align`.
    ///
    /// This returns [`InvalidAddress`] when the high 12 bits of
    /// the new pointer would be non-zero.
    ///
    /// In practice, this is equivalent to using
    /// [`pointer::wrapping_offset`], the same capabilities and
    /// restrictions apply.
    ///
    /// # Panics
    ///
    /// Panics when `align` is not a power of two.
    #[inline(always)]
    #[must_use]
    pub fn align_up(self, align: usize) -> Result<Self, InvalidAddress> {
        Self::try_new(self.0.map_addr(|addr| align::align_up(addr, align)))
    }

    /// Aligns the address down to the next multiple of `align`.
    ///
    /// This returns [`InvalidAddress`] when the high 12 bits of
    /// the new pointer would be non-zero.
    ///
    /// In practice, this is equivalent to using
    /// [`pointer::wrapping_offset`], the same capabilities and
    /// restrictions apply.
    ///
    /// # Panics
    ///
    /// Panics when `align` is not a power of two.
    #[inline(always)]
    #[must_use]
    pub fn align_down(self, align: usize) -> Result<Self, InvalidAddress> {
        Self::try_new(self.0.map_addr(|addr| align::align_down(addr, align)))
    }

    /// Checks if this address is aligned to a multiple of `align`.
    ///
    /// # Panics
    ///
    /// Panics when `align` is not a power of two.
    #[inline(always)]
    pub fn is_aligned(self, align: usize) -> bool {
        align::is_aligned(self.0.addr(), align)
    }

    /// Creates a new physical address by mapping `self`'s address
    /// to a new one.
    ///
    /// This returns [`InvalidAddress`] when the high 12 bits of
    /// the new pointer would be non-zero.
    ///
    /// In practice, this is equivalent to using
    /// [`pointer::wrapping_offset`], the same capabilities and
    /// restrictions apply.
    #[inline]
    #[must_use]
    pub fn map_addr(self, f: impl FnOnce(usize) -> usize) -> Result<Self, InvalidAddress> {
        Self::try_new(self.0.map_addr(f))
    }
}

impl VirtAddr {
    /// Attempts to create a new virtual address from the
    /// supplied pointer.
    ///
    /// This will return [`InvalidAddress`] when the highest
    /// 16 bits of the pointed-to memory address are not all
    /// zeroes or all ones.
    #[inline(always)]
    pub fn try_new<T>(ptr: *mut T) -> Result<Self, InvalidAddress> {
        let addr = ptr.addr();
        match addr & VIRT_ADDR_MASK {
            0 | VIRT_ADDR_MASK => Ok(Self(ptr as *mut ())),
            _ => Err(InvalidAddress(addr)),
        }
    }

    /// Creates a new virtual address from the supplied pointer.
    ///
    /// # Panics
    ///
    /// See the error conditions of [`VirtAddr::try_new`].
    #[inline(always)]
    pub fn new<T>(ptr: *mut T) -> Self {
        Self::try_new(ptr).unwrap()
    }

    /// Creates a new virtual address from the supplied pointer.
    ///
    /// # Safety
    ///
    /// The pointer is not validated, the caller is responsible
    /// for making sure it actually points into virtual memory.
    #[inline(always)]
    pub const unsafe fn new_unchecked<T>(ptr: *mut T) -> Self {
        Self(ptr as *mut ())
    }

    /// Gets the referenced memory address as [`usize`].
    #[inline(always)]
    pub fn addr(self) -> usize {
        self.0.addr()
    }

    /// Gets this address as an immutable pointer to a value
    /// of type `T`.
    ///
    /// # Safety
    ///
    /// While this method in itself is safe, special care must
    /// be applied when *using* the resulting pointer:
    ///
    /// When dereferencing or otherwise using said pointer, it is
    /// up to the caller to ensure that the invariants are upheld
    /// for the pointer this [`VirtAddr`] was constructed with as
    /// well as all the transformations (e.g. alignment) applied
    /// to it up until now.
    #[inline(always)]
    pub const fn as_ptr<T>(self) -> *const T {
        self.as_mut_ptr::<T>() as *const T
    }

    /// Gets this address as a mutable pointer to a value of
    /// type `T`.
    ///
    /// # Safety
    ///
    /// While this method in itself is safe, special care must
    /// be applied when *using* the resulting pointer:
    ///
    /// When dereferencing or otherwise using said pointer, it is
    /// up to the caller to ensure that the invariants are upheld
    /// for the pointer this [`VirtAddr`] was constructed with as
    /// well as all the transformations (e.g. alignment) applied
    /// to it up until now.
    #[inline(always)]
    pub const fn as_mut_ptr<T>(self) -> *mut T {
        self.0.cast::<T>()
    }

    /// Aligns the address up to the next multiple of `align`.
    ///
    /// This returns [`InvalidAddress`] when the high 16 bits of
    /// the new pointer would not be all zeroes or all ones.
    ///
    /// In practice, this is equivalent to using
    /// [`pointer::wrapping_offset`], the same capabilities and
    /// restrictions apply.
    ///
    /// # Panics
    ///
    /// Panics when `align` is not a power of two.
    #[inline(always)]
    #[must_use]
    pub fn align_up(self, align: usize) -> Result<Self, InvalidAddress> {
        Self::try_new(self.0.map_addr(|addr| align::align_up(addr, align)))
    }

    /// Aligns the address down to the next multiple of `align`.
    ///
    /// This returns [`InvalidAddress`] when the high 16 bits of
    /// the new pointer would not be all zeroes or all ones.
    ///
    /// In practice, this is equivalent to using
    /// [`pointer::wrapping_offset`], the same capabilities and
    /// restrictions apply.
    ///
    /// # Panics
    ///
    /// Panics when `align` is not a power of two.
    #[inline(always)]
    #[must_use]
    pub fn align_down(self, align: usize) -> Result<Self, InvalidAddress> {
        Self::try_new(self.0.map_addr(|addr| align::align_down(addr, align)))
    }

    /// Checks if this address is aligned to a multiple of `align`.
    ///
    /// # Panics
    ///
    /// Panics when `align` is not a power of two.
    #[inline(always)]
    pub fn is_aligned(self, align: usize) -> bool {
        align::is_aligned(self.0.addr(), align)
    }

    /// Creates a new physical address by mapping `self`'s address
    /// to a new one.
    ///
    /// This returns [`InvalidAddress`] when the high 16 bits of
    /// the new pointer would not be all zeroes or all ones.
    ///
    /// In practice, this is equivalent to using
    /// [`pointer::wrapping_offset`], the same capabilities and
    /// restrictions apply.
    #[inline]
    #[must_use]
    pub fn map_addr(self, f: impl FnOnce(usize) -> usize) -> Result<Self, InvalidAddress> {
        Self::try_new(self.0.map_addr(f))
    }
}

impl_fmt_traits!(for PhysAddr);
impl_fmt_traits!(for VirtAddr);
