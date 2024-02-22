use std::sync::Arc;
use std::thread;

pub fn example7_shared_ownership_arc() {
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    // This will now work because the reference counter is thread safe - it uses atomic operations
    thread::spawn(move || dbg!(a));
    thread::spawn(move || dbg!(b));
}

pub fn example7_shared_ownership_arc_shadowing_suggested() {
    let a = Arc::new([1, 2, 3]); // This still will be available outside the spawn scope.
    thread::spawn({
        let a = a.clone(); // Shadowing to avoid naming complexity
        move || {
            dbg!(a);
        }
    });
}
