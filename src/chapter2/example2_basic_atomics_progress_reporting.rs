use std::io::{self, Write};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::{self, sleep};

pub fn example2_basic_atomics_progress_reporting() {
    let num_done = AtomicUsize::new(0);

    thread::scope(|s| {
        // A background thread to process all the 100 items.
        s.spawn(|| {
            for i in 0..100 {
                process_item(i); // Assume this takes some time
                num_done.store(i + 1, Relaxed);
            }
        });

        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            };
            println!("Working... {n}/100 done");
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    println!("All items processed");
}

fn process_item(i: usize) -> u32 {
    sleep(std::time::Duration::from_secs(1));
    5 * 6 * 7 * 8 * 9 * 1124 * i as u32
}
