pub mod el1 {
    impl_read_write_msr!(
        /// Vector Base Address Register (EL1)
        ///
        /// Holds the vector base address for any exception that is taken to EL1.
        VBAR_EL1,
        (),
        u64,
        "x",
        "VBAR_EL1"
    );
}

pub mod el2 {
    impl_read_write_msr!(
        /// Vector Base Address Register (EL2)
        ///
        /// Holds the vector base address for any exception that is taken to EL2.
        VBAR_EL2,
        (),
        u64,
        "x",
        "VBAR_EL2"
    );
}

pub mod el3 {
    impl_read_write_msr!(
        /// Vector Base Address Register (EL3)
        ///
        /// Holds the vector base address for any exception that is taken to EL3.
        VBAR_EL3,
        (),
        u64,
        "x",
        "VBAR_EL3"
    );
}
