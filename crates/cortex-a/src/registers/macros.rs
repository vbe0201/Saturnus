/// Read the raw value from a MSR.
macro_rules! read_msr {
    ($T:ty, $width:literal, $name:literal) => {
        #[allow(dead_code)]
        match () {
            #[cfg(target_arch = "aarch64")]
            () => unsafe {
                let x: $T;
                asm!(concat!("mrs {0:", $width, "}, ", $name), out(reg) x, options(nomem, nostack));
                x
            },

            #[cfg(not(target_arch = "aarch64"))]
            () => unimplemented!(),
        }
    };
}

/// Write the raw value from a MSR.
macro_rules! write_msr {
    ($width:literal, $name:literal, $val:expr) => {
        match $val {
            #[cfg(target_arch = "aarch64")]
            val => unsafe { asm!(concat!("msr ", $name, ", {0:", $width, "}"), in(reg) val, options(nomem, nostack)) },

            #[cfg(not(target_arch = "aarch64"))]
            _ => unimplemented!(),
        }
    };
}

macro_rules! impl_set_msr {
    ($T:ty, $width:literal, $name:literal) => {
        #[inline]
        fn set(&self, val: $T) {
            write_msr!($width, $name, val);
        }
    };
}

macro_rules! impl_get_msr {
    ($T:ty, $width:literal, $name:literal) => {
        #[inline]
        fn get(&self) -> $T {
            read_msr!($T, $width, $name)
        }
    };
}
