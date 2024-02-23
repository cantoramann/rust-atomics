use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

pub fn example2_out_of_thin_air_values_cyclic_dependencies() {
    let a = thread::spawn(|| {
        let x = X.load(Relaxed);
        Y.store(x, Relaxed);
    });

    let b = thread::spawn(|| {
        let y = Y.load(Relaxed);
        X.store(y, Relaxed);
    });

    a.join().unwrap();
    b.join().unwrap();

    assert_eq!(X.load(Relaxed), 0);
    assert_eq!(Y.load(Relaxed), 0);
}
