pub mod el1 {
    impl_read_write_msr!(
        /// Exception Link Register - EL1
        ///
        /// When taking an exception to EL1, holds the address to return to.
        ELR_EL1,
        (),
        u64,
        "x",
        "ELR_EL1"
    );
}

pub mod el2 {
    impl_read_write_msr!(
        /// Exception Link Register - EL2
        ///
        /// When taking an exception to EL2, holds the address to return to.
        ELR_EL2,
        (),
        u64,
        "x",
        "ELR_EL2"
    );
}

pub mod el3 {
    impl_read_write_msr!(
        /// Exception Link Register - EL2
        ///
        /// When taking an exception to EL2, holds the address to return to.
        ELR_EL3,
        (),
        u64,
        "x",
        "ELR_EL3"
    );
}
