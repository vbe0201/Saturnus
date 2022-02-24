//! Implementation of the AArch64 Page Table Descriptor.
//!
//! This module provides Level 1, 2 and 3 descriptors which assume page
//! sizes with 4KiB granularity and 48-bit OAs.

use core::mem;

use bitflags::bitflags;
use libutils::units::{gib, mib};
use tock_registers::{fields::Field, register_bitfields};

use super::{
    addr::PhysAddr,
    page::{PageSize, SupportedPageSize},
};

/// Gets the size of an L1 block in memory.
#[inline(always)]
pub const fn l1_block_size<const PAGE_SIZE: usize>() -> u64
where
    PageSize<PAGE_SIZE>: SupportedPageSize,
{
    gib(1)
}

/// Gets the size of an L2 block in memory.
#[inline(always)]
pub const fn l2_block_size<const PAGE_SIZE: usize>() -> u64
where
    PageSize<PAGE_SIZE>: SupportedPageSize,
{
    mib(2)
}

/// Gets the size of an L3 block in memory.
#[inline(always)]
pub const fn l3_block_size<const PAGE_SIZE: usize>() -> u64
where
    PageSize<PAGE_SIZE>: SupportedPageSize,
{
    PAGE_SIZE as u64
}

/// Gets the maximum number of page table descriptors based on the chosen
/// page size.
#[inline(always)]
pub const fn max_table_descriptors<const PAGE_SIZE: usize>() -> usize
where
    PageSize<PAGE_SIZE>: SupportedPageSize,
{
    // `u64` here is representative of the `PageTableDescriptor` types further below.
    PAGE_SIZE / mem::size_of::<u64>()
}

// Table descriptor per ARMv8-A Architecture Reference Manual Figure D5-14.
register_bitfields! {
    u64,

    STAGE1_TABLE_DESCRIPTOR [
        /// For memory accesses from Secure state, specifies the Security state
        /// for subsequent levels of lookup.
        NSTable OFFSET(63) NUMBITS(1) [
            Secure = 0,
            NonSecure = 1
        ],

        /// Access permissions limit for subsequent levels of lookup.
        APTable OFFSET(61) NUMBITS(2) [
            NoEffect = 0b00,
            NoEl0ReadAccess = 0b01,
            NoWriteAccess = 0b10,
            NoWriteAndEl0ReadAccess = 0b11
        ],

        /// XN limit for subsequent levels of lookup.
        UXNTable OFFSET(60) NUMBITS(1) [],

        /// XN limit for subsequent levels of lookup.
        XNTable OFFSET(60) NUMBITS(1) [],

        /// PXN limit for subsequent levels of lookup.
        PXNTable OFFSET(59) NUMBITS(1) [],

        /// Software-reserved bits.
        SOFTWARE_RESERVED OFFSET(55) NUMBITS(4) [],

        /// Unprivileged execute-never.
        UXN OFFSET(54) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Privileged execute-never.
        PXN OFFSET(53) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// A hint bit indicating that the translation table entry is one
        /// of a contiguous set of entries.
        CONTIGUOUS OFFSET(52) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Dirty Bit Modifier.
        DBM OFFSET(51) NUMBITS(1) [],

        /// Guarded Page.
        GP OFFSET(50) NUMBITS(1) [],

        /// Physical address of the next table descriptor (L1 and L2).
        NEXT_TABLE_ADDR_4KIB_48 OFFSET(12) NUMBITS(36) [],

        /// The L1 page descriptor.
        L1_OUTPUT_ADDR_4KIB_48 OFFSET(30) NUMBITS(18) [],

        /// The L2 page descriptor.
        L2_OUTPUT_ADDR_4KIB_48 OFFSET(21) NUMBITS(27) [],

        /// The L3 page descriptor.
        L3_OUTPUT_ADDR_4KIB_48 OFFSET(12) NUMBITS(36) [],

        /// The not global bit.
        NG OFFSET(11) NUMBITS(1) [],

        /// The Access flag.
        AF OFFSET(10) NUMBITS(1) [],

        /// Shareability field.
        SH OFFSET(8) NUMBITS(2) [
            None = 0b00,
            Outer = 0b10,
            Inner = 0b11
        ],

        /// Access Permissions.
        AP OFFSET(6) NUMBITS(2) [
            RW_EL1 = 0b00,
            RW_EL1_EL0 = 0b01,
            RO_EL1 = 0b10,
            RO_EL1_EL0 = 0b11
        ],

        /// Non-secure bit.
        NS OFFSET(5) NUMBITS(1) [],

        /// Memory attributes index into the MAIR_EL1 register.
        AttrIndex OFFSET(2) NUMBITS(3) [],

        /// Descriptor type.
        TYPE OFFSET(1) NUMBITS(1) [
            Block = 0,
            Page = 1
        ],

        /// Whether the descriptor is valid.
        VALID OFFSET(0) NUMBITS(1) []
    ]
}

