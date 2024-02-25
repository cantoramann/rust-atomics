use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);

pub fn example4_spawning_happens_before_relationship() {
    X.store(1, Relaxed);
    let t = thread::spawn(f);
    X.store(2, Relaxed); // The thread t can read the value 2 from X. Or, it can run before this line and read the value 1 from X.
    t.join().unwrap();
    X.store(3, Relaxed);
}

fn f() {
    let x = X.load(Relaxed);
    assert!(x == 1 || x == 2);
}
