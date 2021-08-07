// This is an board-specific module that is made available through the path
// attribute. See the generic module, [`crate::bsp`], for orientation.

pub fn adjust_kernel_base(_base: usize) -> Option<usize> {
    // Inside QEMU, we will not trigger a relocation and stay as we are.
    None
}