bitflags! {
    /// Representation of the software-reserved bits in a [`PageTableEntry`].
    ///
    /// These bits are following the specific interpretation of the values
    /// done by Nintendo.
    #[repr(transparent)]
    pub struct SoftwareReserved: u8 {
        const DISABLE_MERGE_HEAD = 1 << 0;
        const DISABLE_MERGE_HEAD_BODY = 1 << 1;
        const DISABLE_MERGE_HEAD_TAIL = 1 << 2;
        const VALID = 1 << 3;
    }
}

/// Shareability of the memory region denoted by a [`PageTableEntry`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Shareability {
    /// Non-shareable domain.
    NonShareable = 0b00,
    /// Outer shareable domain.
    OuterShareable = 0b10,
    /// Inner shareable domain.
    InnerShareable = 0b11,
}

impl tock_registers::fields::TryFromValue<u64> for Shareability {
    type EnumType = Self;

    fn try_from(v: u64) -> Option<Self::EnumType> {
        use Shareability::*;
        match v {
            0b00 => Some(NonShareable),
            0b10 => Some(OuterShareable),
            0b11 => Some(InnerShareable),
            _ => None,
        }
    }
}

/// Access Permissions for the memory region denoted by a [`PageTableEntry`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AccessPermission {
    /// Read/write access in EL1 only.
    ReadWriteEl1 = 0b00,
    /// Read/write access in EL1 and EL0.
    ReadWrite = 0b01,
    /// Read-only access in EL1 only.
    ReadOnlyEl1 = 0b10,
    /// Read-only access in EL1 and EL0.
    ReadOnly = 0b11,
}

impl AccessPermission {
    /// Indicates whether this permission value entails that the memory
    /// region is accessible from EL0.
    #[inline]
    pub const fn user_accessible(&self) -> bool {
        matches!(self, Self::ReadWrite | Self::ReadOnly)
    }

    /// Indicates whether this permission value enforces read-only access
    /// to the memory region.
    #[inline]
    pub const fn read_only(&self) -> bool {
        matches!(self, Self::ReadOnlyEl1 | Self::ReadOnly)
    }
}

impl tock_registers::fields::TryFromValue<u64> for AccessPermission {
    type EnumType = Self;

    fn try_from(v: u64) -> Option<Self::EnumType> {
        use AccessPermission::*;
        match v {
            0b00 => Some(ReadWriteEl1),
            0b01 => Some(ReadWrite),
            0b10 => Some(ReadOnlyEl1),
            0b11 => Some(ReadOnly),
            _ => None,
        }
    }
}

/// Representation of the generic Page Descriptor API that applies equally
/// to all the different levels.
pub trait PageTableDescriptor {
    /// Reads the bitmask of [`SoftwareReserved`] bits out of this entry.
    fn software_reserved(&self) -> SoftwareReserved;

    fn set_software_reserved(&mut self, value: SoftwareReserved);

    /// Whether this entry is tagged unprivileged execute-never.
    fn user_execute_never(&self) -> bool;

    fn set_user_execute_never(&mut self, value: bool);

    /// Whether this entry is tagged privileged execute-never.
    fn privileged_execute_never(&self) -> bool;

    fn set_privileged_execute_never(&mut self, value: bool);

    /// Whether this entry is one of a contiguous set of entries.
    fn contiguous(&self) -> bool;

    fn set_contiguous(&mut self, value: bool);

    /// Whether this entry is tagged global.
    fn global(&self) -> bool;

    fn set_global(&mut self, value: bool);

    /// Whether this entry has already been accessed for the first itme.
    fn accessed(&self) -> bool;

