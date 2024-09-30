use core::{
    alloc::GlobalAlloc,
    sync::atomic::{AtomicUsize, Ordering},
};

#[global_allocator]
static mut ALLOC: GlobalAllocator = GlobalAllocator::new();
struct GlobalAllocator {
    pos: AtomicUsize,
}

impl GlobalAllocator {
    const fn new() -> Self {
        Self {
            pos: AtomicUsize::new(0),
        }
    }
    unsafe fn init(&mut self, start: usize) {
        self.pos = AtomicUsize::new(start);
    }
}
unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let size = next_power_of_two(layout.size());
        self.pos.fetch_add(size, Ordering::SeqCst) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let mut ptr = ptr;
        let size = next_power_of_two(layout.size());
        for index in 0..size {
            ptr.write(0);
            ptr = ptr.add(index);
        }
    }
}

pub fn init(start: usize) {
    let alloc_mut = &raw mut ALLOC;
    unsafe {
        (*alloc_mut).init(start);
    }
}

fn next_power_of_two(num: usize) -> usize {
    let best_high_bit = (usize::BITS - num.leading_zeros() - 1) as usize;
    if num == 1 << best_high_bit {
        num
    } else {
        1 << (best_high_bit + 1)
    }
}
