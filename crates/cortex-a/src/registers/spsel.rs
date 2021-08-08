use tock_registers::register_bitfields;

register_bitfields! {u64,
    /// Stack pointer select.
    ///
    /// Allows the Stack Pointer to be selected between SP_EL0 and SP_ELx.
    pub SPSel [
        /// Stack pointer to use. Possible values of this bit are:
        ///
        /// - 0: Use SP_EL0 at all Exception levels.
        /// - 1: Use SP_ELx for Exception level ELx.
        SP OFFSET(0) NUMBITS(1) [
            UseSpEl0 = 0,
            UseSpElX = 1
        ]
    ]
}

impl_read_write_msr!(
    /// Stack pointer select.
    ///
    /// Allows the Stack Pointer to be selected between SP_EL0 and SP_ELx.
    #[allow(non_upper_case_globals)]
    SPSel,
    u64,
    "x",
    "SPSel"
);
