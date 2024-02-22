use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

static KEY: AtomicU64 = AtomicU64::new(0);
static TOTAL_KEY_GENS: AtomicU64 = AtomicU64::new(0);

pub fn example9_compare_exchange_lazy_init() {
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(move || {
                let key = thread_getting_key();
                println!("Globally received key: {key}")
            });
        }
    });

    println!("Total key generations: {}", TOTAL_KEY_GENS.load(Relaxed));
}

fn thread_getting_key() -> u64 {
    let key = KEY.load(Relaxed);

    if key == 0 {
        let new_key = generate_random_key();
        match KEY.compare_exchange(0, new_key, Relaxed, Relaxed) {
            Ok(_) => new_key,
            Err(k) => k,
        }
    } else {
        key
    }
}

fn generate_random_key() -> u64 {
    let new_vec = vec![2, 3];
    let address = &new_vec as *const Vec<i32>;
    let key = address as u64;
    TOTAL_KEY_GENS.fetch_add(1, Relaxed);
    println!("Attempting key generation: Locally created key: {}", key);

    key as u64
}
