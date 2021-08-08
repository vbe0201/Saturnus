//! Abstractions for accessing the `MAIR_ELx` registers.
//!
//! The `mair` module is special because for the `MAIR_ELx` registers there are
//! more abstractions and not just a single `tock-registers` interface.

use core::cell::Cell;
use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields! {u8,
    /// Bitfield for specifiying a single memory attribute field inside
    /// a `MAIR_ELx` register.
    ///
    /// There are two main (plus some others, which are not supported) memory attributes:
    /// - Device Memory using the `Device` bitfield
    /// - Normal memory using the `NormalOuter` and `NormalInner` bitfields
    pub MAIR_ATTRIBUTE [
        /// This memory is device memory.
        ///
        /// Possible values:
        ///
        ///   dd  | Meaning
        /// ------|----------------------
        ///  0b00 | Device-nGnRnE memory
        ///  0b01 | Device-nGnRE memory
        ///  0b10 | Device-nGRE memory
        ///  0b11 | Device-GRE memory
        ///
        Device OFFSET(2) NUMBITS(2) [
            /// Device-nGnRnE memory
            NonGatheringNonReorderingNoEarlyWriteAck = 0b00,
            /// Device-nGnRE memory
            NonGatheringNonReorderingEarlyWriteAck =   0b01,
            /// Device-nGRE memory
            NonGatheringReorderingEarlyWriteAck =      0b10,
            /// Device-GRE memory
            GatheringReorderingEarlyWriteAck =         0b11,
        ],

        /// The upper 4 bits of the memory attribute, configuring the outer memory attributes.
        NormalOuter OFFSET(4) NUMBITS(4) [
            /// Normal memory, Outer Write-Through Transient with outer write policy.
            WriteThroughTransientWrite =     0b0001,
            /// Normal memory, Outer Write-Through Transient with outer read policy.
            WriteThroughTransientRead =      0b0010,
            /// Normal memory, Outer Write-Through Transient with outer read and write policy.
            WriteThroughTransientReadWrite = 0b0011,

            /// Normal memory, Outer Non-cacheable
            NonCacheable = 0b0100,

            /// Normal memory, Outer Write-Back Transient with outer write policy.
            WriteBackTransientWrite =     0b0101,
            /// Normal memory, Outer Write-Back Transient with outer read policy.
            WriteBackTransientRead =      0b0110,
            /// Normal memory, Outer Write-Back Transient with outer read and write policy.
            WriteBackTransientReadWrite = 0b0111,

            /// Normal memory, Outer Write-Back Transient.
            WriteThroughNonTransient =          0b1000,
            /// Normal memory, Outer Write-Back Transient with outer write policy.
            WriteThroughNonTransientWrite =     0b1001,
            /// Normal memory, Outer Write-Back Transient with outer read policy.
            WriteThroughNonTransientRead =      0b1010,
            /// Normal memory, Outer Write-Back Transient with outer read and write policy.
            WriteThroughNonTransientReadWrite = 0b1011,

            /// Normal memory, Outer Write-Back Non-transient.
            WriteBackNonTransient =          0b1100,
            /// Normal memory, Outer Write-Back Non-transient with outer write policy.
            WriteBackNonTransientWrite =     0b1101,
            /// Normal memory, Outer Write-Back Non-transient with outer read policy.
            WriteBackNonTransientRead =      0b1110,
            /// Normal memory, Outer Write-Back Non-transient with outer read and write policy.
            WriteBackNonTransientReadWrite = 0b1111
        ],

        /// The lower 4 bits of the memory attribute, configuring the inner memory attributes.
        NormalInner OFFSET(0) NUMBITS(4) [
            /// Normal memory, Inner Write-Through Transient with inner write policy.
            WriteThroughTransientWrite =     0b0001,
            /// Normal memory, Inner Write-Through Transient with inner read policy.
            WriteThroughTransientRead =      0b0010,
            /// Normal memory, Inner Write-Through Transient with inner read and write policy.
            WriteThroughTransientReadWrite = 0b0011,

            /// Normal memory, Inner Non-cacheable
            NonCacheable = 0b0100,

            /// Normal memory, Inner Write-Back Transient with inner write policy.
            WriteBackTransientWrite =     0b0101,
            /// Normal memory, Inner Write-Back Transient with inner read policy.
            WriteBackTransientRead =      0b0110,
            /// Normal memory, Inner Write-Back Transient with inner read and write policy.
            WriteBackTransientReadWrite = 0b0111,

            /// Normal memory, Inner Write-Back Transient.
            WriteThroughNonTransient =          0b1000,
            /// Normal memory, Inner Write-Back Transient with inner write policy.
            WriteThroughNonTransientWrite =     0b1001,
            /// Normal memory, Inner Write-Back Transient with inner read policy.
            WriteThroughNonTransientRead =      0b1010,
            /// Normal memory, Inner Write-Back Transient with inner read and write policy.
            WriteThroughNonTransientReadWrite = 0b1011,

            /// Normal memory, Inner Write-Back Non-transient.
            WriteBackNonTransient =          0b1100,
            /// Normal memory, Inner Write-Back Non-transient with inner write policy.
            WriteBackNonTransientWrite =     0b1101,
            /// Normal memory, Inner Write-Back Non-transient with inner read policy.
            WriteBackNonTransientRead =      0b1110,
            /// Normal memory, Inner Write-Back Non-transient with inner read and write policy.
            WriteBackNonTransientReadWrite = 0b1111
        ],
    ]
}

