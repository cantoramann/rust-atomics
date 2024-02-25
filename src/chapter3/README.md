### Exanple 1 (Relaxed Ordering, pg 54)

Idea is that all the threads will share the same order. In the print statement, each vector usually has the same elements, but they rarely vary, and this becomes more obvious. In `Relaxed` ordering, threads always have a global view of access. The order may change from one run to another, but for each program threads will share the view on atomic udpates. This is `Relaxed`.

--
... add the examples in between
--

### Example 5 (Out-of-Thin-Air Values)

In such a cycle, not practically in Rust, but theoratically, you can get a value other than `0`, such as, as the book says, `37`. This is something theoratically being worked on, at the time of the release.

### Example 8

Around 1% of the threads fail at `compare_exchange`, but the reason is that they get into the function `f` in distant clock times, as the image shows. At least seeing some threads with a failed operation hint that the lock mechanism works as expected.

The reason we use `Relaxed` in the `failure` state of `compare_exchange` is that we cannot use `Release`, as we haven't even acquired yet - and we don't really care about the failure. Yet, we still need to put an ordering, and the most relaxed one is, naturally, `Relaxed`.
