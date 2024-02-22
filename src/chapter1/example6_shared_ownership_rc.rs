use std::{rc::Rc, thread};

pub fn example6_shared_ownership_rc() {
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr()); // This passed. They are the same object with a reference counter.

    // thread::spawn(move || dbg!(b)); will fail - Rc is not thread safe
}