    fn set_accessed(&mut self, value: bool);

    /// Gets the [`Shareability`] of this entry.
    fn shareability(&self) -> Shareability;

    fn set_shareability(&mut self, value: Shareability);

    /// Gets the [`AccessPermission`] for the memory region of this entry.
    fn access_permission(&self) -> AccessPermission;

    fn set_access_permission(&mut self, value: AccessPermission);

    /// Whether this entry is tagged non-secure.
    fn non_secure(&self) -> bool;

    fn set_non_secure(&mut self, value: bool);

    // TODO: Attributes.

    /// Whether this entry represents a block.
    fn is_block(&self) -> bool;

    /// Whether this entry represents a table.
    fn is_table(&self) -> bool;

    /// Whether this entry is empty.
    fn is_empty(&self) -> bool;

    /// Whether this entry is valid and mapped.
    fn is_valid(&self) -> bool;

    fn set_valid(&mut self, value: bool);
}

macro_rules! impl_page_table_descriptor {
    ($descriptor:ident) => {
        impl $descriptor {
            /// Creates a new invalid page table entry.
            #[inline(always)]
            pub const fn new() -> Self {
                Self(0)
            }
        }

        impl From<u64> for $descriptor {
            fn from(descriptor: u64) -> Self {
                Self(descriptor)
            }
        }

        impl PageTableDescriptor for $descriptor {
            #[inline]
            fn software_reserved(&self) -> SoftwareReserved {
                let bits = STAGE1_TABLE_DESCRIPTOR::SOFTWARE_RESERVED.read(self.0);
                SoftwareReserved::from_bits_truncate(bits as u8)
            }

            #[inline]
            fn set_software_reserved(&mut self, value: SoftwareReserved) {
                self.0 = STAGE1_TABLE_DESCRIPTOR::SOFTWARE_RESERVED
                    .val(value.bits() as _)
                    .modify(self.0);
            }

            #[inline]
            fn user_execute_never(&self) -> bool {
                STAGE1_TABLE_DESCRIPTOR::UXN.is_set(self.0)
            }

            #[inline]
            fn set_user_execute_never(&mut self, value: bool) {
                self.0 = STAGE1_TABLE_DESCRIPTOR::UXN.val(value as _).modify(self.0);
            }

            #[inline]
            fn privileged_execute_never(&self) -> bool {
                STAGE1_TABLE_DESCRIPTOR::PXN.is_set(self.0)
            }

            #[inline]
            fn set_privileged_execute_never(&mut self, value: bool) {
                self.0 = STAGE1_TABLE_DESCRIPTOR::PXN.val(value as _).modify(self.0);
            }

            #[inline]
            fn contiguous(&self) -> bool {
                STAGE1_TABLE_DESCRIPTOR::CONTIGUOUS.is_set(self.0)
            }

            #[inline]
            fn set_contiguous(&mut self, value: bool) {
                self.0 = STAGE1_TABLE_DESCRIPTOR::CONTIGUOUS
                    .val(value as _)
                    .modify(self.0);
            }

            #[inline]
            fn global(&self) -> bool {
                !STAGE1_TABLE_DESCRIPTOR::NG.is_set(self.0)
            }

            #[inline]
            fn set_global(&mut self, value: bool) {
                self.0 = STAGE1_TABLE_DESCRIPTOR::NG
                    .val((!value) as _)
                    .modify(self.0);
            }

            #[inline]
            fn accessed(&self) -> bool {
                STAGE1_TABLE_DESCRIPTOR::AF.is_set(self.0)
            }

            #[inline]
            fn set_accessed(&mut self, value: bool) {
                self.0 = STAGE1_TABLE_DESCRIPTOR::AF.val(value as _).modify(self.0);
            }

            #[inline]
            fn shareability(&self) -> Shareability {
                STAGE1_TABLE_DESCRIPTOR::SH.read_as_enum(self.0).unwrap()
            }

            #[inline]
            fn set_shareability(&mut self, value: Shareability) {
                self.0 = STAGE1_TABLE_DESCRIPTOR::SH.val(value as _).modify(self.0);
            }

            #[inline]
            fn access_permission(&self) -> AccessPermission {
                // SAFETY: `AP` is 2 bits wide and `TryFromValue` covers all cases.
                unsafe {
                    STAGE1_TABLE_DESCRIPTOR::AP
                        .read_as_enum(self.0)
                        .unwrap_unchecked()
                }
            }

            #[inline]
            fn set_access_permission(&mut self, value: AccessPermission) {
                self.0 = STAGE1_TABLE_DESCRIPTOR::AP.val(value as _).modify(self.0);
            }

            #[inline]
            fn non_secure(&self) -> bool {
                STAGE1_TABLE_DESCRIPTOR::NS.is_set(self.0)
            }

            #[inline]
            fn set_non_secure(&mut self, value: bool) {
                self.0 = STAGE1_TABLE_DESCRIPTOR::NS.val(value as _).modify(self.0);
            }

            // TODO: Attributes.

            #[inline]
            fn is_block(&self) -> bool {
                let software_bits = self.software_reserved();
                software_bits.contains(SoftwareReserved::VALID)
                    && !STAGE1_TABLE_DESCRIPTOR::TYPE.is_set(self.0)
            }

            #[inline]
            fn is_table(&self) -> bool {
                let software_bits = self.software_reserved();
                !software_bits.contains(SoftwareReserved::VALID)
                    && STAGE1_TABLE_DESCRIPTOR::TYPE.is_set(self.0)
            }

            #[inline]
            fn is_empty(&self) -> bool {
                let software_bits = self.software_reserved();
                !software_bits.contains(SoftwareReserved::VALID)
                    && !STAGE1_TABLE_DESCRIPTOR::TYPE.is_set(self.0)
            }

            #[inline]
            fn is_valid(&self) -> bool {
                STAGE1_TABLE_DESCRIPTOR::VALID.is_set(self.0)
            }

            #[inline]
            fn set_valid(&mut self, value: bool) {
                self.0 = STAGE1_TABLE_DESCRIPTOR::VALID
                    .val(value as _)
                    .modify(self.0);
            }
        }
    };
}

