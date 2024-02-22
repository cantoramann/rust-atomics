use std::thread;

pub fn example1_basic_spawn() {
    thread::spawn(f);
    thread::spawn(f);

    println!("Hello from the main thread!");
}

pub fn f() {
    println!("Hello from a spawned thread!");

    let id = thread::current().id();
    println!("My thread id is {id:?}");
}
