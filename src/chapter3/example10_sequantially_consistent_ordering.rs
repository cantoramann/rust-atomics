use std::{
    sync::atomic::{AtomicBool, Ordering::SeqCst},
    thread,
};

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();

pub fn example10_sequantially_consistent_ordering() {
    let a = thread::spawn(|| {
        A.store(true, SeqCst);
        if !B.load(SeqCst) {
            unsafe { S.push('!') };
        }
    });

    let b = thread::spawn(|| {
        B.store(true, SeqCst);
        if !A.load(SeqCst) {
            unsafe { S.push('?') };
        }
    });

    a.join().unwrap();
    b.join().unwrap();
    println!("{}", unsafe { S.to_string() });
}

pub fn example10_sequantially_consistent_ordering_no_print() {
    let a = thread::spawn(|| {
        A.store(true, SeqCst);
        thread::sleep(std::time::Duration::from_millis(10)); // this line sleeps after A is set to true. While this thread is sleeping, B also sets B to true. So, there won't be any print.
        if !B.load(SeqCst) {
            unsafe { S.push('!') };
        }
    });

    let b = thread::spawn(|| {
        B.store(true, SeqCst);
        if !A.load(SeqCst) {
            unsafe { S.push('?') };
        }
    });

    a.join().unwrap();
    b.join().unwrap();
    println!("{}", unsafe { S.to_string() });
}
