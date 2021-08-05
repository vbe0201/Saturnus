use crate::StaticCell;

#[used]
#[link_section = ".vectors"]
static EXCEPTION_TABLE: StaticCell<ExceptionVectorTable> =
    StaticCell::new(ExceptionVectorTable::empty());

/// A single aarch64 exception vector.
pub type ExceptionVector = unsafe extern "C" fn() -> !;

/// An [`ExceptionVector`] which is aligned to `0x80` bytes, so that a correct layout of an
/// [`ExceptionVectorTable`] can be guaranteed only by specifying the alignment.
#[derive(Clone, Copy)]
#[repr(align(0x80))]
pub struct AlignedExceptionVector(pub ExceptionVector);

// required for the vector table to keep layout compatibility
static_assertions::assert_eq_size!(AlignedExceptionVector, Option<AlignedExceptionVector>);

/// ABI compatible representation of an aarch64 exception vector table.
///
/// # Layout
///
/// Offset | Event type            | Description
/// -------|-----------------------|------------------------
/// 0x000  | Synchronous Exception | EL is using `SP_EL0` stack
/// 0x080  | IRQ                   | EL is using `SP_EL0` stack
/// 0x100  | FIQ                   | EL is using `SP_EL0` stack
/// 0x180  | SError                | EL is using `SP_EL0` stack
/// 0x200  | Synchronous Exception | EL is using `SP_ELx` stack
/// 0x280  | IRQ                   | EL is using `SP_ELx` stack
/// 0x300  | FIQ                   | EL is using `SP_ELx` stack
/// 0x380  | SError                | EL is using `SP_ELx` stack
/// 0x400  | Synchronous Exception | From lower EL in AArch64
/// 0x480  | IRQ                   | From lower EL in AArch64
/// 0x500  | FIQ                   | From lower EL in AArch64
/// 0x580  | SError                | From lower EL in AArch64
/// 0x600  | Synchronous Exception | From lower EL in AArch32
/// 0x680  | IRQ                   | From lower EL in AArch32
/// 0x700  | FIQ                   | From lower EL in AArch32
/// 0x780  | SError                | From lower EL in AArch32
#[repr(C, align(0x800))]
pub struct ExceptionVectorTable(pub [Option<AlignedExceptionVector>; 16]);
static_assertions::assert_eq_size!(ExceptionVectorTable, [u8; 0x800]);

impl ExceptionVectorTable {
    /// Return an exception vector table whose entries are all `0` pointers.
    pub const fn empty() -> Self {
        ExceptionVectorTable([None; 16])
    }
}

/// Sets up the global exception table that is linked into the `.vectors` section.
pub unsafe extern "C" fn setup_exception_table() {
    unsafe extern "C" fn loop_handler() -> ! {
        loop {}
    }

    let table = unsafe { &mut *EXCEPTION_TABLE.get() };

    for ent in table.0.iter_mut() {
        *ent = Some(AlignedExceptionVector(loop_handler));
    }
}
