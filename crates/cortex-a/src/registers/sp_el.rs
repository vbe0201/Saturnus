pub mod el0 {
    impl_read_write_msr!(
        /// Stack pointer (EL0)
        ///
        /// Holds the stack pointer associated with EL0. At higher Exception levels, this is used
        /// as the current stack pointer when the value of SPSel.SP is 0.
        SP_EL0,
        (),
        u64,
        "x",
        "SP_EL0"
    );
}

pub mod el1 {
    impl_read_write_msr!(
        /// Stack pointer (EL1)
        ///
        /// When executing at EL1, the value of SPSel.SP determines the current stack pointer.
        SP_EL1,
        (),
        u64,
        "x",
        "SP_EL1"
    );
}

pub mod el2 {
    impl_read_write_msr!(
        /// Stack pointer (EL2)
        ///
        /// When executing at EL2, the value of SPSel.SP determines the current stack pointer.
        SP_EL2,
        (),
        u64,
        "x",
        "SP_EL2"
    );
}

pub mod el3 {
    impl_read_write_msr!(
        /// Stack pointer (EL3)
        ///
        /// When executing at EL3, the value of SPSel.SP determines the current stack pointer.
        SP_EL3,
        (),
        u64,
        "x",
        "SP_EL3"
    );
}
