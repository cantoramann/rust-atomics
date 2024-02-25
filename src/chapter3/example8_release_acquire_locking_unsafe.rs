use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::thread;

static mut DATA: String = String::new();
static mut WAITER_COUNT: u64 = 0;
static LOCKED: AtomicBool = AtomicBool::new(false);

pub fn example8_release_acquire_locking_unsafe() {
    thread::scope(|s| {
        for _ in 0..100000 {
            s.spawn(f);
        }
    });

    println!("DATA: {}", unsafe { DATA.len() });
    println!("WAITER_COUNT: {}", unsafe { WAITER_COUNT });
}

fn f() {
    println!("Got in `f`: clock cycle {}", unsafe {
        core::arch::x86_64::_rdtsc()
    });
    if LOCKED
        .compare_exchange(false, true, Acquire, Relaxed)
        .is_ok()
    {
        // We hold the lock if the result of the above operation is true - we Acquire on success (here), and Relaxed ordering on failure
        // In the above `compare_exchange` example, we don't care about the failure, as the lock is held by someone else, and we cannot access the data
        unsafe { DATA.push('!') };
        println!("Leaving the lock: clock cycles {}", unsafe {
            core::arch::x86_64::_rdtsc()
        });

        LOCKED.store(false, Release);
    } else {
        unsafe { WAITER_COUNT += 1 };
    }
}
