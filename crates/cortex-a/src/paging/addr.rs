//! Physical and virtual addresses representation and manipulation.

use core::{fmt, ops};

use crate::utils;

const PHYS_UPPER_BITS_MASK: usize = !utils::bitmask(0, 52);
const VIRT_UPPER_BITS_MASK: usize = !utils::bitmask(0, 48);

/// Tried to create an address that was not valid.
///
/// This means that the upper bits weren't all zeroes or ones.
#[derive(Debug)]
pub struct MalformedAddress(usize);

/// A virtual memory address.
///
/// This is a wrapper type around an `usize`, which guarantees that the upper most
/// bits are either all ones or zeroes. The amount of upper bits is controlled by the
/// [`ADDRESS_BITS`] constant.
///
/// All operator implementations (`Add`, `Sub`, etc) are wrapping operations (including in debug
/// mode) and they will all keep the upper bits unchanged.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VirtAddr(usize);

impl VirtAddr {
    /// Creates a virtual address from the given pointer
    ///
    /// # Panics
    ///
    /// If the upper bits are not all zeros or ones.
    #[inline]
    pub fn from_ptr<T>(ptr: *const T) -> Self {
        Self::new(ptr as usize)
    }

    /// Creates a new virtual address, which is guaranteed to be canonical.
    ///
    /// # Panics
    ///
    /// If the upper bits are not all zeros or ones.
    #[inline]
    pub const fn new(addr: usize) -> Self {
        match Self::try_new(addr) {
            Ok(addr) => addr,
            Err(_) => panic!("VirtAddr::new: address is malformed")
        }
    }

    /// Tries to create a new virtual address, which is guaranteed to be canonical.
    ///
    /// Returns an error if the address is malformed.
    #[inline]
    pub const fn try_new(addr: usize) -> Result<Self, MalformedAddress> {
        match addr & VIRT_UPPER_BITS_MASK {
            0 | 0xFFFF => Ok(Self(addr)),
            _ => Err(MalformedAddress(addr)),
        }
    }

    /// Create a new virtual address without checking if it's malformed.
    ///
    /// # Safety
    ///
    /// The upper bits of the address must be all zeros or ones.
    #[inline]
    pub const unsafe fn new_unchecked(addr: usize) -> Self {
        Self(addr)
    }

    /// Creates a new virtual address of `0`.
    #[inline]
    pub const fn zero() -> Self {
        Self(0)
    }

    /// Converts this address to the inner `usize`.
    #[inline]
    pub const fn as_usize(self) -> usize {
        self.0
    }

    /// Converts the address to a raw pointer.
    #[inline]
    pub const fn as_ptr<T>(self) -> *const T {
        self.as_usize() as *const T
    }

    /// Converts the address to a raw pointer.
    #[inline]
    pub const fn as_mut_ptr<T>(self) -> *mut T {
        self.as_usize() as *mut T
    }

    /// Align this address upwards to the given alignment.
    ///
    /// Note that this method will leave the upper bits of this address unchanged
    ///
    /// # Panics
    ///
    /// If the alignment is not a power of two.
    #[inline]
    pub const fn align_up(self, align: usize) -> Self {
        Self(utils::align_up(self.as_usize(), align))
    }

    /// Align this address downwards to the given alignment.
    ///
    /// Note that this method will leave the upper bits of this address unchanged
    ///
    /// # Panics
    ///
    /// If the alignment is not a power of two.
    #[inline]
    pub const fn align_down(self, align: usize) -> Self {
        Self(utils::align_down(self.as_usize(), align))
    }

    /// Check if this address is aligned to the given alignment.
    ///
    /// # Panics
    ///
    /// If the alignment is not a power of two.
    #[inline]
    pub const fn is_aligned(self, align: usize) -> bool {
        utils::is_aligned(self.as_usize(), align)
    }
}

/// A physical memory address.
///
/// In contrast to a [`VirtAddr`], a physical memory address does not have any
/// guarantes and is basically a wrapper around an `usize`.
///
/// All operator implementations (`Add`, `Sub`, etc) are wrapping operations, even in debug mode.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PhysAddr(usize);

impl PhysAddr {
    /// Creates a new physical address from the given pointer
    #[inline]
    pub fn from_ptr<T>(ptr: *const T) -> Self {
        Self::new(ptr as usize)
    }

    /// Creates a new physical address, which is guaranteed to be canonical.
    ///
    /// # Panics
    ///
    /// If the upper bits are not all zeros.
    #[inline]
    pub const fn new(addr: usize) -> Self {
        match Self::try_new(addr) {
            Ok(addr) => addr,
            Err(_) => panic!("PhysAddr::new: address is malformed")
        }
    }

    /// Tries to create a new physical address, which is guaranteed to be canonical.
    ///
    /// Returns an error if the address is malformed.
    #[inline]
    pub const fn try_new(addr: usize) -> Result<Self, MalformedAddress> {
        match addr & PHYS_UPPER_BITS_MASK {
            0 => Ok(Self(addr)),
            _ => Err(MalformedAddress(addr)),
        }
    }

    /// Create a new physical address without checking if it's malformed.
    ///
    /// # Safety
    ///
    /// The upper bits of the address must be all zeroes or ones.
    #[inline]
    pub const unsafe fn new_unchecked(addr: usize) -> Self {
        Self(addr)
    }

    /// Creates a new virtual address of `0`.
    #[inline]
    pub const fn zero() -> Self {
        Self(0)
    }

    /// Converts this address to the inner `usize`.
    #[inline]
    pub const fn as_usize(self) -> usize {
        self.0
    }

