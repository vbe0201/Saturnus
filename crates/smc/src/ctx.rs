//! Secure Monitor context to define and capture input/output registers.

use libutils::mem::zeroed;

use crate::call::FunctionId;

// The ARM architecture actually only intends R1-R6 to be used for
// argument passing while R0-R3 should contain the results. Nintendo
// however allows R1-R7 for inputs and outputs equally and so will we.

/// The inputs and outputs to a Secure Monitor Call.
///
/// Nintendo's implementation requires the *Function Identifier* as an
/// input to `x0` while `x1` through `x7` are input registers.
///
/// When the call has finished, `x0` stores a result code while `x1`
/// through `x7` may contain outputs or remain unmodified, depending on
/// the inputs and the function being called.
#[derive(Clone, Debug)]
#[repr(C)]
pub struct SecureMonitorContext {
    x: [u64; 8],
}

impl SecureMonitorContext {
    /// Creates a new context with all-zeroed registers.
    #[inline(always)]
    pub const fn new() -> Self {
        // SAFETY: `Smc64Context` uses C ABI and solely holds an array
        // of 8 contiguous integers in memory which are safe to zero.
        unsafe { zeroed() }
    }

    /// Gets a raw pointer to the argument/output memory region
    /// of this context.
    ///
    /// # Safety
    ///
    /// The pointer must not outlive this context object.
    ///
    /// The caller must also ensure that the memory the pointer
    /// (non-transitively) points to is never written to (except inside an
    /// `UnsafeCell`) using this pointer or any pointer derived from it.
    /// If mutable access is needed, use [`Self::as_mut_ptr`] instead.
    ///
    /// The caller must never read past `size_of::<u64>() * 7` bytes offset
    /// from this pointer (which constrains the space for  up to 7 arguments
    /// or output values for a function call).
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.x[1..].as_ptr() as *const _
    }

    /// Returns an unsafe mutable pointer to the argument/output memory
    /// region of this context.
    ///
    /// # Safety
    ///
    /// The pointer must not outlive this context object.
    ///
    /// The caller must never read or write past `size_of::<u64>() * 7`
    /// bytes offset from this pointer (which constrains the space for up
    /// to 7 arguments or output values for a function call).
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.x[1..].as_mut_ptr() as *mut _
    }

    /// Loads in the function to call given its identifier.
    ///
    /// Such identifiers can be constructed using
    /// [`crate::call::make_function_identifier`].
    #[inline]
    pub fn function(mut self, function: FunctionId) -> Self {
        self.x[0] = function as u64;
        self
    }

    /// Loads an argument for the function call at the position denoted
    /// by `idx`.
    ///
    /// # Panics
    ///
    /// This method panics when called with `idx >= 7`. Only up to
    /// 7 arguments (index 6, that is) can be passed along with a
    /// function call.
    #[inline]
    pub fn input<U: Into<u64>>(mut self, idx: usize, value: U) -> Self {
        self.x[idx + 1] = value.into();
        self
    }

    /// Gets an output value at the position denoted by `idx`.
    ///
    /// # Panics
    ///
    /// This method panics when called with `idx >= 7`. Only up to
    /// 7 outputs (index 6, that is) can be retrieved from an SMC
    /// result.
    pub fn output(&self, idx: usize) -> u64 {
        self.x[idx + 1]
    }

    /// Gets the result code of an SMC *after* it has completed.
    #[inline]
    pub fn result(&self) -> u64 {
        self.x[0]
    }
}

impl Default for SecureMonitorContext {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

assert_eq_size!(SecureMonitorContext, [u64; 8]);
