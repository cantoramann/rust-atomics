use std::thread;

pub fn example2_basic_spawn_join() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from the main thread!");

    t1.join().unwrap();
    t2.join().unwrap();
}

pub fn f() {
    println!("Hello from a spawned thread!");

    let id = thread::current().id();
    println!("My thread id is {id:?}");
}
