use cortex_a::paging::{PhysAddr, VirtAddr};
use tock_registers::{interfaces::Readable, register_bitfields, registers::InMemoryRegister};

use crate::page_allocator::PageAllocator;

/// Errors that can happen while performing page table operations.
#[derive(Clone, Copy, Debug)]
pub enum Error {
    /// Virtual address was mapped already.
    AlreadyMapped,
    /// One of the given addresses was not aligned to 4 KiB.
    UnalignedAddress,
}

/// Raw representation of a page table.
#[repr(C, align(0x1000))]
pub struct PageTable([u64; 512]);

impl PageTable {
    fn zeroed() -> Self {
        Self([0; 512])
    }
}

/// A page table that allows mapping of 4 KiB pages, using 4 KiB granule.
pub struct PageTableMapper {
    table: *mut PageTable,
}

impl PageTableMapper {
    /// Create a new page table by allocating a page from the given page allocator.
    pub fn new(page_alloc: &PageAllocator) -> Self {
        Self {
            table: unsafe {
                let page = page_alloc.allocate().cast::<PageTable>();
                page.as_ptr().write(PageTable::zeroed());
                page.as_ptr()
            },
        }
    }

    /// Return the pointer to the root page table, which can be inserted into the
    /// translation system registers.
    pub fn root_ptr(&self) -> *const PageTable {
        self.table
    }

    /// Map a single 4 KiB page from `paddr` to `vaddr` in virtual memory space.
    pub fn map(
        &mut self,
        paddr: PhysAddr,
        vaddr: VirtAddr,
        attrs: InMemoryRegister<u64, PAGE_DESCRIPTOR::Register>,
        page_alloc: &PageAllocator,
    ) -> Result<(), Error> {
        if paddr.as_usize() & 0xFFF != 0 || vaddr.as_usize() & 0xFFF != 0 {
            return Err(Error::UnalignedAddress);
        }

        let indices = indices(vaddr);
        let mut table = self.table;

        for (lvl, idx) in indices.into_iter().rev().enumerate() {
            let entry = unsafe { &mut *table.cast::<u64>().add(idx as usize) };

            // if we reached the lowest level, perform the mapping operation
            if lvl == 2 {
                // only use flags that are in the upper and lower attributes block of the
                // descriptor
                let attrs = attrs.get() & !0xFFFF_FFFF_F000;
                *entry = paddr.as_usize() as u64 | attrs | 0b11;
                return Ok(());
            }

            match *entry & 0b11 {
                // this entry is invalid, so we need to make it point to the next
                // level of page tables
                0b00 => {
                    // allocate a new table and zero it
                    let new_table = unsafe {
                        let ptr = page_alloc.allocate().cast::<PageTable>();
                        ptr.as_ptr().write(PageTable::zeroed());
                        ptr
                    };

                    // point the entry to the new table and mark it as a table descriptor
                    *entry = (new_table.as_ptr() as u64 >> 12) | 0b11;

                    // walk the new table
                    table = new_table.as_ptr();
                }
                // this entry points to the next page table, so follow it
                0b11 => {
                    let new_table = (*entry >> 12 << 12) as *mut PageTable;
                    table = new_table;
                }
                // if this entry is a page descriptor, the address is already mapped
                0b01 => return Err(Error::AlreadyMapped),
                _ => unreachable!(),
            }
        }

        unreachable!()
    }

    /// Map `count` bytes from `vaddr` to `paddr`.
    pub fn map_many(
        &mut self,
        paddr: PhysAddr,
        vaddr: VirtAddr,
        count: usize,
        attrs: InMemoryRegister<u64, PAGE_DESCRIPTOR::Register>,
        page_alloc: &PageAllocator,
    ) -> Result<(), Error> {
        for idx in 0..((count + 0xFFF) / 0x1000) {
            let vaddr = VirtAddr::new(vaddr.as_usize() + idx * 0x1000);
            let paddr = PhysAddr::new(paddr.as_usize() + idx * 0x1000);
            self.map(paddr, vaddr, InMemoryRegister::new(attrs.get()), page_alloc)?;
        }

        Ok(())
    }
}

/// Get all three page table indices from the given virtual address.
fn indices(vaddr: VirtAddr) -> [usize; 3] {
    let mut indices = [0; 3];
    let mut shift = 12;

    for vpn in indices.iter_mut() {
        *vpn = (vaddr.as_usize() >> shift) & 0x1FF;
        shift += 9;
    }

    indices
}

// A level 3 page descriptor, as per ARMv8-A Architecture Reference Manual Figure D5-17.
register_bitfields! {u64,
    pub PAGE_DESCRIPTOR [
        /// Unprivileged execute-never.
        UXN OFFSET(54) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Privileged execute-never.
        PXN OFFSET(53) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Access flag.
        AF OFFSET(10) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Shareability field.
        SH       OFFSET(8) NUMBITS(2) [
            OuterShareable = 0b10,
            InnerShareable = 0b11
        ],

        /// Access Permissions.
        AP OFFSET(6) NUMBITS(2) [
            RW_EL1 = 0b00,
            RW_EL1_EL0 = 0b01,
            RO_EL1 = 0b10,
            RO_EL1_EL0 = 0b11
        ],

        /// Memory attributes index into the MAIR_EL1 register.
        AttrIndx OFFSET(2) NUMBITS(3) [],
    ]
}
