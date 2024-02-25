use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;

static X: AtomicI32 = AtomicI32::new(0);

pub fn example5_basic_relaxed_ordering() {
    for _ in 0..100000 {
        let handle1 = std::thread::spawn(|| {
            a();
        });

        let handle2 = std::thread::spawn(|| {
            b();
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
    }
}

fn a() {
    X.fetch_add(5, Relaxed);
    X.fetch_add(10, Relaxed);
}

fn b() {
    let a = X.load(Relaxed);
    let b = X.load(Relaxed);
    let c = X.load(Relaxed);
    let d = X.load(Relaxed);

    // if not all values are equal, print them
    if a != b || b != c || c != d {
        println!("{} {} {} {}", a, b, c, d);
    }
}
