//! Holds the Current Exception Level.

use tock_registers::{interfaces::Readable, register_bitfields};

register_bitfields! {u64,
    pub CurrentEL [
        /// Current Exception level. Possible values of this field are:
        ///
        /// - 0b00: EL0
        /// - 0b01: EL1
        /// - 0b10: EL2
        /// - 0b11: EL3
        ///
        /// When the HCR_EL2.NV bit is 1, Non-secure EL1 read accesses to the CurrentEL register
        /// return the value of 0x2 in this field.
        ///
        /// This field resets to a value that is architecturally UNKNOWN.
        EL OFFSET(2) NUMBITS(2) [
            EL0 = 0,
            EL1 = 1,
            EL2 = 2,
            EL3 = 3
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CurrentEL::Register;

    #[inline]
    fn get(&self) -> u64 {
        read_msr!(u64, "x", "CurrentEL")
    }
}

#[doc(hidden)]
#[allow(non_upper_case_globals)]
pub const CurrentEL: Reg = Reg {};
