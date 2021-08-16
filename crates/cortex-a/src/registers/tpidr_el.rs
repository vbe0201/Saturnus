pub mod el1 {
    impl_read_write_msr!(
        /// EL1 Software Thread ID Register
        ///
        /// Provides a location where software executing at EL1 can store thread identifying
        /// information, for OS management purposes.
        TPIDR_EL1,
        (),
        u64,
        "x",
        "TPIDR_EL1"
    );
}

pub mod el2 {
    impl_read_write_msr!(
        /// EL2 Software Thread ID Register
        ///
        /// Provides a location where software executing at EL2 can store thread identifying
        /// information, for OS management purposes.
        TPIDR_EL2,
        (),
        u64,
        "x",
        "TPIDR_EL2"
    );
}

pub mod el3 {
    impl_read_write_msr!(
        /// EL3 Software Thread ID Register
        ///
        /// Provides a location where software executing at EL3 can store thread identifying
        /// information, for OS management purposes.
        TPIDR_EL3,
        (),
        u64,
        "x",
        "TPIDR_EL3"
    );
}
