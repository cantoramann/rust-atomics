use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

// should work, as compare_change is an atomic operation that includes the condition check.
pub fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let mut id = NEXT_ID.load(Relaxed);

    loop {
        assert!(id < u32::MAX, "too many IDs!");
        match NEXT_ID.compare_exchange_weak(id, id + 1, Relaxed, Relaxed) {
            Ok(_) => return id,
            Err(x) => id = x, // x is the new value that another thread should have changed.
        }
    }
}
