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
