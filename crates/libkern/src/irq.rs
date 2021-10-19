//! Handling and processing of interrupts for the executing core.

/// Represents the interrupt state that is configured for the executing core.
#[repr(u32)]
pub enum InterruptState {
    /// Interrupts enabled on the execution core.
    Enabled,
    /// Interrupts disabled on the execution core.
    Disabled,
}

#[cfg(target_arch = "aarch64")]
impl tock_registers::fields::TryFromValue<u64> for InterruptState {
    type EnumType = Self;

    fn try_from(v: u64) -> Option<Self::EnumType> {
        use InterruptState::*;
        // Match DAIF mask states.
        match v {
            0b0 => Some(Disabled),
            0b1 => Some(Enabled),
            _ => None,
        }
    }
}

/// Disables interrupts on the executing core.
///
/// This does not place instruction synchronization after the `msr` as per
/// ARMv8-A Architecture Reference Manual, section C5.1.3:
///
/// "Writes to PSTATE.{PAN, D, A, I, F} occur in program order without the need
/// for additional synchronization."
///
/// Returns the previous state of the IRQ mask bit represented as [`InterruptState`].
/// This should be used when restoring the former state with [`restore_interrupts`]
/// is desired.
///
/// # Safety
///
/// This is hardware land. Use cautiously.
#[inline(always)]
pub unsafe fn disable_interrupts() -> InterruptState {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe {
            use cortex_a::registers::DAIF;
            use tock_registers::interfaces::Readable;

            // SAFETY: `DAIF::I` is a single bit. The conversion to
            // `InterruptState` is infallible in every possible case.
            let state = DAIF.read_as_enum::<InterruptState>(DAIF::I).unwrap();
            asm!("msr daifset, #2", options(nomem, nostack, preserves_flags));

            state
        },

        () => unimplemented!(),
    }
}

/// Enables interrupts on the executing core.
///
/// This does not place instruction synchronization after the `msr` as per
/// ARMv8-A Architecture Reference Manual, section C5.1.3:
///
/// "Writes to PSTATE.{PAN, D, A, I, F} occur in program order without the need
/// for additional synchronization."
///
/// Returns the previous state of the IRQ mask bit represented as [`InterruptState`].
/// This should be used when restoring the former state with [`restore_interrupts`]
/// is desired.
///
/// # Safety
///
/// This is hardware land. Use cautiously.
#[inline(always)]
pub unsafe fn enable_interrupts() -> InterruptState {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe {
            use cortex_a::registers::DAIF;
            use tock_registers::interfaces::Readable;

            // SAFETY: `DAIF::I` is a single bit. The conversion to
            // `InterruptState` is infallible in every possible case.
            let state = DAIF.read_as_enum::<InterruptState>(DAIF::I).unwrap();
            asm!("msr daifclr, #2", options(nomem, nostack, preserves_flags));

            state
        },

        () => unimplemented!(),
    }
}

/// Restores a given [`InterruptState`] to the configuration for the executing core.
///
/// This may be used to revert a [`disable_interrupts`] or [`enable_interrupts`]
/// operation to the former system configuration after a critical section is done.
///
/// # Safety
///
/// This is hardware land. Use cautiously.
#[inline(always)]
pub unsafe fn restore_interrupts(state: InterruptState) {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe {
            use cortex_a::registers::DAIF;
            use tock_registers::interfaces::ReadWriteable;

            DAIF.modify(DAIF::I.val(state as u64));
        },

        () => unimplemented!(),
    }
}
