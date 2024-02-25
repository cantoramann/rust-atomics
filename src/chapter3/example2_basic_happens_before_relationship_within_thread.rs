use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

pub fn example3_basic_happens_before_relationship_within_thread() {
    let handle1 = std::thread::spawn(|| {
        a();
    });

    let handle2 = std::thread::spawn(|| {
        b();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn a() {
    X.store(10, Relaxed);
    Y.store(20, Relaxed); // This line will be executed after the previous line
}

fn b() {
    let r1 = Y.load(Relaxed);
    let r2 = X.load(Relaxed); // This line will be executed after the previous line
    println!("r1: {}, r2: {}", r1, r2);
}
