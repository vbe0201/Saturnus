use tock_registers::register_bitfields;

register_bitfields! {u64,
    /// Interrupt Mask Bits
    pub DAIF [
        /// Process state D mask. The possible values of this bit are:
        ///
        /// - 0: Watchpoint, Breakpoint, and Software Step exceptions targeted at the current Exception
        ///   level are not masked.
        ///
        /// - 1: Watchpoint, Breakpoint, and Software Step exceptions targeted at the current Exception
        ///   level are masked.
        ///
        /// When the target Exception level of the debug exception is higher than the current
        /// Exception level, the exception is not masked by this bit.
        ///
        /// When this register has an architecturally-defined reset value, this field resets to 1.
        D OFFSET(9) NUMBITS(1) [
            Unmasked = 0,
            Masked = 1
        ],

        /// SError interrupt mask bit. The possible values of this bit are:
        ///
        /// - 0 Exception not masked.
        /// - 1 Exception masked.
        ///
        /// When this register has an architecturally-defined reset value, this field resets to 1.
        A OFFSET(8) NUMBITS(1) [
            Unmasked = 0,
            Masked = 1
        ],

        /// IRQ mask bit. The possible values of this bit are:
        ///
        /// - 0 Exception not masked.
        /// - 1 Exception masked.
        ///
        /// When this register has an architecturally-defined reset value, this field resets to 1.
        I OFFSET(7) NUMBITS(1) [
            Unmasked = 0,
            Masked = 1
        ],

        /// FIQ mask bit. The possible values of this bit are:
        ///
        /// - 0 Exception not masked.
        /// - 1 Exception masked.
        ///
        /// When this register has an architecturally-defined reset value, this field resets to 1.
        F OFFSET(6) NUMBITS(1) [
            Unmasked = 0,
            Masked = 1
        ]
    ]
}

impl_read_write_msr!(
    /// Interrupt Mask Bits
    DAIF,
    u64,
    "x",
    "DAIF"
);
