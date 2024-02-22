use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

// Problem: Rust atomics overflow. So, at 4,294,967,296, it will overflow to 0. Won't be unique anymore.
pub fn allocate_new_id_iter1() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    NEXT_ID.fetch_add(1, Relaxed)
}

// If you want to limit to 1000, this will partially work - yet, before the panic, it will already have incremented to 1001. Other threads can call the function in the meantime before the panic as well. So, the limit isn't strict.
pub fn allocate_new_id_iter2() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Relaxed);
    assert!(id < 1000, "too many IDs!");
    id
}

// Still a brief period of extra ids will be possible, but typically capped by the number of active threads.
pub fn allocate_new_id_iter3() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Relaxed);
    if id >= 1000 {
        NEXT_ID.fetch_sub(1, Relaxed);
        panic!("too many IDs!");
    }
    id
}
