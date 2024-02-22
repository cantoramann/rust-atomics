### Examples 1 - 5 (up to page 7)

> So far we've looked at (1) transferring ownership of a value to a thread using a move closure and (2) borrowing data from longer-living parent threads. When sharing data between two threads where neither thread is guaranteed to outlive the other, neither of them can be the owner of that data. Any data shared between them will need to live as long as the longest living thread.

### Examples 6, 7 (pages 7 - 10)

> There are several ways to create something that's not owned by a single thread.

1. Statics: Not `'static`. These are like macros, owned by the program, getting init before the program starts and lives throughout.
2. Leaking: `let x: &'static [i32; 3] = Box::leak(Box::new([1,2,3]));`. Now, the `Box` will live forever, without an owner, allowing it to be borrowed by any other thread as long as the program runs. Yet, this is basically leaking memory. Much of it would cause RAM issues.
3. Reference counters, `std::rd::Rc` and `std::sync::Arc`. The difference is that `Rc` is not thread safe, it only can be cloned by different variables and would be fine, such as `let a = Arc::new([1,2,3]) && let b = a.clone()`. However, for thread-safety, use `Arc` , since it's implemented with atomicity considerations.

So, the following will be possible with `Arc`:

```rust
let a = Arc::new([1,2,3]);
let b = a.clone() // So far Rc would also work.

thread::spawn(move || dbg!(a));
thread::spawn(move || dbg!(b));
```

### Example 8 (pages 10 - 12)

How `unsafe` can interfere the optimized binary in highly unexpected ways if not used rigorously.
