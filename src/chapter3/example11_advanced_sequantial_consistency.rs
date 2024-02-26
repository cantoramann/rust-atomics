use std::{
    sync::atomic::{
        AtomicU32,
        Ordering::{Relaxed, SeqCst},
    },
    thread::{self, sleep},
};

static NUM_WRITERS: u32 = 10000;
static NUM_READERS: u32 = 8;

static X: AtomicU32 = AtomicU32::new(1);

pub fn example11_advanced_sequantial_consistency() {
    thread::scope(|s| {
        for i in 0..NUM_READERS {
            // only do some reading based on the current value of X. Do Relaxed ordering.
            s.spawn(move || {
                for j in 0..300 {
                    sleep(std::time::Duration::from_nanos((i * 100 * i) as u64));
                    let data = X.load(SeqCst);
                    println!("Thread {} read count {}, and read value: {}", i, j, data);
                }
            });
        }

        for _ in 0..NUM_WRITERS {
            s.spawn(f);
        }
    });
}

fn f() {
    let _ = X.fetch_add(1, SeqCst);
}
