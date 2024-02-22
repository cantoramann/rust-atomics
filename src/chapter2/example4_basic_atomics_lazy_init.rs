use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::sleep;

pub fn example4_basic_atomics_lazy_init() {
    static X: AtomicU64 = AtomicU64::new(0); // Assume that the value won't be 0 after read
    let mut x = X.load(Relaxed);
    if x == 0 {
        x = calculate_x();
        X.store(x, Relaxed);
    }
}

fn calculate_x() -> u64 {
    sleep(std::time::Duration::from_secs(10));
    5 * 6 * 7 * 8 * 9 * 1124
}
