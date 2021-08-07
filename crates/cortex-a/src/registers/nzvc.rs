use tock_registers::register_bitfields;

register_bitfields! {u64,
    /// Condition flags.
    ///
    /// Allows access to the condition flags.
    pub NZVC [
        /// Negative condition flag.
        ///
        /// Set to 1 if the result of the last flag-setting instruction was negative.
        N OFFSET(31) NUMBITS(1) [],

        /// Zero condition flag.
        ///
        /// Set to 1 if the result of the last flag-setting instruction was zero, and to 0
        /// otherwise. A result of zero often indicates an equal result from a comparison
        Z OFFSET(30) NUMBITS(1) [],

        /// Carry condition flag.
        ///
        /// Set to 1 if the last flag-setting instruction resulted in a carry condition, for
        /// example an unsigned overflow on an addition.
        C OFFSET(29) NUMBITS(1) [],

        /// Overflow condition flag.
        ///
        /// Set to 1 if the last flag-setting instruction resulted in an overflow condition, for
        /// example a signed overflow on an addition
        V OFFSET(28) NUMBITS(1) [],
    ]
}

impl_read_write_msr!(
    /// Condition flags.
    ///
    /// Allows access to the condition flags.
    NZVC,
    u64,
    "x",
    "NZVC"
);
