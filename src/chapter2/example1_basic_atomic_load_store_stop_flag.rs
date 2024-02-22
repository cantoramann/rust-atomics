use std::io::{self, Write};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::{self, sleep}; // Make sure to include Write trait for flush()

pub fn example1_basic_atomic_load_store_stop_flag() {
    static STOP: AtomicBool = AtomicBool::new(false);

    // Spawn a thread to do the work
    let background_thread = thread::spawn(|| {
        println!("Background thread started");
        while !STOP.load(Relaxed) {
            some_work();
        }
    });

    // use the main thread to listen for user input
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => {
                println!("commands: help, stop");
                io::stdout().flush().unwrap(); // Manually flush stdout to ensure the output is displayed
            }
            "stop" => break,
            cmd => {
                println!("unknown command: {cmd:?}");
                io::stdout().flush().unwrap(); // Flush after printing unknown command
            }
        }
    }

    // Inform the background thread it needs to stop
    STOP.store(true, Relaxed);

    // Wait until the background thread finishes
    background_thread.join().unwrap();
}

fn some_work() -> i32 {
    sleep(std::time::Duration::from_secs(10));
    println!("Background thread woke up from sleep");
    io::stdout().flush().unwrap(); // Manually flush stdout to ensure the output is displayed

    // write me a computation heavy math
    let computation_heavy_math = 1 * 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10;
    computation_heavy_math
}
