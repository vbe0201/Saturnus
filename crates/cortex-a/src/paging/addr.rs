//! Physical and virtual addresses representation and manipulation.

use core::{fmt, ops};

/// The amount of bits any virtual address or physical address might have.
///
/// For this paging module to work correctly, you need to set `TxSZ` to `64 - ADDRESS_BITS`,
/// because that's the only address space size it supports.
pub const ADDRESS_BITS: usize = 49;
const ADDRESS_BITS_MASK: usize = (1 << (ADDRESS_BITS - 1)) - 1;
const UPPER_BITS_MASK: usize = !ADDRESS_BITS_MASK;

/// Align a value upwards.
///
/// # Panics
///
/// If the alignment is not a power of two.
#[inline(always)]
pub const fn align_up(val: usize, align: usize) -> usize {
    align_down(val + align - 1, align)
}

/// Align a value downwards.
///
/// # Panics
///
/// If the alignment is not a power of two.
#[inline(always)]
pub const fn align_down(val: usize, align: usize) -> usize {
    assert!(align.is_power_of_two(), "'align' must be a power of two");
    val & !(align - 1)
}

/// Tried to create an address that was not valid.
///
/// This means that the upper bits weren't all zeros or ones.
#[derive(Debug)]
pub struct MalformedAddress(usize);

/// A virtual memory address.
///
/// This is a wrapper type around an `usize`, which guarantees that the upper most
/// bits are either all ones or zeros. The amount of upper bits is controlled by the
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
    pub fn new(addr: usize) -> Self {
        Self::try_new(addr).expect("VirtAddr::new: address is malformed")
    }

    /// Tries to create a new virtual address, which is guaranteed to be canonical.
    ///
    /// Returns an error if the address is malformed.
    #[inline]
    pub fn try_new(addr: usize) -> Result<Self, MalformedAddress> {
        match addr & UPPER_BITS_MASK {
            0 | UPPER_BITS_MASK => Ok(Self(addr)),
            _ => Err(MalformedAddress(addr)),
        }
    }

    /// Create a new virtual address without checking if it's malformed.
    ///
    /// # Safety
    ///
    /// The upper bits of the address must be all zeros or ones.
    #[inline]
    pub unsafe fn new_unchecked(addr: usize) -> Self {
        Self(addr)
    }

    /// Converts this address to the inner `usize`.
    #[inline]
    pub const fn as_usize(self) -> usize {
        self.0
    }

    /// Converts the address to a raw pointer.
    #[inline]
    pub fn as_ptr<T>(self) -> *const T {
        self.as_usize() as *const T
    }

    /// Converts the address to a raw pointer.
    #[inline]
    pub fn as_mut_ptr<T>(self) -> *mut T {
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
    pub fn align_up(self, align: usize) -> Self {
        let upper = self.as_usize() & UPPER_BITS_MASK;
        let addr = (align_up(self.as_usize(), align) & ADDRESS_BITS_MASK) | upper;

        // Safety:
        // We forward the upper bits of `self` into the result,
        // and it's guaranteed that the upper bits of `self` are
        // valid.
        unsafe { Self::new_unchecked(addr) }
    }

    /// Align this address downwards to the given alignment.
    ///
    /// Note that this method will leave the upper bits of this address unchanged
    ///
    /// # Panics
    ///
    /// If the alignment is not a power of two.
    #[inline]
    pub fn align_down(self, align: usize) -> Self {
        let upper = self.as_usize() & UPPER_BITS_MASK;
        let addr = (align_down(self.as_usize(), align) & ADDRESS_BITS_MASK) | upper;

        // Safety:
        // We forward the upper bits of `self` into the result,
        // and it's guaranteed that the upper bits of `self` are
        // valid.
        unsafe { Self::new_unchecked(addr) }
    }

    /// Check if this address is aligned to the given alignment.
    ///
    /// # Panics
    ///
    /// If the alignment is not a power of two.
    #[inline]
    pub fn is_aligned(self, align: usize) -> bool {
        assert!(align.is_power_of_two(), "'align' must be a power of two");
        self.0 & (align - 1) == 0
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

    /// Creates a new physical address.
    #[inline]
    pub fn new(addr: usize) -> Self {
        Self(addr)
    }

    /// Converts this address to the inner `usize`.
    #[inline]
    pub const fn as_usize(self) -> usize {
        self.0
    }

    /// Converts the address to a raw pointer.
    #[inline]
    pub fn as_ptr<T>(self) -> *const T {
        self.as_usize() as *const T
    }

    /// Converts the address to a raw pointer.
    #[inline]
    pub fn as_mut_ptr<T>(self) -> *mut T {
        self.as_usize() as *mut T
    }

    /// Align this address upwards to the given alignment.
    ///
    /// # Panics
    ///
    /// If the alignment is not a power of two.
    #[inline]
    pub fn align_up(self, align: usize) -> Self {
        Self(align_up(self.as_usize(), align))
    }

    /// Align this address downwards to the given alignment.
    ///
    /// # Panics
    ///
    /// If the alignment is not a power of two.
    #[inline]
    pub fn align_down(self, align: usize) -> Self {
        Self(align_down(self.as_usize(), align))
    }

    /// Check if this address is aligned to the given alignment.
    ///
    /// # Panics
    ///
    /// If the alignment is not a power of two.
    #[inline]
    pub fn is_aligned(self, align: usize) -> bool {
        assert!(align.is_power_of_two(), "'align' must be a power of two");
        self.0 & (align - 1) == 0
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

    fn add(self, rhs: usize) -> Self::Output {
        let upper = self.as_usize() & UPPER_BITS_MASK;
        let result = self.as_usize().wrapping_add(rhs);

        // Safety:
        // We forward the upper bits of `self` into the result,
        // and it's guaranteed that the upper bits of `self` are
        // valid.
        unsafe { Self::new_unchecked((result & ADDRESS_BITS_MASK) | upper) }
    }
}

impl ops::Add for VirtAddr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.as_usize()
    }
}

impl ops::Sub<usize> for VirtAddr {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        let upper = self.as_usize() & UPPER_BITS_MASK;
        let result = self.as_usize().wrapping_sub(rhs);

        // Safety:
        // We forward the upper bits of `self` into the result,
        // and it's guaranteed that the upper bits of `self` are
        // valid.
        unsafe { Self::new_unchecked((result & ADDRESS_BITS_MASK) | upper) }
    }
}

impl ops::Sub for VirtAddr {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self - rhs.as_usize()
    }
}

impl ops::Add<usize> for PhysAddr {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0.wrapping_add(rhs))
    }
}

impl ops::Add for PhysAddr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.as_usize()
    }
}

impl ops::Sub<usize> for PhysAddr {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0.wrapping_sub(rhs))
    }
}

impl ops::Sub for PhysAddr {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self - rhs.as_usize()
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
