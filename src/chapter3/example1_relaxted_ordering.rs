use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);

pub fn example1_relaxted_ordering() {
    thread::scope(|s| {
        for i in 0..25 {
            if i % 5 == 0 {
                s.spawn(|| {
                    read();
                });
            }
            s.spawn(move || {
                add(i);
            });
        }
    });
}

fn add(val: i32) {
    // cpu intensive operation
    for _ in 0..10000 {}
    X.fetch_add(val, Relaxed);
}

fn read() {
    let mut vec = Vec::new();
    for i in 0..25 {
        vec.push(X.load(Relaxed));
    }

    println!("{:?}", vec);
}
