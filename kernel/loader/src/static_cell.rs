use core::cell::UnsafeCell;

/// A cell around a `T`, which implements `Send` + `Sync` and can be
/// access using `unsafe`.
#[repr(transparent)]
pub struct StaticCell<T: ?Sized> {
    inner: UnsafeCell<T>,
}

impl<T> StaticCell<T> {
    /// Construct a new instance of a `StaticCell` containing the given value.
    pub const fn new(inner: T) -> Self {
        Self {
            inner: UnsafeCell::new(inner),
        }
    }
}

impl<T: ?Sized> StaticCell<T> {
    /// Gets a mutable pointer to the wrapped value.
    pub const fn get(&self) -> *mut T {
        self.inner.get()
    }

    /// Returns a mutable reference to the underlying data.
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    /// Gets a mutable pointer to the wrapped value.
    /// The difference to [`get`] is that this function accepts a raw pointer,
    /// which is useful to avoid the creation of temporary references.
    pub const fn raw_get(this: *const Self) -> *mut T {
        this as *const T as *mut T
    }
}

unsafe impl<T> Send for StaticCell<T> {}
unsafe impl<T> Sync for StaticCell<T> {}
