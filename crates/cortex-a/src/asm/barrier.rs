//! Abstractions for executing Aarch64 barriers, like `dmb`, `dsb`, `isb`.

mod sealed {
    pub trait Sealed {}
}

macro_rules! barrier_types {
    ($($(#[$doc:meta])* $name:ident: $asm:literal,)*) => {
        $(
        $(#[$doc])*
        pub enum $name {}

        impl sealed::Sealed for $name {}
        impl BarrierType for $name {
            #[doc(hidden)]
            #[inline(always)]
            unsafe fn __dsb() {
                match () {
                    #[cfg(target_arch = "aarch64")]
                    () => unsafe { ::core::arch::asm!(concat!("dsb ", $asm), options(nostack)) },
                    #[cfg(not(target_arch = "aarch64"))]
                    () => unimplemented!(),
                }
            }

            #[doc(hidden)]
            #[inline(always)]
            unsafe fn __dmb() {
                match () {
                    #[cfg(target_arch = "aarch64")]
                    () => unsafe { ::core::arch::asm!(concat!("dmb ", $asm), options(nostack)) },
                    #[cfg(not(target_arch = "aarch64"))]
                    () => unimplemented!(),
                }
            }
        }
        )*
    };
}

/// Represents the type of a [`dmb`] or [`dsb`] memory barrier.
pub trait BarrierType: sealed::Sealed {
    /// Execute a `dsb` barrier with this type.
    #[doc(hidden)]
    unsafe fn __dsb();

    /// Execute a `dmb` barrier with this type.
    #[doc(hidden)]
    unsafe fn __dmb();
}

barrier_types![
    /// Full system is the required shareability domain, reads and writes are
    /// the required access types, both before and after the barrier instruction.
    /// This option is referred to as the full system barrier.
    SY: "sy",

    /// Full system is the required shareability domain, writes are the required
    /// access type, both before and after the barrier instruction.
    ST: "st",

    /// Full system is the required shareability domain, reads are the required
    /// access type before the barrier instruction, and reads and writes are the
    /// required access types after the barrier instruction.
    LD: "ld",

    /// Inner Shareable is the required shareability domain, reads and writes are
    /// the required access types, both before and after the barrier instruction.
    ISH: "ish",

    /// Inner Shareable is the required shareability domain, writes are the required
    /// access type, both before and after the barrier instruction.
    ISHST: "ishst",

    /// Inner Shareable is the required shareability domain, reads are the required
    /// access type before the barrier instruction, and reads and writes are the
    /// required access types after the barrier instruction.
    ISHLD: "ishld",

    /// Non-shareable is the required shareability domain, reads and writes are the
    /// required access, both before and after the barrier instruction.
    NSH: "nsh",

    /// Non-shareable is the required shareability domain, writes are the required
    /// access type, both before and after the barrier instruction.
    NSHST: "nshst",

    /// Non-shareable is the required shareability domain, reads are the required
    /// access type before the barrier instruction, and reads and writes are the
    /// required access types after the barrier instruction.
    NSHLD: "nshld",

    /// Outer Shareable is the required shareability domain, reads and writes are
    /// the required access types, both before and after the barrier instruction.
    OSH: "osh",

    /// Outer Shareable is the required shareability domain, writes are the required
    /// access type, both before and after the barrier instruction.
    OSHST: "oshst",

    /// Outer Shareable is the required shareability domain, writes are the required
    /// access type, both before and after the barrier instruction.
    OSHLD: "oshld",
];

/// Data Memory Barrier is a memory barrier that ensures the ordering of observations
/// of memory accesses.
///
/// The type of this memory barrier can be specified using the `T` generic type.
#[inline(always)]
pub unsafe fn dmb<T: BarrierType>() {
    unsafe { T::__dmb() }
}

/// Data Synchronization Barrier is a memory barrier that ensures the completion of
/// memory accesses.
///
/// The type of this memory barrier can be specified using the `T` generic type.
#[inline(always)]
pub unsafe fn dsb<T: BarrierType>() {
    unsafe { T::__dsb() }
}

/// Instruction Synchronization Barrier flushes the pipeline in the PE and is a
/// context synchronization event.
///
/// The type of this memory barrier is always [`SY`] as this is the only valid
/// option for a `isb` barrier.
#[inline(always)]
pub unsafe fn isb() {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { ::core::arch::asm!("isb sy", options(nostack)) },
        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}
