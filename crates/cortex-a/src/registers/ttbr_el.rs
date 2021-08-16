use tock_registers::register_bitfields;

register_bitfields! {u64,
    pub TTBR [
        /// An ASID for the translation table base address. The TCR_EL1.A1 field selects either
        /// TTBR0_EL1.ASID or TTBR1_EL1.ASID.
        ///
        /// If the implementation has only 8 bits of ASID, then the upper 8 bits of this field are
        /// RES 0.
        ASID OFFSET(48) NUMBITS(16) [],

        /// Translation table base address
        BADDR OFFSET(1) NUMBITS(47) [],

        /// Common not Private
        CnP OFFSET(0) NUMBITS(1) []
    ]
}

pub mod el1_0 {
    impl_read_write_msr!(
        /// Translation Table Base Register 0 (EL1)
        TTBR0_EL1,
        super::TTBR::Register,
        u64,
        "x",
        "TTBR0_EL1"
    );
}

pub mod el1_1 {
    impl_read_write_msr!(
        /// Translation Table Base Register 0 (EL1)
        TTBR1_EL1,
        super::TTBR::Register,
        u64,
        "x",
        "TTBR1_EL1"
    );
}

pub mod el2_0 {
    impl_read_write_msr!(
        /// Translation Table Base Register 0 (EL2)
        TTBR0_EL2,
        super::TTBR::Register,
        u64,
        "x",
        "TTBR0_EL2"
    );
}

pub mod el2_1 {
    impl_read_write_msr!(
        /// Translation Table Base Register 1 (EL2)
        TTBR1_EL2,
        super::TTBR::Register,
        u64,
        "x",
        "TTBR1_EL2"
    );
}

pub mod el3 {
    impl_read_write_msr!(
        /// Translation Table Base Register 0 (EL3)
        TTBR0_EL3,
        super::TTBR::Register,
        u64,
        "x",
        "TTBR0_EL3"
    );
}
