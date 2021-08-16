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

macro_rules! impl_read_write_msr {
    ($(#[$doc:meta])* $name:ident, $reg_T:ty, $T:ty, $width:literal, $msr:literal) => {
        pub struct Reg;

        impl tock_registers::interfaces::Readable for Reg {
            type T = $T;
            type R = $reg_T;

            #[inline]
            fn get(&self) -> $T {
                read_msr!($T, $width, $msr)
            }
        }

        impl tock_registers::interfaces::Writeable for Reg {
            type T = $T;
            type R = $reg_T;

            #[inline]
            fn set(&self, val: $T) {
                write_msr!($width, $msr, val);
            }
        }

        $(#[$doc])*
        pub static mut $name: Reg = Reg {};
    };

    ($(#[$doc:meta])* $name:ident, $T:ty, $width:literal, $msr:literal) => {
        impl_read_write_msr!($(#[$doc])* $name, $name::Register, $T, $width, $msr);
    };
}

macro_rules! impl_read_msr {
    ($(#[$doc:meta])* $name:ident, $reg_T:ty, $T:ty, $width:literal, $msr:literal) => {
        pub struct Reg;

        impl tock_registers::interfaces::Readable for Reg {
            type T = $T;
            type R = $reg_T;

            #[inline]
            fn get(&self) -> $T {
                read_msr!($T, $width, $msr)
            }
        }

        $(#[$doc])*
        pub static mut $name: Reg = Reg {};
    };

    ($(#[$doc:meta])* $name:ident, $T:ty, $width:literal, $msr:literal) => {
        impl_read_msr!($(#[$doc])* $name, $name::Register, $T, $width, $msr);
    };
}
