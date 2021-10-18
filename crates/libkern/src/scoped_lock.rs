//! Implementation of a scoped lock that limits access to a resource to a single
//! concurrent reader or writer.

use core::{cell::UnsafeCell, fmt, ops};

/// Generic lock API that must be provided by suitable backend implementations
/// for locking functionality.
pub unsafe trait LockApi: Sync + Send {
    /// Locks the scoped lock.
    fn lock(&self);

    /// Unlocks the scoped lock.
    fn unlock(&self);
}

/// A scoped lock providing exclusive mutable access to a value.
///
/// Data access is guarded by a [`ScopedLockGuard`] which will release the lock
/// for others to re-acquire when it goes out of scope.
pub struct ScopedLock<T: ?Sized, Impl: LockApi> {
    r#impl: Impl,
    data: UnsafeCell<T>,
}

/// A guard that permits exclusive mutable access to a value.
///
/// Can be obtained with [`ScopedLock::lock`] and when it goes out of scope it
/// will release the lock.
pub struct ScopedLockGuard<'s, T: ?Sized + 's, Impl: LockApi + 's> {
    r#impl: &'s Impl,
    data: &'s mut T,
}

impl<T, Impl: LockApi> ScopedLock<T, Impl> {
    #[inline(always)]
    pub(crate) const fn new_with_impl(value: T, r#impl: Impl) -> Self {
        ScopedLock {
            r#impl,
            data: UnsafeCell::new(value),
        }
    }

    // Until const fns in traits become a thing, we implement ScopedLock
    // constructors manually per backend implementation as const fns. See:
    // - libkern/src/spin_lock.rs
}

impl<T, Impl: LockApi> ScopedLock<T, Impl> {
    /// Consumes this lock and unwraps the underlying data.
    #[inline(always)]
    pub fn into_inner(self) -> T {
        // This is statically guaranteed to be the only active
        // reference to this object so we don't have to lock.
        let Self { data, .. } = self;
        data.into_inner()
    }

    /// Gets exclusive mutable access to the underlying data.
    ///
    /// This operation does not require locking as we can guarantee
    /// unique access through Rust's static safety.
    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut T {
        // This is statically guaranteed to be the only active
        // reference to the value so we don't have to lock.
        self.data.get_mut()
    }
}

impl<T: ?Sized, Impl: LockApi> ScopedLock<T, Impl> {
    /// Locks the scoped lock and returns a guard that permits access to the
    /// inner data.
    ///
    /// Said guard may be dereferenced for direct access to the underlying
    /// data and when the guard falls out of scope, the lock is released.
    #[inline(always)]
    pub fn lock(&self) -> ScopedLockGuard<T, Impl> {
        self.r#impl.lock();

        ScopedLockGuard {
            r#impl: &self.r#impl,
            data: unsafe { &mut *self.data.get() },
        }
    }
}

unsafe impl<T: ?Sized + Send, Impl: LockApi> Sync for ScopedLock<T, Impl> {}
unsafe impl<T: ?Sized + Send, Impl: LockApi> Send for ScopedLock<T, Impl> {}

impl<'s, T: ?Sized, Impl: LockApi> ScopedLockGuard<'s, T, Impl> {
    /// Leak the lock guard, returning a mutable reference to the underlying data.
    ///
    /// This function will consume and permanently lock the original [`ScopedLock`]
    /// into only having its data available.
    #[inline(always)]
    pub fn leak(this: Self) -> &'s mut T {
        let data = this.data as *mut _; // Keep the pointer to avoid double-aliasing.
        core::mem::forget(this);
        unsafe { &mut *data }
    }
}

impl<'s, T: ?Sized, Impl: LockApi> ops::Deref for ScopedLockGuard<'s, T, Impl> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'s, T: ?Sized, Impl: LockApi> ops::DerefMut for ScopedLockGuard<'s, T, Impl> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl<'s, T: ?Sized + fmt::Debug, Impl: LockApi> fmt::Debug for ScopedLockGuard<'s, T, Impl> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<'s, T: ?Sized + fmt::Display, Impl: LockApi> fmt::Display for ScopedLockGuard<'s, T, Impl> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<'s, T: ?Sized, Impl: LockApi> Drop for ScopedLockGuard<'s, T, Impl> {
    /// Dropping the lock guard will release the lock it was created from.
    fn drop(&mut self) {
        self.r#impl.unlock();
    }
}
