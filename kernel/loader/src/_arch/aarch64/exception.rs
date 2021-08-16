// This is an architecture-specific module that is made available through the
// path attribute. See the generic module, [`crate::exception`], for orientation.

use core::{cell::UnsafeCell, fmt};

use cortex_a::{asm, registers::*};
use tock_registers::{
    interfaces::{Readable, Writeable},
    registers::InMemoryRegister,
};

// Load definitions of all the exception vector entries in the table.
global_asm!(include_str!("exception.s"));

/// Initializes exception handling by loading our exception vector table into the
/// the exception vector base register `VBAR_EL1`.
///
/// # Safety
///
/// - Changes the hardware state of the executing core.
/// - The vector table at `__exception_vector_start` must adhere to the alignment and
/// size constraints demanded by the ARMv8-A Architecture Reference Manual.
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn setup_exception_vector() {
    // Provided by `exception.s`.
    extern "Rust" {
        static __exception_vector_table: UnsafeCell<()>;
    }

    // Load in the address of the exception vector table into the base register.
    VBAR_EL1.set(__exception_vector_table.get() as u64);

    // Force the register update to complete before next instruction.
    asm::barrier::isb();
}

// Wrapper structures around AArch64 system registers that allow pretty-printing
// all the information stored in their bitfields conveniently from exception handlers.
struct EsrEl1;

#[repr(transparent)]
struct SpsrEl1(InMemoryRegister<u64, SPSR_EL1::Register>);

/// An exception context that is passed into every exception handler.
///
/// Stored in stack memory when entering an exception vector.
#[repr(C)]
struct ExceptionContext {
    // All general-purpose registers from `x0` (inclusive) to `x30` (exclusive).
    gpr: [u64; 30],
    // The link register `x30`.
    lr: u64,
    // Exception link register - the program counter at the time the exception occurred.
    elr_el1: u64,
    // Saved program status.
    spsr_el1: SpsrEl1,
}

assert_eq_size!(ExceptionContext, [u64; 33]);

#[no_mangle]
unsafe extern "C" fn default_exception_handler(e: &ExceptionContext) {
    panic!(
        "\n\nAn exception occurred!\n\
         FAR_EL1: {:#018x}\n\
         {}\n\
         {}",
        unsafe { FAR_EL1.get() },
        EsrEl1,
        e
    );
}

#[no_mangle]
unsafe extern "C" fn unsupported_exception_handler(_: &ExceptionContext) {
    unreachable!("This exception should never occur in EL1!");
}

#[rustfmt::skip]
impl fmt::Display for EsrEl1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ESR_EL1::EC::Value::*;

        // Extract a copy of the register in its current state.
        let esr_el1 = unsafe { ESR_EL1.extract() };

        // Raw print the whole register.
        writeln!(f, "ESR_EL1: {:#010x}", esr_el1.get())?;
        // Raw print and pretty-print the exception class.
        writeln!(
            f,
            "    Exception Class               (EC):  {:#x} ({})",
            esr_el1.read(ESR_EL1::EC),
            match esr_el1.read_as_enum(ESR_EL1::EC) {
                Some(Unknown) => "Unknown",
                Some(TrappedWFIorWFE) => "WFI or WFE trapped",
                Some(TrappedMCRorMRC) => "MCR or MRC trapped",
                Some(TrappedMCRRorMRRC) => "MCRR or MRRC trapped",
                Some(TrappedMCRorMRC2) => "MCR or MCR2 trapped",
                Some(TrappedLDCorSTC) => "LDC or STC trapped",
                Some(TrappedFP) => "FP trapped",
                Some(TrappedMRRC) => "MRRC trapped",
                Some(BranchTarget) => "Branch target",
                Some(IllegalExecutionState) => "Illegal execution state",
                Some(SVC32) => "SVC32",
                Some(SVC64) => "SVC64",
                Some(HVC64) => "HVC64",
                Some(SMC64) => "SMC64",
                Some(TrappedMsrMrs) => "MSR or MRS trapped",
                Some(TrappedSve) => "Sve trapped",
                Some(PointerAuth) => "Pointer auth",
                Some(InstrAbortLowerEL) => "Instruction abort, lower EL",
                Some(InstrAbortCurrentEL) => "Instruction abort, current EL",
                Some(PCAlignmentFault) => "PC alignment fault",
                Some(DataAbortLowerEL) => "Data abort, lower EL",
                Some(DataAbortCurrentEL) => "Data abort, current EL",
                Some(SPAlignmentFault) => "SP alignment fault",
                Some(TrappedFP32) => "FP32 trapped",
                Some(TrappedFP64) => "FP64 trapped",
                Some(SError) => "SError",
                Some(BreakpointLowerEL) => "Breakpoint hit, lower EL",
                Some(BreakpointCurrentEL) => "Breakpoint hit, current EL",
                Some(SoftwareStepLowerEL) => "Software step, lower EL",
                Some(SoftwareStepCurrentEL) => "Software step, current EL",
                Some(WatchpointLowerEL) => "Watchpoint hit, lower EL",
                Some(WatchpointCurrentEL) => "Watchpoint hit, current EL",
                Some(Bkpt32) => "Bkpt32",
                Some(Brk64) => "Brk64",
                None => "N/A",
            }
        )?;
        // Raw print the instruction specific syndrome.
        write!(f, "    Instruction Specific Syndrome (ISS): {:#x}", esr_el1.read(ESR_EL1::ISS))
    }
}

