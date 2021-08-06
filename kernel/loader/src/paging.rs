use core::fmt;

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/paging.rs"]
mod arch_paging;

// FIXME: Make this architecture agnostic!
pub use arch_paging::*;

macro_rules! addr_type {
    ($(#[$attr:meta])* $pub:vis struct $name:ident;) => {
        $(#[$attr])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(transparent)]
        $pub struct $name(usize);

        impl $name {
            /// Create a new address.
            pub fn new(addr: usize) -> Self {
                Self(addr)
            }

            /// Create a new address.
            pub fn from_ptr<T>(ptr: *const T) -> Self {
                Self::new(ptr as usize)
            }

            /// Interpret this address as a pointer to a `T`.
            pub fn as_ptr<T>(self) -> *const T {
                self.0 as *const T
            }

            /// Interpret this address as a pointer to a `T`.
            pub fn as_mut_ptr<T>(self) -> *mut T {
                self.0 as *mut T
            }

            /// Return the raw value of this address.
            pub fn as_usize(self) -> usize {
                self.0
            }
        }

        impl fmt::Pointer for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Pointer::fmt(&self.as_ptr::<u8>(), f)
            }
        }
    };
}

addr_type! {
    /// A Virtual address
    pub struct VirtAddr;
}

addr_type! {
    /// A Physical address
    pub struct PhysAddr;
}
