use tock_registers::register_bitfields;

register_bitfields! {u64,
    /// Floating-point Control Register
    ///
    /// Controls floating-point behavior.
    pub FPCR [
        /// Alternative half-precision control bit.
        ///
        /// - 0: IEEE half-precision format selected.
        /// - 1: Alternative half-precision format selected.
        AHP OFFSET(26) NUMBITS(1) [],

        /// Default NaN use for NaN propagation.
        ///
        /// - 0: NaN operands propagate through to the output of a floating-point operation.
        /// - 1: Any operation involving one or more NaNs returns the Default NaN
        DN OFFSET(25) NUMBITS(1) [],

        /// Flushing denormalized numbers to zero control bit.
        ///
        /// - 0:
        ///   - If FPCR.AH is 0, the flushing to zero of
        ///     single-precision and double-precision denormalized
        ///     inputs to, and outputs of, floating-point instructions
        ///     not enabled by this control, but other factors might
        ///     cause the input denormalized numbers to be flushed to zero.
        ///   - If FPCR.AH is 1, the flushing to zero of single-precision
        ///     and double-precision denormalized outputs of floating-point
        ///     instructions not enabled by this control, but other factors
        ///     might cause the input denormalized numbers to be flushed to zero.
        /// - 1:
        ///   - If FPCR.AH is 0, denormalized single-precision and double-precision
        ///     inputs to, and outputs from, floating-point instructions are flushed to zero.
        ///   - If FPCR.AH is 1, denormalized single-precision and double-precision outputs
        ///     from floating-point instructions are flushed to zero.
        FZ OFFSET(24) NUMBITS(1) [],

        /// Rounding Mode control field.
        ROUNDING_MODE OFFSET(22) NUMBITS(2) [
            RoundToNearest = 0b00,
            /// Round towards plus infinity
            RoundToPlusInfinity = 0b01,
            /// Round towards minus infinity
            RoundToMinusInfinity = 0b10,
            /// Round towards zero
            RoundToZero = 0b11,
        ],

        /// Flushing denormalized numbers to zero control bit on half-precision data-processing instructions.
        ///
        /// - 0: For some instructions, this bit disables flushing to
        ///   zero of inputs and outputs that are half-precision denormalized numbers.
        /// - 1: Flushing denormalized numbers to zero enabled.
        ///   For some instructions that do not convert a half-precision
        ///   input to a higher precision output, this bit enables flushing
        ///   to zero of inputs and outputs that are half-precision denormalized numbers.
        FZ16 OFFSET(19) NUMBITS(1) [],

        /// Input Denormal floating-point exception trap enable.
        ///
        /// - 0: Untrapped exception handling selected.
        ///   If the floating-point exception occurs, the FPSR.IDC bit is set to 1.
        /// - 1: Trapped exception handling selected.
        ///   If the floating-point exception occurs, the PE does not update the FPSR.IDC bit.
        IDE OFFSET(15) NUMBITS(1) [],

        /// Inexact floating-point exception trap enable.
        ///
        /// - 0: Untrapped exception handling selected.
        ///   If the floating-point exception occurs, the FPSR.IXC bit is set to 1.
        /// - 1: Trapped exception handling selected.
        ///   If the floating-point exception occurs, the PE does not update the FPSR.IXC bit.
        IXE OFFSET(12) NUMBITS(1) [],

        /// Underflow floating-point exception trap enable.
        ///
        /// - 0: Untrapped exception handling selected.
        ///   If the floating-point exception occurs, the FPSR.UFC bit is set to 1.
        /// - 1: Trapped exception handling selected.
        ///   If the floating-point exception occurs and Flush-to-zero is not enabled, the PE does not update the FPSR.UFC bit.
        UFE OFFSET(11) NUMBITS(1) [],

        /// Overflow floating-point exception trap enable.
        ///
        /// - 0: Untrapped exception handling selected.
        ///   If the floating-point exception occurs, the FPSR.OFC bit is set to 1.
        /// - 1: Trapped exception handling selected.
        ///   If the floating-point exception occurs, the PE does not update the FPSR.OFC bit.
        OFE OFFSET(10) NUMBITS(1) [],

        /// Divide by Zero floating-point exception trap enable.
        ///
        /// - 0: Untrapped exception handling selected.
        ///   If the floating-point exception occurs, the FPSR.DZC bit is set to 1.
        /// - 1: Trapped exception handling selected.
        ///   If the floating-point exception occurs, the PE does not update the FPSR.DZC bit.
        DZE OFFSET(9) NUMBITS(1) [],

        /// Invalid Operation floating-point exception trap enable.
        ///
        /// - 0: Untrapped exception handling selected.
        ///   If the floating-point exception occurs, the FPSR.IOC bit is set to 1.
        /// - 1: Trapped exception handling selected.
        ///   If the floating-point exception occurs, the PE does not update the FPSR.IOC bit.
        IOE OFFSET(8) NUMBITS(1) [],

        /// Controls how the output elements other than the lowest element
        /// of the vector are determined for Advanced SIMD scalar instructions.
        NEP OFFSET(2) NUMBITS(1) [],

        /// Alternate Handling. Controls alternate handling of floating-point numbers.
        ///
        /// The Arm architecture supports two models for handling some of the corner
        /// cases of the floating-point behaviors, such as the nature of flushing
        /// of denormalized numbers, the detection of tininess and other exceptions
        /// and a range of other behaviors.
        /// The value of the FPCR.AH bit selects between these models.
        ///
        /// For more information on the FPCR.AH bit,
        /// see 'Flushing denormalized numbers to zero',
        /// 'Floating-point exceptions and exception traps'
        /// and the pseudocode of the floating-point instructions.
        AH OFFSET(1) NUMBITS(1) [],

        /// Flush Inputs to Zero.
        ///
        /// Controls whether single-precision, double-precision
        /// and BFloat16 input operands that are denormalized
        /// numbers are flushed to zero.
        ///
        /// - 0: The flushing to zero of single-precision and double-precision
        ///   denormalized inputs to floating-point instructions not enabled
        ///   by this control, but other factors might cause the
        ///   input denormalized numbers to be flushed to zero.
        /// - 1: Denormalized single-precision and double-precision
        ///   inputs to most floating-point instructions flushed to zero.
        FIZ OFFSET(0) NUMBITS(1) []
    ]
}

impl_read_write_msr!(
    /// Floating-point Control Register
    ///
    /// Controls floating-point behavior.
    FPCR,
    u64,
    "x",
    "FPCR"
);
