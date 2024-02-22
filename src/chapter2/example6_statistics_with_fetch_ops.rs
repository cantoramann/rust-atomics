use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicU64, AtomicUsize};
use std::thread;
use std::time::Instant;

pub fn example6_statistics_with_fetch_ops() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        // Four background threads to process all 100 items
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    process_item(t * 25 * i);
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Relaxed);
                    total_time.fetch_add(time_taken, Relaxed);
                    max_time.fetch_max(time_taken, Relaxed);
                }
            });
        }

        loop {
            let total_time = std::time::Duration::from_micros(total_time.load(Relaxed));
            let max_time = std::time::Duration::from_micros(max_time.load(Relaxed));
            let n = num_done.load(Relaxed);
            if n == 0 {
                println!("Working.. nothing done yet");
            } else if n == 100 {
                break;
            } else {
                println!(
                    "Working.. {n}/100 done, {:?} average, {:?} peak",
                    total_time / n as u32,
                    max_time
                );
            }
            thread::sleep(std::time::Duration::from_secs(1)); // No more signaling, so unnecessary waits will happen again, like in example3. The main thread will sometimes wake up between 4 ops, sometimes 3, sometimes who knows.
        }
    });

    println!("All done!");
}

fn process_item(_i: usize) -> u64 {
    // Do some work
    thread::sleep(std::time::Duration::from_secs(1));
    5 * 6 * 7 * 8 * 9 * 1124
}
