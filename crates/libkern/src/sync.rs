//! Useful synchronization primitives for the Kernel.

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/spin_lock.rs"]
mod arch_spin_lock;

#[cfg(target_arch = "aarch64")]
use arch_spin_lock::UnalignedSpinLock as SpinLockImpl;

#[cfg(not(target_arch = "aarch64"))]
type SpinLockImpl = spin::mutex::TicketMutex<()>;

pub type SpinLock<T> = lock_api::Mutex<SpinLockImpl, T>;
pub type SpinLockGuard<'a, T> = lock_api::MutexGuard<'a, SpinLockImpl, T>;
