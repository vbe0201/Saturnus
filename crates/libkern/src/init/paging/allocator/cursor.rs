use core::{
    alloc::Layout,
    mem::align_of,
    num::NonZeroUsize,
    ptr::{addr_of_mut, NonNull},
};

use utils::align::{align_up, is_aligned};

use super::hole::{Hole, HoleList};

// A cursor that walks through the holes of a `HoleList`
// and aids in performing list surgery in-between.
pub struct Cursor {
    // A pointer to the `Hole::next` field of the previous
    // `Hole` in the cursor. When surgery is done on the
    // current hole, the pointed-to value must be updated.
    previous_hole_next: *mut Option<NonNull<Hole>>,
    // A pointer to the current hole in the list.
    current: NonNull<Hole>,
}

impl Cursor {
    pub fn new(list: &mut HoleList) -> Option<Self> {
        list.head.map(|current| Self {
            previous_hole_next: addr_of_mut!(list.head),
            current,
        })
    }

    pub fn advance(mut self) -> Option<Self> {
        self.current_mut().next.map(|next| Self {
            previous_hole_next: addr_of_mut!(self.current_mut().next),
            current: next,
        })
    }

    pub fn set_previous_next(&mut self, hole: Option<NonNull<Hole>>) {
        // SAFETY: The data structure maintains the invariant
        // of always having `previous_hole_next` point to a
        // valid node in the allocated region.
        unsafe {
            *self.previous_hole_next = hole;
        }
    }

    pub fn current_ptr(&self) -> NonNull<Hole> {
        self.current
    }

    pub fn current(&self) -> &Hole {
        unsafe { self.current.as_ref() }
    }

    pub fn current_mut(&mut self) -> &mut Hole {
        unsafe { self.current.as_mut() }
    }

    fn current_last_alloc_addr(&self, layout: Layout) -> usize {
        align_up(self.current.addr().get(), layout.align()) + layout.size() - 1
    }

    pub fn is_current_allocatable(&self, layout: Layout) -> bool {
        let last_hole_addr = self.current.addr().get() + self.current().size - 1;
        let last_alloc_addr = self.current_last_alloc_addr(layout);

        last_alloc_addr <= last_hole_addr
    }

    pub unsafe fn split_current(
        mut self,
        address: usize,
        size: usize,
    ) -> Result<(*mut u8, usize), Self> {
        let start_addr = self.current.addr().get();
        let end_addr = start_addr.wrapping_add(self.current().size);

        if start_addr <= address && address.wrapping_add(size) <= end_addr {
            debug_assert!(is_aligned(address, align_of::<Hole>()));

            // We are in bounds, so calculate the pointer to the next hole.
            let alloc = {
                // SAFETY: `start_addr` is already non-null and
                // `address` is guaranteed to be >= at this point.
                let addr = NonZeroUsize::new_unchecked(address);
                // SAFETY: `current` has provenance over the entire
                // memory region, the resulting pointer inherits it.
                self.current.with_addr(addr)
            };

            // Do fragmentation at front, if necessary.
            if self.current != alloc {
                self.previous_hole_next = addr_of_mut!(self.current_mut().next);

                alloc.as_uninit_mut().write(Hole {
                    next: self.current().next,
                    size: start_addr + self.current().size - address,
                });
                *self.current_mut() = Hole {
                    // SAFETY: `alloc`'s storage is initialized at this point.
                    // Its lifetime is tied to the whole allocation.
                    next: Some(alloc),
                    size: address - start_addr,
                };
            }

            // Do fragmentation at tail, if necessary.
            if alloc.as_ref().size != size {
                // SAFETY:
                //
                // * The caller guarantees that adding `size` to `addr`
                //   does not overflow the address space.
                //
                // * The resulting pointer will inherit `alloc`'s
                //   provenance  over the whole allocator region.
                let next = alloc
                    .map_addr(|addr| NonZeroUsize::new_unchecked(addr.get().wrapping_add(size)));

                next.as_uninit_mut().write(Hole {
                    next: alloc.as_ref().next,
                    size: alloc.as_ref().size - size,
                });
                alloc.as_uninit_mut().write(Hole {
                    next: Some(next),
                    size,
                });
            }

            self.set_previous_next(alloc.as_ref().next);
            Ok((alloc.as_ptr() as *mut u8, size))
        } else {
            Err(self)
        }
    }

    pub unsafe fn coalesce_current(
        mut self,
        mut hole: NonNull<Hole>,
        size: usize,
    ) -> Result<(Self, bool), Self> {
        // Calculate the address range of the hole to coalesce.
        let hole_start_addr = hole.addr().get();
        let hole_end_addr = hole_start_addr + size;

        // Calculate the address range of the current hole.
        let current_start_addr = self.current.addr().get();
        let current_end_addr = current_start_addr + self.current().size;

        // Attempt to merge the holes together, if possible.
        if hole_start_addr < hole_end_addr {
            if hole_end_addr < current_start_addr {
                *hole.as_mut() = Hole {
                    next: Some(self.current),
                    size,
                };
                return Ok((self, true));
            } else if hole_end_addr == current_start_addr {
                *hole.as_mut() = Hole {
                    next: self.current().next,
                    size: self.current().size + size,
                };
                return Ok((self, true));
            }
        } else if current_end_addr == hole_start_addr {
            self.current_mut().size += size;
            return Ok((self, false));
        }

        return Err(self);
    }
}
