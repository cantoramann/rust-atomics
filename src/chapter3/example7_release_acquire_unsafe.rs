use std::sync::atomic::{
    AtomicBool, AtomicU64,
    Ordering::{Acquire, Relaxed, Release},
};
use std::thread;

static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

pub fn example7_release_acquire_unsafe() {
    thread::spawn(|| {
        // Safety: Nothing else is accessing DATA
        // because we haven't set the READY flag yet

        unsafe { DATA = 123 };
        READY.store(true, Release);
    });

    while !READY.load(Acquire) {
        thread::sleep(std::time::Duration::from_millis(100)); // Sleep for 100 ms to avoid busy waiting
        println!("Waiting...");
    }

    // Safety: Nothing is mutating DATA, because READY is set
    println!("DATA: {}", unsafe { DATA }); // When not made unsafe: use of mutable static is unsafe and requires unsafe function or block
}
