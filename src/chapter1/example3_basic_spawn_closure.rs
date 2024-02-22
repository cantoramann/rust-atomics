use std::thread;

pub fn example3_basic_spawn_closure() {
    let numbers = vec![1, 2, 3];

    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    });

    println!("Hello from the main thread!");
}

pub fn f() {
    println!("Hello from a spawned thread!");

    let id = thread::current().id();
    println!("My thread id is {id:?}");
}
