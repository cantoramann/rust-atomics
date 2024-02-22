use std::thread;

pub fn example5_basic_spawn_scoped() {
    let numbers = vec![1, 2, 3];

    thread::scope(|s| {
        // no move constructor needed, because we get the scope. The assumption is that the scope will outlive the thread.
        // In other words, no thread spawned under the scope s can outlive the scope s.
        s.spawn(|| {
            println!("Length: {}", numbers.len());
        });
        s.spawn(|| {
            s.spawn(|| {
                for n in &numbers {
                    println!("{n}");
                }
            });
        });
    });
}
