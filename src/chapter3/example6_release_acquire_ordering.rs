use std::thread::{self, sleep};
use std::{
    sync::atomic::{
        AtomicBool, AtomicU64,
        Ordering::{Acquire, Relaxed, Release},
    },
    time::Duration,
};

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

pub fn example6_release_acquire_ordering() {
    thread::spawn(|| {
        DATA.store(123, Relaxed);
        sleep(std::time::Duration::from_secs(2)); // Sleep for 2 seconds to see if the it will pass the ownership while sleeping - turns out it does not
        READY.store(true, Release); // Everything from before this store
    });

    while !READY.load(Acquire) {
        thread::sleep(Duration::from_millis(100)); // Sleep for 100 ms to avoid busy waiting
        println!("Waiting...");
    }

    println!("DATA: {}", DATA.load(Relaxed));
}

pub fn example6_same_example_in_relaxed_ordering() {
    for _ in 0..1000 {
        thread::spawn(|| {
            DATA.store(123, Relaxed);
            READY.store(true, Relaxed);
        });

        println!("DATA: {}", DATA.load(Relaxed));
    }
}
