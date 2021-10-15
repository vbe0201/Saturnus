struct FreePageList {
    head: Option<&'static mut FreePageFrame>,
}

struct FreePageFrame {
    next: Option<&'static mut FreePageFrame>,
    size: usize,
}

pub struct InitialPageAllocator {
    start_address: usize,
    next_free_address: usize,
    page_list: FreePageList,
}

pub struct InitialPageTable;
