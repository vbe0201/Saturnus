use tock_registers::register_bitfields;

register_bitfields! {u64,
    /// Data independent timing.
    ///
    /// Allows access to the Data Independent Timing bit.
    pub DIT [
        /// Data Independent Timing.
        ///
        /// - 0: The architecture makes no statement about the timing properties of any instructions.
        /// - 1: The architecture requires that:
        ///   - The timing of every load and store instruction is insensitive to the
        ///     value of the data being loaded or stored
        ///   - For certain data processing instructions, the instruction takes a time
        ///     which is independent of:
        ///     - The values of the data supplied in any of its registers
        ///     - The values of the NZCV flags.
        ///   - For certain data processing instructions, the response of the
        ///     instruction to asynchronous exceptions does not vary based on:
        ///     - The values of the data supplied in any of its registers.
        ///     - The values of the NZCV flags.
        DIT OFFSET(24) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ]
    ]
}

impl_read_write_msr!(
    /// Data independent timing.
    ///
    /// Allows access to the Data Independent Timing bit.
    DIT,
    u64,
    "x",
    "DIT"
);
