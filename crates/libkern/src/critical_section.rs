//! Implementation of kernel [Critical Sections].
//!
//! [Critical Section]: https://en.wikipedia.org/wiki/Critical_section

use core::marker::PhantomData;

use lock_api::RawMutex;

use crate::irq::without_interrupts;

/// Critical section token.
///
/// An instance of this type indicates the execution of a critical section
/// for the duration of lifetime `'cs`. This means that interrupts are
/// disabled and synchronization is employed so that only the current CPU
/// can execute it at the time.
///
/// User code should not try to construct this type; instead APIs should be
/// designed around it:
///
/// * to grant access to resources for the duration of `'cs`
///
/// * APIs can consume the token for static validation that the caller is
///   inside a critical section.
#[derive(Clone, Copy, Debug)]
pub struct Token<'cs> {
    // Invariant over `'cs`, to make sure it cannot shrink.
    _0: PhantomData<&'cs mut &'cs ()>,
}

/// Meta type through which a critical section can be entered.
#[repr(transparent)]
pub struct CriticalSection<Lock: RawMutex> {
    lock: Lock,
}

impl<Lock: RawMutex> CriticalSection<Lock> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self { lock: Lock::INIT }
    }

    /// Enters a critical section and executes the supplied closure.
    ///
    /// A section [`Token`] bound to lifetime `'cs` will be constructed
    /// and passed into the closure for use in one of the motives listed
    /// in its documentation.
    ///
    /// For the duration of the critical section, hardware interrupts will
    /// be disabled and it is guaranteed that only one core can execute
    /// the section at a time.
    ///
    /// # Safety
    ///
    /// This is hardware land. Use cautiously.
    #[inline(always)]
    pub unsafe fn enter<F, R>(&'static self, f: F) -> R
    where
        F: FnOnce(Token<'_>) -> R,
    {
        without_interrupts(|| {
            let token = Token { _0: PhantomData };

            self.lock.lock();
            let result = f(token);
            self.lock.unlock();

            result
        })
    }
}