    /// Converts the address to a raw pointer.
    #[inline]
    pub const fn as_ptr<T>(self) -> *const T {
        self.as_usize() as *const T
    }

    /// Converts the address to a raw pointer.
    #[inline]
    pub const fn as_mut_ptr<T>(self) -> *mut T {
        self.as_usize() as *mut T
    }

    /// Align this address upwards to the given alignment.
    ///
    /// # Panics
    ///
    /// If the alignment is not a power of two.
    #[inline]
    pub const fn align_up(self, align: usize) -> Self {
        Self(utils::align_up(self.as_usize(), align))
    }

    /// Align this address downwards to the given alignment.
    ///
    /// # Panics
    ///
    /// If the alignment is not a power of two.
    #[inline]
    pub const fn align_down(self, align: usize) -> Self {
        Self(utils::align_down(self.as_usize(), align))
    }

    /// Check if this address is aligned to the given alignment.
    ///
    /// # Panics
    ///
    /// If the alignment is not a power of two.
    #[inline]
    pub const fn is_aligned(self, align: usize) -> bool {
        utils::is_aligned(self.as_usize(), align)
    }
}

macro_rules! impl_fmt_traits {
    (for $for:ident) => {
        impl fmt::Debug for $for {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_tuple(stringify!($for))
                    .field(&format_args!("{:#X}", self.0))
                    .finish()
            }
        }

        impl fmt::Binary for $for {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Binary::fmt(&self.0, f)
            }
        }

        impl fmt::LowerHex for $for {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::LowerHex::fmt(&self.0, f)
            }
        }

        impl fmt::Octal for $for {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Octal::fmt(&self.0, f)
            }
        }

        impl fmt::UpperHex for $for {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::UpperHex::fmt(&self.0, f)
            }
        }

        impl fmt::Pointer for $for {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Pointer::fmt(&(self.0 as *const ()), f)
            }
        }
    };
}

impl_fmt_traits!(for VirtAddr);
impl_fmt_traits!(for PhysAddr);

impl ops::Add<usize> for VirtAddr {
    type Output = Self;

    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0.checked_add(rhs).unwrap())
    }
}

impl ops::Add for VirtAddr {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.as_usize()
    }
}

impl ops::AddAssign<usize> for VirtAddr {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

impl ops::AddAssign for VirtAddr {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self += rhs.as_usize();
    }
}

impl ops::Sub<usize> for VirtAddr {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0.checked_sub(rhs).unwrap())
    }
}

impl ops::Sub for VirtAddr {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self - rhs.as_usize()
    }
}

impl ops::SubAssign<usize> for VirtAddr {
    #[inline]
    fn sub_assign(&mut self, rhs: usize) {
        *self = *self - rhs;
    }
}

impl ops::SubAssign for VirtAddr {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self -= rhs.as_usize();
    }
}

impl ops::Add<usize> for PhysAddr {
    type Output = Self;

    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0.checked_add(rhs).unwrap())
    }
}

impl ops::Add for PhysAddr {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.as_usize()
    }
}

impl ops::AddAssign<usize> for PhysAddr {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

impl ops::AddAssign for PhysAddr {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self += rhs.as_usize();
    }
}

impl ops::Sub<usize> for PhysAddr {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0.checked_sub(rhs).unwrap())
    }
}

impl ops::Sub for PhysAddr {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self - rhs.as_usize()
    }
}

impl ops::SubAssign<usize> for PhysAddr {
    #[inline]
    fn sub_assign(&mut self, rhs: usize) {
        *self = *self - rhs;
    }
}

impl ops::SubAssign for PhysAddr {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self -= rhs.as_usize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;

    #[test]
    fn new_virt_addr() {
        assert!(VirtAddr::try_new(0x0).is_ok());
        assert!(VirtAddr::try_new(0xABCD).is_ok());
        assert!(VirtAddr::try_new(0xFFFF_FFFF_FFFF).is_ok());
        assert!(VirtAddr::try_new(0x1_FFFF_FFFF_FFFF).is_err());
        assert!(VirtAddr::try_new(0xFFFF_0000_0000_0000).is_ok());
        assert!(VirtAddr::try_new(0xFFFF_0000_ABCD_0000).is_ok());
    }

    #[test]
    fn keep_upper_bits_when_adding_virt_addresses() {
        let a = VirtAddr::new(0xFFFF_F000_ABCD_0000);
        let b = VirtAddr::new(0x0000_F000_0000_ABCD);
        let c = a + b;
        assert_eq!(c.as_usize(), 0xFFFF_E000_ABCD_ABCD);

        let a = VirtAddr::new(0x0000_F000_ABCD_0000);
        let b = VirtAddr::new(0x0000_F000_0000_ABCD);
        let c = a + b;
        assert_eq!(c.as_usize(), 0x0000_E000_ABCD_ABCD);
    }

    macro_rules! assert_fmt {
        ($fmt:literal, $addr:ident, $expected:literal) => {
            assert_eq!(std::format!($fmt, $addr).as_str(), $expected);
        };
    }

    #[test]
    fn phys_and_virt_format() {
        let v = VirtAddr::new(0xABCD);
        let p = PhysAddr::new(0xEF00);

        assert_fmt!("{:?}", v, "VirtAddr(0xABCD)");
        assert_fmt!("{:x}", v, "abcd");
        assert_fmt!("{:p}", v, "0xabcd");

        assert_fmt!("{:?}", p, "PhysAddr(0xEF00)");
        assert_fmt!("{:x}", p, "ef00");
        assert_fmt!("{:p}", p, "0xef00");
    }
}
