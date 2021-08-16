use tock_registers::register_bitfields;

register_bitfields! {u64,
    /// System Control Register
    pub SCTLR [
        /// Enhanced Privileged Access Never. When PSTATE.PAN is 1, determines whether an EL1 data
        /// access to a page with stage 1 EL0 instruction access permission generates a Permission
        /// fault as a result of the Privileged Access Never mechanism
        EPAN OFFSET(57) NUMBITS(1) [
            DontFault = 0,
            Fault = 1,
        ],

        /// When HCR_EL2.{E2H, TGE} != {1, 1}, traps execution of an LD64B or ST64B instruction at
        /// EL0 to EL1.
        EnALS OFFSET(56) NUMBITS(1) [
            Trap = 0,
            DontTrap = 1,
        ],

        /// When HCR_EL2.{E2H, TGE} != {1, 1}, traps execution of an ST64BV0 instruction at EL0 to
        /// EL1.
        EnAS0 OFFSET(55) NUMBITS(1) [
            Trap = 0,
            DontTrap = 1,
        ],

        /// When HCR_EL2.{E2H, TGE} != {1, 1}, traps execution of an ST64BV instruction at EL0 to
        /// EL1.
        EnASR OFFSET(54) NUMBITS(1) [
            Trap = 0,
            DontTrap = 1,
        ],

        /// TWE Delay. A 4-bit unsigned number that, when SCTLR_EL1.TWEDEn is 1, encodes the
        /// minimum delay in taking a trap of WFE* caused by SCTLR_EL1.nTWE as 2(TWEDEL + 8)
        /// cycles.
        TWEDEL OFFSET(46) NUMBITS(3) [],

        /// TWE Delay Enable. Enables a configurable delayed trap of the WFE* instruction caused by
        /// SCTLR_EL1.nTWE.
        TWEDEn OFFSET(45) NUMBITS(1) [
            ImplementationDefined = 0,
            CustomDelay = 1,
        ],

        /// Default PSTATE.SSBS value on Exception Entry.
        DSSBS OFFSET(44) NUMBITS(1) [
            SetToZero = 0,
            SetToOne = 1,
        ],

        /// Allocation Tag Access in EL1. When SCR_EL3.ATA=1 and HCR_EL2.ATA=1, controls EL1 access
        /// to Allocation Tags.
        ATA OFFSET(43) NUMBITS(1) [
            AccessPrevented = 0b0,
            AccessGranted = 0b1,
        ],

        /// Allocation Tag Access in EL0. When SCR_EL3.ATA=1, HCR_EL2.ATA=1, and HCR_EL2.{E2H, TGE}
        /// != {1, 1}, controls EL0 access to Allocation Tags.
        ATA0 OFFSET(42) NUMBITS(1) [
            AccessPrevented = 0,
            AccessGranted = 1,
        ],

        /// Tag Check Fault in EL1. Controls the effect of Tag Check Faults due to Loads and Stores in EL1.
        ///
        /// - 0b00: Tag Check Faults have no effect on the PE.
        /// - 0b01: Tag Check Faults cause a synchronous exception.
        /// - 0b10: Tag Check Faults are asynchronously accumulated.
        /// - 0b11: When FEAT_MTE3 is implemented:
        ///   Tag Check Faults cause a synchronous exception on reads, and are asynchronously
        ///   accumulated on writes.
        TCF OFFSET(40) NUMBITS(2) [
            NoEffect = 0b00,
            SyncException = 0b01,
            AsyncAccumulated = 0b10,
            SyncExceptionAndAsyncAccumulated = 0b11,
        ],

        /// Tag Check Fault in EL0. When HCR_EL2.{E2H,TGE} != {1,1}, controls the effect of Tag Check
        /// Faults due to Loads and Stores in EL0.
        ///
        /// - 0b00: Tag Check Faults have no effect on the PE.
        /// - 0b01: Tag Check Faults cause a synchronous exception.
        /// - 0b10: Tag Check Faults are asynchronously accumulated.
        /// - 0b11: When FEAT_MTE3 is implemented:
        ///   Tag Check Faults cause a synchronous exception on reads, and are asynchronously
        ///   accumulated on writes.
        TCF0  OFFSET(38) NUMBITS(2) [
            NoEffect = 0b00,
            SyncException = 0b01,
            AsyncAccumulated = 0b10,
            SyncExceptionAndAsyncAccumulated = 0b11,
        ],

        /// When synchronous exceptions are not being generated by Tag Check Faults, this field
        /// controls whether on exception entry into EL1, all Tag Check Faults due to instructions
        /// executed before exception entry, that are reported asynchronously, are synchronized
        /// into TFSRE0_EL1 and TFSR_EL1 registers.
        ITFSB OFFSET(37) NUMBITS(1) [
            NotSynchronized = 0,
            Synchronized = 1,
        ],

        /// PAC Branch Type compatibility at EL1.
        BT1 OFFSET(36) NUMBITS(1) [],

        /// PAC Branch Type compatibility at EL0.
        BT0 OFFSET(35) NUMBITS(1) [],

        /// Controls enabling of pointer authentication (using the APIAKey_EL1 key) of instruction
        /// addresses in the EL1&0 translation regime.
        EnIA OFFSET(31) NUMBITS(1) [
            Disable = 0,
            Enable = 1,
        ],

        /// Controls enabling of pointer authentication (using the APIBKey_EL1 key) of instruction
        /// addresses in the EL1&0 translation regime.
        EnIB OFFSET(30) NUMBITS(1) [
            Disable = 0,
            Enable = 1,
        ],

        /// Load Multiple and Store Multiple Atomicity and Ordering Enable.
        ///
        /// - 0: For all memory accesses at EL0, A32 and T32 Load Multiple and Store Multiple can
        ///   have an interrupt taken during the sequence memory accesses, and the memory accesses
        ///   are not required to be ordered.
        /// - 1: The ordering and interrupt behavior of A32 and T32 Load Multiple and Store Multiple
        ///   at EL0 is as defined for Armv8.0.
        LSMAOE OFFSET(29) NUMBITS(1) [],

        /// No Trap Load Multiple and Store Multiple to Device-nGRE/Device-nGnRE/Device-nGnRnE
        /// memory.
        ///
        /// - 0: All memory accesses by A32 and T32 Load Multiple and Store Multiple at EL0 that
        ///   are marked at stage 1 as Device-nGRE/Device-nGnRE/Device-nGnRnE memory are trapped and
        ///   generate a stage 1 Alignment fault.
        /// - 1: All memory accesses by A32 and T32 Load Multiple and Store Multiple at EL0 that
        ///   are marked at stage 1 as Device-nGRE/Device-nGnRE/Device-nGnRnE memory are not trapped.
        nTLSMD OFFSET(28) NUMBITS(1) [
            Trap = 0,
            DontTrap = 1,
        ],

        /// Controls enabling of pointer authentication (using the APDAKey_EL1 key) of instruction
        /// addresses in the EL1&0 translation regime.
        EnDA OFFSET(27) NUMBITS(1) [
            Disable = 0,
            Enable = 1,
        ],

        /// Traps EL0 execution of cache maintenance instructions to EL1, from AArch64 state only.
        UCI OFFSET(26) NUMBITS(1) [
            Trap = 0,
            DontTrap = 1,
        ],

        /// Endianness of data accesses at EL1, and stage 1 translation table walks in the EL1&0 translation regime.
        EE OFFSET(25) NUMBITS(1) [
            LittleEndian = 0,
            BigEndian = 1,
        ],

        /// Endianness of data accesses at EL0.
        E0E OFFSET(24) NUMBITS(1) [
            LittleEndian = 0,
            BigEndian = 1,
        ],

        /// Write permission implies XN (Execute-never). For the EL1&0 translation regime, this bit can force
        /// all memory regions that are writable to be treated as XN.
        WXN OFFSET(19) NUMBITS(1) [
            Disable = 0,
            Enable = 1,
        ],

        /// Traps EL0 execution of WFE instructions to EL1, from both Execution states.
        NTWE OFFSET(18) NUMBITS(1) [
            Trap = 0,
            DontTrap = 1,
        ],

        /// Traps EL0 executions of WFI instructions to EL1, from both execution states:
        NTWI OFFSET(16) NUMBITS(1) [
            Trap = 0,
            DontTrap = 1,
        ],

        /// Traps EL0 accesses to the CTR_EL0 to EL1, from AArch64 state only.
        UCT OFFSET(15) NUMBITS(1) [
            Trap = 0,
            DontTrap = 1,
        ],

        /// Traps EL0 execution of DC ZVA instructions to EL1, from AArch64 state only.
        DZE OFFSET(14) NUMBITS(1) [
            Trap = 0,
            DontTrap = 1,
        ],

        /// Instruction access Cacheability control, for accesses at EL0 and EL1:
        I OFFSET(12) NUMBITS(1) [
            NonCacheable = 0,
            Cacheable = 1
        ],

        /// User Mask Access. Traps EL0 execution of MSR and MRS instructions that access the
        /// PSTATE.{D, A, I, F} masks to EL1, from AArch64 state only.
        UMA OFFSET(9) NUMBITS(1) [
            Trap = 0,
            DontTrap = 1,
        ],

        /// Non-aligned access. This bit controls generation of Alignment faults at EL1 and EL0 under certain conditions.
        ///
        /// LDAPR, LDAPRH, LDAPUR, LDAPURH, LDAPURSH, LDAPURSW, LDAR, LDARH, LDLAR, LDLARH,
        /// STLLR, STLLRH, STLR, STLRH, STLUR, and STLURH will or will not generate an Alignment
        /// fault if all bytes being accessed are not within a single 16-byte quantity,
        /// aligned to 16 bytes for accesses.
        NAA OFFSET(6) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// SP Alignment check enable for EL0.
        ///
        /// When set to 1, if a load or store instruction executed at EL0 uses the SP
        /// as the base address and the SP is not aligned to a 16-byte boundary,
        /// then a SP alignment fault exception is generated.
        SA0 OFFSET(4) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// SP Alignment check enable.
        ///
        /// When set to 1, if a load or store instruction executed at EL1 uses the SP
        /// as the base address and the SP is not aligned to a 16-byte boundary,
        /// then a SP alignment fault exception is generated.
        SA OFFSET(3) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// Cacheability control, for data accesses.
        C OFFSET(2) NUMBITS(1) [
            NonCacheable = 0,
            Cacheable = 1
        ],

        /// Alignment check enable. This is the enable bit for Alignment fault checking at EL1 and EL0.
        ///
        /// Instructions that load or store one or more registers, other than load/store exclusive
        /// and load-acquire/store-release, will or will not check that the address being accessed
        /// is aligned to the size of the data element(s) being accessed depending on this flag.
        ///
        /// Load/store exclusive and load-acquire/store-release instructions have an alignment check
        /// regardless of the value of the A bit.
        A OFFSET(1) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// MMU enable for EL1 and EL0 stage 1 address translation. Possible values of this bit are:
        ///
        /// - 0: EL1 and EL0 stage 1 address translation disabled.
        ///   - See the SCTLR_EL1.I field for the behavior of instruction accesses to Normal memory.
        /// - 1: EL1 and EL0 stage 1 address translation enabled.
        M OFFSET(0) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ]
    ]
}

pub mod el1 {
    impl_read_write_msr!(
        /// System Control Register (EL1)
        ///
        /// Provides top level control of the system, including its memory system, at EL1 and EL0.
        SCTLR_EL1,
        super::SCTLR::Register,
        u64,
        "x",
        "SCTLR_EL1"
    );
}

pub mod el2 {
    impl_read_write_msr!(
        /// System Control Register (EL2)
        ///
        /// Provides top level control of the system, including its memory system, at EL2.
        SCTLR_EL2,
        super::SCTLR::Register,
        u64,
        "x",
        "SCTLR_EL2"
    );
}

pub mod el3 {
    impl_read_write_msr!(
        /// System Control Register (EL3)
        ///
        /// Provides top level control of the system, including its memory system, at EL3.
        SCTLR_EL3,
        super::SCTLR::Register,
        u64,
        "x",
        "SCTLR_EL3"
    );
}