/// A single attribute from a `MAIR_ELx` register that implements `Readable` and `Writable` from
/// the `tock-register` crate.
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct SingleAttribute(Cell<u8>);

impl Readable for SingleAttribute {
    type T = u8;
    type R = MAIR_ATTRIBUTE::Register;

    #[inline]
    fn get(&self) -> u8 {
        self.0.get()
    }
}

impl Writeable for SingleAttribute {
    type T = u8;
    type R = MAIR_ATTRIBUTE::Register;

    #[inline]
    fn set(&self, x: u8) {
        self.0.set(x)
    }
}

/// A collection of all 8 memory attributes that can be written into a `MAIR_ELx` register.
///
/// This type is used to provide a good way of configuring all of the 8 memory attributes inside
/// a `MAIR_ELx` register. To write the value of a `MemoryAttributes` structure into a register
/// use the [`bits`](Self::bits) method.
pub struct MemoryAttributes {
    attrs: [SingleAttribute; 8],
}

impl MemoryAttributes {
    /// Create a new [`MemoryAttributes`] structure where each attribute is `0`.
    pub const fn new() -> Self {
        const EMPTY: SingleAttribute = SingleAttribute(Cell::new(0));
        Self { attrs: [EMPTY; 8] }
    }

    /// Get a reference to the attribute with the given `IDX`.
    ///
    /// The returned reference can be used like a normal `tock-registers` register
    /// to modify the attribute.
    pub const fn attr<const IDX: usize>(&self) -> &SingleAttribute {
        &self.attrs[IDX]
    }

    /// Return the raw value that can be written into a `MAIR_ELx` register.
    #[inline]
    pub fn bits(self) -> u64 {
        let attr0 = self.attrs[0].0.get() as u64;
        let attr1 = self.attrs[1].0.get() as u64;
        let attr2 = self.attrs[2].0.get() as u64;
        let attr3 = self.attrs[3].0.get() as u64;
        let attr4 = self.attrs[4].0.get() as u64;
        let attr5 = self.attrs[5].0.get() as u64;
        let attr6 = self.attrs[6].0.get() as u64;
        let attr7 = self.attrs[7].0.get() as u64;

        attr0
            | (attr1 << 8)
            | (attr2 << 16)
            | (attr3 << 24)
            | (attr4 << 32)
            | (attr5 << 40)
            | (attr6 << 48)
            | (attr7 << 56)
    }
}

macro_rules! mair_el_reg {
    ($(#[$doc:meta])* $name:ident, $reg:literal) => {
        pub struct Reg;

        impl super::Readable for Reg {
            type T = u64;
            type R = ();

            #[inline]
            fn get(&self) -> u64 {
                read_msr!(u64, "x", $reg)
            }
        }

        impl super::Writeable for Reg {
            type T = u64;
            type R = ();

            #[inline]
            fn set(&self, x: u64) {
                write_msr!("x", $reg, x);
            }
        }

        $(#[$doc])*
        pub const $name: Reg = Reg {};
    };
}

mod el1 {
    mair_el_reg!(
        /// Memory Attribute Indirection Register (EL1).
        ///
        /// Provides the memory attribute encodings corresponding to the possible AttrIndx values
        /// in a Long-descriptor format translation table entry for stage 1 translations at EL1.
        MAIR_EL1,
        "MAIR_EL1"
    );
}

mod el2 {
    mair_el_reg!(
        /// Memory Attribute Indirection Register (EL2).
        ///
        /// Provides the memory attribute encodings corresponding to the possible AttrIndx values
        /// in a Long-descriptor format translation table entry for stage 1 translations at EL2.
        MAIR_EL2,
        "MAIR_EL2"
    );
}

mod el3 {
    mair_el_reg!(
        /// Memory Attribute Indirection Register (EL3).
        ///
        /// Provides the memory attribute encodings corresponding to the possible AttrIndx values
        /// in a Long-descriptor format translation table entry for stage 1 translations at EL3.
        MAIR_EL3,
        "MAIR_EL3"
    );
}

pub use el1::MAIR_EL1;
pub use el2::MAIR_EL2;
pub use el3::MAIR_EL3;
