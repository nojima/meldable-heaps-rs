# meldable-heaps

A meldable heap implementation in Rust.

* **Iterative implementation of meld**
    * `meld` is a core operation of meldable heaps, which merges two heaps into a heap. `meld` is usually implemented recursively, but such an implementation may cause stack overflow in the worst case. We implemented `meld` iteratively without using recursion in order to avoid stack overflow.

* **No unsafe code**
    * This implementation is written only in safe Rust. No `unsafe` are used.

* **No dependencies**
    * This crate does not depend on any other crates expect for development dependencies. This crate even does not depend on `std`. Therefore, this crate can be used in `no_std` environments.

## Heap Comparison

The three heaps implemented in this crate have the following properties. The abbreviation am. indicates that the given complexity is amortized.

|             | push         | pop          | meld          | per-node overhead
|-------------|--------------|--------------|---------------|--------------------
| PairingHeap | O(1)         | O(log n) am. | O(1)          | 2 pointers
| LeftistHeap | O(log n)     | O(log n)     | O(log n)      | 2 pointers + 1 byte
| SkewHeap    | O(log n) am. | O(log n) am. | O(log n) am.  | 2 pointers

The pairing heap is said to be the fastest meldable heap in practice. If you need a heap that is fast on average, pairing heap is a good choice.

The leftist heap guarantees `pop` in O(log n) time in the worst case. If worst-case latency is important, the leftist heap is a good choice.

## Allocator

Node-based heaps perform a lot of memory allocations. Therefore, the allocator has a significant impact on  the performance. According to our benchmarks, [mimalloc](https://docs.rs/mimalloc/latest/mimalloc/) greatly improves the speed of meldable heaps. Therefore, we recommend using this crate with mimalloc.
