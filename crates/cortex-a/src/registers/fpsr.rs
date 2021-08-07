use tock_registers::register_bitfields;

register_bitfields! {u64,
    /// Floating-point Status Register
    ///
    /// Provides floating-point system status information.
    pub FPSR [
        /// Cumulative saturation bit, Advanced SIMD only.
        ///
        /// This bit is set to 1 to indicate that an Advanced SIMD integer operation has saturated
        /// since 0 was last written to this bit.
        QC OFFSET(27) NUMBITS(1) [],

        /// Input Denormal cumulative floating-point exception bit.
        ///
        /// This bit is set to 1 to indicate that the Input Denormal floating-point exception has
        /// occurred since 0 was last written to this bit.
        IDC OFFSET(7) NUMBITS(1) [],

        /// Inexact cumulative floating-point exception bit.
        ///
        /// This bit is set to 1 to indicate that the Inexact floating-point exception has occurred
        /// since 0 was last written to this bit.
        IXC OFFSET(4) NUMBITS(1) [],

        /// Underflow cumulative floating-point exception bit.
        ///
        /// This bit is set to 1 to indicate that the Underflow floating-point exception has
        /// occurred since 0 was last written to this bit
        UFC OFFSET(3) NUMBITS(1) [],

        /// Overflow cumulative floating-point exception bit.
        ///
        /// This bit is set to 1 to indicate that the Overflow floating-point exception has
        /// occurred since 0 was last written to this bit.
        OFC OFFSET(2) NUMBITS(1) [],

        /// Divide by Zero cumulative floating-point exception bit.
        ///
        /// This bit is set to 1 to indicate that the Divide by Zero floating-point exception has
        /// occurred since 0 was last written to this bit.
        DZC OFFSET(1) NUMBITS(1) [],

        /// Invalid Operation cumulative floating-point exception bit.
        ///
        /// This bit is set to 1 to indicate that the Invalid Operation floating-point exception
        /// has occurred since 0 was last written to this bit.
        IOC OFFSET(0) NUMBITS(1) [],
    ]
}

impl_read_write_msr!(
    /// Floating-point Status Register
    ///
    /// Provides floating-point system status information.
    FPSR,
    u64,
    "x",
    "FPSR"
);
