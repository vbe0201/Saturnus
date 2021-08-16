pub mod el1 {
    impl_read_write_msr!(
        /// Fault Address Register (EL1).
        ///
        /// Holds the faulting Virtual Address for all synchronous Instruction or Data Abort, PC
        /// alignment fault and Watchpoint exceptions that are taken to EL1.
        FAR_EL1,
        (),
        u64,
        "x",
        "FAR_EL1"
    );
}

pub mod el2 {
    impl_read_write_msr!(
        /// Fault Address Register (EL2).
        ///
        /// Holds the faulting Virtual Address for all synchronous Instruction or Data Abort, PC
        /// alignment fault and Watchpoint exceptions that are taken to EL2.
        FAR_EL2,
        (),
        u64,
        "x",
        "FAR_EL2"
    );
}

pub mod el3 {
    impl_read_write_msr!(
        /// Fault Address Register (EL3).
        ///
        /// Holds the faulting Virtual Address for all synchronous Instruction or Data Abort, PC
        /// alignment fault and Watchpoint exceptions that are taken to EL3.
        FAR_EL3,
        (),
        u64,
        "x",
        "FAR_EL3"
    );
}
