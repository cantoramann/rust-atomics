use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

static mut TOTAL_LOCKS: u32 = 0;

struct Data {
    a: i32,
    b: i32,
    value: String,
}

pub fn example9_release_acquire_lazy_init() {
    let num_spawns = 8;
    std::thread::scope(|s| {
        for i in 0..num_spawns {
            s.spawn(|| {
                for _ in 0..100000 {
                    let data = get_data();
                    assert_eq!(data.a, 1);
                    assert_eq!(data.b, 2);
                    assert_eq!(data.value, "Hello, World!");
                }
            });
        }
    });

    println!("Failures: {}", unsafe { TOTAL_LOCKS });
    println!("Number of threads: {}", num_spawns);
}

fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

    let mut p = PTR.load(Acquire);

    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));
        if let Err(e) = PTR.compare_exchange(std::ptr::null_mut(), p, Release, Acquire) {
            // Safety: `p` comes from `Box`, specifically `Box::into_raw`
            // `p` wasn't shared with any other thread
            drop(unsafe { Box::from_raw(p) }); // Drop `p` because the pointer has already been set by another thread
            p = e;
        } else {
            unsafe { TOTAL_LOCKS += 1 };
        }
    }

    unsafe { &*p }
}

fn generate_data() -> Data {
    Data {
        a: 1,
        b: 2,
        value: "Hello, World!".to_string(),
    }
}