#[inline]
fn stringify_flag(flag_set: bool) -> &'static str {
    if flag_set {
        "Set"
    } else {
        "Cleared"
    }
}

#[inline]
fn stringify_mask(masked: bool) -> &'static str {
    if masked {
        "Masked"
    } else {
        "Unmasked"
    }
}

#[rustfmt::skip]
impl fmt::Display for SpsrEl1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Raw print the whole register followed by pretty-printed flag bits.
        writeln!(f, "SPSR_EL1: {:#010x}", self.0.get())?;
        writeln!(f, "    Flags:")?;
        writeln!(f, "        Negative (N): {}", stringify_flag(self.0.is_set(SPSR_EL1::N)))?;
        writeln!(f, "        Zero     (Z): {}", stringify_flag(self.0.is_set(SPSR_EL1::Z)))?;
        writeln!(f, "        Carry    (C): {}", stringify_flag(self.0.is_set(SPSR_EL1::C)))?;
        writeln!(f, "        Overflow (V): {}", stringify_flag(self.0.is_set(SPSR_EL1::V)))?;

        // Pretty-print the exception state mask bits.
        writeln!(f, "    Exception state:")?;
        writeln!(f, "        Debug  (D): {}", stringify_mask(self.0.is_set(SPSR_EL1::D)))?;
        writeln!(f, "        SError (A): {}", stringify_mask(self.0.is_set(SPSR_EL1::A)))?;
        writeln!(f, "        IRQ    (I): {}", stringify_mask(self.0.is_set(SPSR_EL1::I)))?;
        writeln!(f, "        FIQ    (F): {}", stringify_mask(self.0.is_set(SPSR_EL1::F)))?;
        
        // Pretty-print the illegal execution state.
        write!(
            f,
            "    Illegal Execution State (IL): {}",
            stringify_flag(self.0.is_set(SPSR_EL1::IL)),
        )
    }
}

#[inline]
fn alternating_newline(i: usize) -> &'static str {
    if i % 2 == 0 {
        "   "
    } else {
        "\n"
    }
}

impl fmt::Display for ExceptionContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Raw print the whole ELR_EL1 register.
        writeln!(f, "ELR_EL1: {:#018x}", self.elr_el1)?;
        // Pretty-print the SPSR_EL1 register.
        writeln!(f, "{}\n", self.spsr_el1)?;

        // Raw print 3 general-purpose registers per line.
        writeln!(f, "General purpose registers:")?;
        for (i, reg) in self.gpr.iter().enumerate() {
            write!(f, "    x{:<2}: {:>#018x}{}", i, reg, alternating_newline(i))?;
        }

        // Raw print the link register.
        write!(f, "    lr: {:#018x}", self.lr)
    }
}
