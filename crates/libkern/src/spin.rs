//! Useful spin-based synchronization primitives for the Kernel.

use crate::critical_section::CriticalSection as CriticalSectionBase;

// Spin Lock implementation guidelines:
//
// - The architecture-specific lock type must implement `lock_api::RawMutex`.
//     - An exemption is the `try_lock` method - we don't need this facility.
//
// - The lock type must ensure exclusivity across multiple executing CPUs.
//
// - The lock type should be constructible in const context.

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/spin_lock.rs"]
mod arch_spin_lock;

#[cfg(target_arch = "aarch64")]
pub use arch_spin_lock::UnalignedSpinLock as UnalignedSpinLockImpl;

/// A spin-based mutex implementation ensuring exclusive access to a resource.
pub type SpinLock<T> = lock_api::Mutex<UnalignedSpinLockImpl, T>;
/// The access guard to a protected resource obtained from locking [`SpinLock`].
pub type SpinLockGuard<'a, T> = lock_api::MutexGuard<'a, UnalignedSpinLockImpl, T>;

/// A critical section that ensures exclusivity through a spin lock.
pub type CriticalSection = CriticalSectionBase<UnalignedSpinLockImpl>;
