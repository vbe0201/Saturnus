/// Declares a new function element for the `.init_array` segment.
///
/// These will run before [`crate::main`].
#[macro_export]
macro_rules! init_array {
    ($name:ident $body:expr) => {
        #[allow(dead_code)]
        pub unsafe extern "C" fn $name() {
            #[link_section = ".init_array"]
            #[used]
            static __INIT_ARRAY_PTR: unsafe extern "C" fn() = $name;

            #[inline(always)]
            fn inner() {
                $body
            }

            inner()
        }
    };
}

/// Get the address one or more linker symbols.
///
/// # Example
///
/// ```ignore
/// # use kernel_loader::linker_symbol;
/// # fn main() {
/// let (start, end): (*mut (), *mut ()) = linker_symbol!(__start__, __end__);
///
/// let init_array_start: *const unsafe extern "C" fn() = linker_symbol!(__init_array_start__ as unsafe extern "C" fn());
/// # }
/// ```
#[macro_export]
macro_rules! linker_symbol {
    ($name:ident as $T:ty) => {
        unsafe {
            let ptr: *mut $T;
            asm!(
                concat!("adrp {0}, ", stringify!($name)),
                concat!("add {0}, {0}, #:lo12:", stringify!($name)),
                out(reg) ptr
            );
            ptr
        }
    };

    ($($name:ident $(as $T:ty)?),*$(,)?) => {
        ($(
            $crate::linker_symbol!($name as $crate::linker_symbol!(@get_ty,$($T)?)),
        )*)
    };

    (@get_ty,) => { () };
    (@get_ty, $T:ty) => { $T };
}