/// Representation of a Level 1 Page Table Descriptor.
#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct L1PageTableDescriptor(u64);

impl L1PageTableDescriptor {
    /// Gets the physical output address of this entry.
    #[inline]
    pub fn output_addr(&self) -> PhysAddr {
        let addr = STAGE1_TABLE_DESCRIPTOR::L1_OUTPUT_ADDR_4KIB_48.read(self.0);
        PhysAddr::new(addr as usize)
    }

    /// Gets the physical address of the next L2 table.
    #[inline]
    pub fn next_table(&self) -> PhysAddr {
        let addr = STAGE1_TABLE_DESCRIPTOR::NEXT_TABLE_ADDR_4KIB_48.read(self.0);
        PhysAddr::new(addr as usize)
    }
}

impl_page_table_descriptor!(L1PageTableDescriptor);
assert_eq_size!(L1PageTableDescriptor, u64);

/// Representation of a Level 2 Page Table Descriptor.
#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct L2PageTableDescriptor(u64);

impl L2PageTableDescriptor {
    /// Gets the physical output address of this entry.
    #[inline]
    pub fn output_addr(&self) -> PhysAddr {
        let addr = STAGE1_TABLE_DESCRIPTOR::L2_OUTPUT_ADDR_4KIB_48.read(self.0);
        PhysAddr::new(addr as usize)
    }

    /// Gets the physical address of the next L3 table.
    #[inline]
    pub fn next_table(&self) -> PhysAddr {
        let addr = STAGE1_TABLE_DESCRIPTOR::NEXT_TABLE_ADDR_4KIB_48.read(self.0);
        PhysAddr::new(addr as usize)
    }
}

impl_page_table_descriptor!(L2PageTableDescriptor);
assert_eq_size!(L2PageTableDescriptor, u64);

/// Representation of a Level 3 Page Table Descriptor.
#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct L3PageTableDescriptor(u64);

impl L3PageTableDescriptor {
    /// Gets the physical output address of this entry.
    #[inline]
    pub fn output_addr(&self) -> PhysAddr {
        let addr = STAGE1_TABLE_DESCRIPTOR::L3_OUTPUT_ADDR_4KIB_48.read(self.0);
        PhysAddr::new(addr as usize)
    }
}

impl_page_table_descriptor!(L3PageTableDescriptor);
assert_eq_size!(L3PageTableDescriptor, u64);
