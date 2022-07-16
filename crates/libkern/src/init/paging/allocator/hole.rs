use core::{alloc::Layout, ptr::NonNull};

use super::cursor::Cursor;

// A hole in the allocator's memory region that marks
// a chunk of free, allocatable storage of `size` bytes.
// Holes form a linked list, each referencing their neighbor.
pub struct Hole {
    pub next: Option<NonNull<Self>>,
    pub size: usize,
}

// A linked list of holes, starting at the head node.
#[repr(transparent)]
pub struct HoleList {
    pub head: Option<NonNull<Hole>>,
}

impl HoleList {
    pub const fn empty() -> Self {
        Self { head: None }
    }

    pub fn cursor(&mut self) -> Option<Cursor> {
        Cursor::new(self)
    }

    pub fn is_allocatable(&mut self, layout: Layout) -> bool {
        #[inline]
        fn is_allocatable_impl(list: &mut HoleList, layout: Layout) -> Result<(), ()> {
            let mut cursor = list.cursor().ok_or(())?;

            loop {
                if cursor.is_current_allocatable(layout) {
                    return Ok(());
                }

                cursor = cursor.advance().ok_or(())?;
            }
        }

        is_allocatable_impl(self, layout).is_ok()
    }

    #[allow(clippy::result_unit_err)]
    pub unsafe fn try_allocate(&mut self, address: usize, size: usize) -> Result<*mut u8, ()> {
        let mut cursor = self.cursor().ok_or(())?;

        loop {
            match cursor.split_current(address, size) {
                Ok((ptr, _size)) => return Ok(ptr),
                Err(c) => cursor = c.advance().ok_or(())?,
            }
        }
    }

    pub unsafe fn free(&mut self, mut hole: NonNull<Hole>, size: usize) {
        let mut cursor = match self.cursor() {
            Some(cursor) => cursor,
            None => {
                *hole.as_mut() = Hole { next: None, size };
                self.head = Some(hole);

                return;
            }
        };

        loop {
            match cursor.coalesce_current(hole, size) {
                Ok((mut c, link_prev)) => {
                    if link_prev {
                        c.set_previous_next(Some(hole));
                    }
                    break;
                }
                Err(c) => {
                    let mut current = c.current_ptr();
                    match c.advance() {
                        Some(c) => cursor = c,
                        None => {
                            *hole.as_mut() = Hole { next: None, size };
                            current.as_mut().next = Some(hole);
                            break;
                        }
                    }
                }
            }
        }
    }
}
