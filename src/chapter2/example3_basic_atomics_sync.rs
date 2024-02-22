use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

pub fn example3_basic_atomics_sync() {
    let num_done = AtomicUsize::new(0);

    let main_thread = thread::current();

    thread::scope(|s| {
        // A background thread to process all the 100 items.
        s.spawn(|| {
            for i in 0..100 {
                process_item(i);
                num_done.store(i + 1, Relaxed);
                main_thread.unpark();
            }
        });

        // The main thread shows status updates
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            };
            println!("Working... {}/100 done", n);
            thread::park_timeout(std::time::Duration::from_secs(100)); // Now the main thread can be interrupted!
        }
    });

    println!("All items processed");
}

fn process_item(i: usize) -> u32 {
    thread::sleep(std::time::Duration::from_secs(1));
    5 * 6 * 7 * 8 * 9 * 1124 * i as u32
}
