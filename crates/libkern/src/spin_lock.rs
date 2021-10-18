//! Implementations of cache-aligned and unaligned spin locks as
//! synchronization primitives.

use core::cell::UnsafeCell;

use crate::scoped_lock::ScopedLock;

/// A [spin lock](https://en.m.wikipedia.org/wiki/Spinlock) providing mutually
/// exclusive acccess to a value.
pub type SpinLock<T> = ScopedLock<T, UnalignedSpinLockImpl>;

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/spin_lock.rs"]
mod arch_spin_lock;
use self::arch_spin_lock::UnalignedSpinLock as UnalignedSpinLockImpl;

impl<T> ScopedLock<T, UnalignedSpinLockImpl> {
    /// Creates a new unaligned spin lock around a given value.
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self::new_with_impl(value, UnalignedSpinLockImpl::new())
    }
}

impl<T: Default> Default for ScopedLock<T, UnalignedSpinLockImpl> {
    #[inline(always)]
    fn default() -> Self {
        Self::new(T::default())
    }
}
