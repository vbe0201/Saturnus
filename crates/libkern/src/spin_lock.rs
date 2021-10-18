//! Implementations of cache-aligned and unaligned spin locks as
//! synchronization primitives.

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/spin_lock.rs"]
mod arch_spin_lock;
use self::arch_spin_lock::UnalignedSpinLock as UnalignedSpinLockImpl;
