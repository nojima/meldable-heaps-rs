# skew-heap-rs

A skew heap implementation in Rust.

* **No unsafe code**
    * This implementation is written only in safe Rust. No `unsafe` are used.

* **Iterative implementation of meld**
    * `meld` is a core operation of skew heap, which merges two skew heaps. `meld` is usually implemented recursively, but such an implementation may cause stack overflow in the worst case. We implemented `meld` iteratively without using recursion in order to avoid stack overflow.

## Allocator

SkewHeap performs a lot of memory allocations. Therefore, the allocator has a significant impact on  the performance. According to our benchmarks, [mimalloc](https://docs.rs/mimalloc/latest/mimalloc/) greatly improves the speed of SkewHeap. Therefore, we recommend using this crate with mimalloc.
