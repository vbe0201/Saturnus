//! Implementation of the AArch64 Page Table Descriptor.

use bitflags::bitflags;
use tock_registers::{fields::Field, register_bitfields};

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

        /// Physical address of the next table descriptor (L2) or the
        /// page descriptor (L3).
        OUTPUT_ADDR_4KIB_48 OFFSET(12) NUMBITS(36) [],

        /// The not global bit.
        NG OFFSET(11) NUMBITS(1) [],

        /// The Access flag.
        AF OFFSET(10) NUMBITS(1) [],

        /// Shareability field.
        SH OFFSET(8) NUMBITS(2) [
            NonShareable = 0b00,
            OuterShareable = 0b10,
            InnerShareable = 0b11
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
    NonShareable,
    OuterShareable,
    InnerShareable,
}

/// Access Permissions for the memory region denoted by a [`PageTableEntry`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AccessPermission {
    ReadWriteEl1,
    ReadWrite,
    ReadOnlyEl1,
    ReadOnly,
}

impl AccessPermission {
    /// Indicates whether this permission value entails that the memory
    /// region is accessible from EL0.
    #[inline]
    pub const fn user_accessible(&self) -> bool {
        match self {
            Self::ReadWrite | Self::ReadOnly => true,
            _ => false,
        }
    }

    /// Indicates whether this permission value enforces read-only access
    /// to the memory region.
    #[inline]
    pub const fn read_only(&self) -> bool {
        match self {
            Self::ReadOnlyEl1 | Self::ReadOnly => true,
            _ => false,
        }
    }
}

/// Representation of a generic page table entry.
#[repr(transparent)]
pub struct PageTableEntry {
    entry: u64,
}

impl PageTableEntry {
    /// An invalid [`PageTableEntry`] tag.
    pub const INVALID: Self = Self::new();

    /// Creates a new invalid page table entry.
    #[inline(always)]
    pub const fn new() -> Self {
        Self { entry: 0 }
    }

    #[inline(always)]
    const fn read(&self, field: Field<u64, STAGE1_TABLE_DESCRIPTOR::Register>) -> u64 {
        (self.entry & (field.mask << field.shift)) >> field.shift
    }

    #[inline(always)]
    const fn is_set(&self, field: Field<u64, STAGE1_TABLE_DESCRIPTOR::Register>) -> bool {
        self.entry & (field.mask << field.shift) != 0
    }

    /// Reads the bitmask of [`SoftwareReserved`] bits out of this entry.
    #[inline]
    pub const fn software_reserved(&self) -> SoftwareReserved {
        let bits = self.read(STAGE1_TABLE_DESCRIPTOR::SOFTWARE_RESERVED);
        unsafe { SoftwareReserved::from_bits_unchecked(bits as u8) }
    }

    /// Whether this entry has the unprivileged execute-never bit set.
    #[inline]
    pub const fn user_execute_never(&self) -> bool {
        self.is_set(STAGE1_TABLE_DESCRIPTOR::UXN)
    }

    /// Whether this entry has the privileged execute-never bit set.
    #[inline]
    pub const fn privileged_execute_never(&self) -> bool {
        self.is_set(STAGE1_TABLE_DESCRIPTOR::PXN)
    }

    /// Whether this entry is one of a contiguous set of entries.
    #[inline]
    pub const fn contiguous(&self) -> bool {
        self.is_set(STAGE1_TABLE_DESCRIPTOR::CONTIGUOUS)
    }

    /// Whether this entry is marked global.
    #[inline]
    pub const fn global(&self) -> bool {
        !self.is_set(STAGE1_TABLE_DESCRIPTOR::NG)
    }

    /// Whether this entry has been accessed for the first time already.
    #[inline]
    pub const fn accessed(&self) -> bool {
        self.is_set(STAGE1_TABLE_DESCRIPTOR::AF)
    }

    /// Gets the [`Shareability`] of this entry.
    #[inline]
    pub const fn shareability(&self) -> Shareability {
        use STAGE1_TABLE_DESCRIPTOR::SH;

        let value = self.read(SH);
        match value {
            _ if value == SH::NonShareable.value => Shareability::NonShareable,
            _ if value == SH::OuterShareable.value => Shareability::OuterShareable,
            _ if value == SH::InnerShareable.value => Shareability::InnerShareable,
            _ => unreachable!(),
        }
    }

    /// Gets the [`AccessPermission`] for the memory region of this entry.
    #[inline]
    pub const fn access_permission(&self) -> AccessPermission {
        use STAGE1_TABLE_DESCRIPTOR::AP;

        let value = self.read(AP);
        match value {
            _ if value == AP::RW_EL1.value => AccessPermission::ReadWriteEl1,
            _ if value == AP::RW_EL1_EL0.value => AccessPermission::ReadWrite,
            _ if value == AP::RO_EL1.value => AccessPermission::ReadOnlyEl1,
            _ if value == AP::RO_EL1_EL0.value => AccessPermission::ReadOnly,
            _ => unreachable!(),
        }
    }

    /// Whether this entry is marked non-secure.
    #[inline]
    pub const fn non_secure(&self) -> bool {
        self.is_set(STAGE1_TABLE_DESCRIPTOR::NS)
    }

    // TODO: Attributes.

    /// Whether this entry represents a block.
    #[inline]
    pub const fn is_block(&self) -> bool {
        let software_bits = self.software_reserved();
        software_bits.contains(SoftwareReserved::VALID)
            && !self.is_set(STAGE1_TABLE_DESCRIPTOR::TYPE)
    }

    /// Whether this entry represents a table.
    #[inline]
    pub const fn is_table(&self) -> bool {
        let software_bits = self.software_reserved();
        !software_bits.contains(SoftwareReserved::VALID)
            && self.is_set(STAGE1_TABLE_DESCRIPTOR::TYPE)
    }

    /// Whether this entry is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        let software_bits = self.software_reserved();
        !software_bits.contains(SoftwareReserved::VALID)
            && !self.is_set(STAGE1_TABLE_DESCRIPTOR::TYPE)
    }

    /// Whether this entry is valid and mapped.
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.is_set(STAGE1_TABLE_DESCRIPTOR::VALID)
    }
}

assert_eq_size!(PageTableEntry, u64);
