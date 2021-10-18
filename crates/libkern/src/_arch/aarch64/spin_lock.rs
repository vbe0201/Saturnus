use core::cell::UnsafeCell;

#[repr(transparent)]
pub struct UnalignedSpinLock {
    packed_tickets: UnsafeCell<u32>,
}

assert_eq_size!(UnalignedSpinLock, u32);

impl UnalignedSpinLock {
    #[inline(always)]
    pub const fn new() -> Self {
        UnalignedSpinLock {
            packed_tickets: UnsafeCell::new(0),
        }
    }

    #[inline(always)]
    pub fn lock(&self) {
        let _temp0: u32;
        let _temp1: u32;
        let _temp2: u32;

        unsafe {
            let mut _packed_tickets = self.packed_tickets.get() as usize;
            asm!(
                r#"
                    prfm pstl1keep, [{packed_tickets:x}]

                1:
                    ldaxr {0:w}, [{packed_tickets:x}]
                    add {2:w}, {0:w}, #0x10000
                    stxr {1:w}, {2:w}, [{packed_tickets:x}]
                    cbnz {1:w}, 1b

                    and {1:w}, {0:w}, #0xFFFF
                    cmp {1:w}, {0:w}, lsr #16
                    b.eq 3f
                    sevl

                2:
                    wfe
                    ldaxrh {1:w}, [{packed_tickets:x}]
                    cmp {1:w}, {0:w}, lsr #16
                    b.ne 2b

                3:
            "#,
                out(reg) _temp0,
                out(reg) _temp1,
                out(reg) _temp2,
                packed_tickets = inout(reg) _packed_tickets,
            )
        }
    }

    #[inline(always)]
    pub fn unlock(&self) {
        unsafe {
            let value = *self.packed_tickets.get() + 1;
            let mut _packed_tickets = self.packed_tickets.get() as usize;
            asm!(
                r#"
                stlrh {0:w}, [{packed_tickets:x}]
            "#,
                in(reg) value,
                packed_tickets = inout(reg) _packed_tickets,
            )
        }
    }
}

unsafe impl Sync for UnalignedSpinLock {}
unsafe impl Send for UnalignedSpinLock {}
