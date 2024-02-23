### Exanple 1 (Relaxed Ordering, pg 54)

Idea is that all the threads will share the same order. In the print statement, each vector usually has the same elements, but they rarely vary, and this becomes more obvious. In `Relaxed` ordering, threads always have a global view of access. The order may change from one run to another, but for each program threads will share the view on atomic udpates. This is `Relaxed`.

### Example 2 (Out-of-Thin-Air Values)

In such a cycle, not practically in Rust, but theoratically, you can get a value other than `0`, such as, as the book says, `37`. This is something theoratically being worked on, at the time of the release.
