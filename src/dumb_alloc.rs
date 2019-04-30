use core::alloc::{GlobalAlloc, Layout};
use core::arch::wasm32;
use core::ptr::{null_mut, NonNull};

#[cfg(feature = "nightly")]
use core::alloc::AllocErr;

#[cfg(not(feature = "nightly"))]
pub struct AllocErr;

pub struct DumbAlloc {
    /*head: *mut u8,
end: *mut u8,*/}

const PAGE_SIZE: usize = 65536;

fn round_to_align(size: usize, align: usize) -> usize {
    size + (align - (size % align))
}

unsafe impl Sync for DumbAlloc {}

impl DumbAlloc {
    pub const INIT: Self = DumbAlloc {
        /*head: 0 as *mut u8,
        end: 0 as *mut u8,*/
    };

    fn alloc_impl(&self, layout: Layout) -> Result<*mut u8, AllocErr> {
        if layout.size() == 0 || layout.align() == 0 {
            return Err(AllocErr);
        }

        let size = round_to_align(layout.size(), layout.align());

        let pages = (size / PAGE_SIZE) + 1;

        self.alloc_pages(pages)
    }

    fn alloc_pages(&self, pages: usize) -> Result<*mut u8, AllocErr> {
        let ptr = wasm32::memory_grow(0, pages);
        if ptr != usize::max_value() {
            let ptr = (ptr * PAGE_SIZE) as *mut u8;
            Ok(ptr as *mut u8)
        } else {
            Err(AllocErr)
        }
    }
}

unsafe impl GlobalAlloc for DumbAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match self.alloc_impl(layout) {
            Ok(ptr) => ptr,
            Err(AllocErr) => null_mut(),
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {}
}
