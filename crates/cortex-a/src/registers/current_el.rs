use tock_registers::register_bitfields;

register_bitfields! {u64,
    /// Holds the Current Exception Level.
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

impl_read_msr!(
    /// Holds the Current Exception Level.
    #[allow(non_upper_case_globals)]
    CurrentEL,
    u64,
    "x",
    "CurrentEL"
);
