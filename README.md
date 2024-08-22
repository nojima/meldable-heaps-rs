# skew-heap-rs

A skew heap implementation in Rust.

* **No unsafe code**
    * This implementation is written only in safe Rust. No `unsafe` are used.

* **Iterative implementation of meld**
    * `meld` is a core operation of skew heap, which merges two skew heaps. `meld` is usually implemented recursively, but such an implementation may cause stack overflow in the worst case. We implemented `meld` iteratively without using recursion in order to avoid stack overflow.
