use std::sync::atomic::Ordering::Relaxed;
use std::{sync::atomic::AtomicUsize, thread};

pub fn example_5_progress_reporting_multiple_threads_fetch_add() {
    let num_done = &AtomicUsize::new(0); // Reference because given that now we have to move below, we cannot move num_done into the closure - it would be a copy. We don't want that. Also, `Atomic` anything does not implement the `Copy` trait, so we would get an error if we tried to move it to more than one thread.

    thread::scope(|s| {
        // Four background threads to process all 100 items
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    process_item(t * 25 * i);
                    num_done.fetch_add(1, Relaxed);
                }
            });
        }

        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working... {n}/100 done");
            thread::sleep(std::time::Duration::from_secs(1)); // No more signaling, so unnecessary waits will happen again, like in example3. The main thread will sometimes wake up between 4 ops, sometimes 3, sometimes who knows.
        }
    })
}

fn process_item(_i: usize) -> u64 {
    // Do some work
    thread::sleep(std::time::Duration::from_secs(1));
    5 * 6 * 7 * 8 * 9 * 1124
}
