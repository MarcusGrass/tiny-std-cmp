use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{fence, AtomicBool, Ordering};

use dlmalloc::Dlmalloc;

#[global_allocator]
static ALLOCATOR: GlobalDlmalloc = GlobalDlmalloc;

struct GlobalDlmalloc;

static mut DLMALLOC: Dlmalloc = Dlmalloc::new();

unsafe impl GlobalAlloc for GlobalDlmalloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        Self::lock();
        let ptr = DLMALLOC.malloc(layout.size(), layout.align());
        Self::unlock();
        ptr
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        Self::lock();
        DLMALLOC.free(ptr, layout.size(), layout.align());
        Self::unlock();
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        Self::lock();
        let ptr = DLMALLOC.calloc(layout.size(), layout.align());
        Self::unlock();
        ptr
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        Self::lock();
        let ptr = DLMALLOC.realloc(ptr, layout.size(), layout.align(), new_size);
        Self::unlock();
        ptr
    }
}

static LOCK: AtomicBool = AtomicBool::new(false);
impl GlobalDlmalloc {
    fn lock() {
        while LOCK
            .compare_exchange_weak(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {}
        fence(Ordering::SeqCst);
    }

    fn unlock() {
        LOCK.store(false, Ordering::SeqCst);
    }
}

fn main() {
    thread_shared_memory_sequential_count();
}
const THREAD_LOOP_COUNT: usize = 100_000;

#[inline]
fn thread_shared_memory_sequential_count() {
    let run_for = THREAD_LOOP_COUNT;
    let mut sum = 0;
    for _ in 0..run_for {
        let handle = std::thread::spawn(move || core::hint::black_box(1));
        sum += handle.join().unwrap();
    }
    assert_eq!(run_for, sum);
}
