use skew_heap::SkewHeap;
use std::collections::BinaryHeap;

fn main() {
    divan::main();
}

trait Heap<T: Ord> {
    fn new() -> Self;
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn is_empty(&self) -> bool;
}

impl<T: Ord> Heap<T> for BinaryHeap<T> {
    fn new() -> Self {
        BinaryHeap::new()
    }

    fn push(&mut self, value: T) {
        self.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.pop()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T: Ord> Heap<T> for skew_heap::SkewHeap<T> {
    fn new() -> Self {
        skew_heap::SkewHeap::new()
    }

    fn push(&mut self, value: T) {
        self.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.pop()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

type Entry = [u64; 5];

/*
Benchmark results:

Timer precision: 20 ns
skew_heap                   fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ push_pop_bench                         │               │               │               │         │
   ├─ BinaryHeap<[u64; 5]>                │               │               │               │         │
   │  ├─ 100000             15.95 ms      │ 19.27 ms      │ 16.19 ms      │ 16.42 ms      │ 20      │ 20
   │  ├─ 200000             37.87 ms      │ 49.84 ms      │ 39.71 ms      │ 41 ms         │ 20      │ 20
   │  ├─ 300000             63.76 ms      │ 99.69 ms      │ 73.67 ms      │ 73.87 ms      │ 20      │ 20
   │  ├─ 400000             94.08 ms      │ 117.7 ms      │ 101.5 ms      │ 103.5 ms      │ 20      │ 20
   │  ├─ 500000             129.9 ms      │ 171.4 ms      │ 137.7 ms      │ 142.4 ms      │ 20      │ 20
   │  ├─ 600000             180.6 ms      │ 328.4 ms      │ 240.1 ms      │ 240.9 ms      │ 20      │ 20
   │  ├─ 700000             257.5 ms      │ 369.5 ms      │ 289 ms        │ 292.8 ms      │ 20      │ 20
   │  ╰─ 800000             314.8 ms      │ 422.1 ms      │ 364.4 ms      │ 359.6 ms      │ 20      │ 20
   ╰─ SkewHeap<[u64; 5]>                  │               │               │               │         │
      ├─ 100000             57.09 ms      │ 78.17 ms      │ 65.52 ms      │ 66.26 ms      │ 20      │ 20
      ├─ 200000             156.9 ms      │ 237.4 ms      │ 173.1 ms      │ 182.2 ms      │ 20      │ 20
      ├─ 300000             278.8 ms      │ 381.1 ms      │ 308.7 ms      │ 318.2 ms      │ 20      │ 20
      ├─ 400000             438 ms        │ 572.9 ms      │ 471.2 ms      │ 482.6 ms      │ 20      │ 20
      ├─ 500000             593.5 ms      │ 763.5 ms      │ 642.5 ms      │ 650.6 ms      │ 20      │ 20
      ├─ 600000             806 ms        │ 968.8 ms      │ 847.7 ms      │ 848.8 ms      │ 20      │ 20
      ├─ 700000             1.055 s       │ 1.248 s       │ 1.117 s       │ 1.135 s       │ 20      │ 20
      ╰─ 800000             1.248 s       │ 1.682 s       │ 1.441 s       │ 1.443 s       │ 20      │ 20
*/
#[divan::bench(
    types = [BinaryHeap<Entry>, SkewHeap<Entry>],
    args = [100000, 200000, 300000, 400000, 500000, 600000, 700000, 800000],
    sample_count = 20,
)]
fn push_pop_bench<H: Heap<Entry>>(n: u64) {
    divan::black_box(push_pop(n, H::new()));
}

fn push_pop<H: Heap<Entry>>(n: u64, mut heap: H) {
    for _ in 0..n {
        heap.push(rand::random::<Entry>());
    }
    while !heap.is_empty() {
        let x = heap.pop();
        divan::black_box(x);
    }
}
