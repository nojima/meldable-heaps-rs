use std::collections::BinaryHeap;

use divan::Bencher;
use mimalloc::MiMalloc;
use meldable_heaps::SkewHeap;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

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

impl<T: Ord> Heap<T> for SkewHeap<T> {
    fn new() -> Self {
        SkewHeap::new()
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

Timer precision: 10 ns
skew_heap                   fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ push_pop_bench                         │               │               │               │         │
   ├─ BinaryHeap<[u64; 5]>                │               │               │               │         │
   │  ├─ 1000000            188.5 ms      │ 225.4 ms      │ 192.8 ms      │ 202.2 ms      │ 3       │ 3
   │  ├─ 2000000            556 ms        │ 563.4 ms      │ 558.7 ms      │ 559.4 ms      │ 3       │ 3
   │  ├─ 3000000            1.072 s       │ 1.16 s        │ 1.13 s        │ 1.121 s       │ 3       │ 3
   │  ├─ 4000000            1.674 s       │ 1.734 s       │ 1.729 s       │ 1.712 s       │ 3       │ 3
   │  ├─ 5000000            1.993 s       │ 2.266 s       │ 2.091 s       │ 2.117 s       │ 3       │ 3
   │  ├─ 6000000            2.532 s       │ 2.612 s       │ 2.58 s        │ 2.574 s       │ 3       │ 3
   │  ├─ 7000000            3.117 s       │ 3.178 s       │ 3.132 s       │ 3.142 s       │ 3       │ 3
   │  ╰─ 8000000            3.721 s       │ 3.792 s       │ 3.749 s       │ 3.754 s       │ 3       │ 3
   ╰─ SkewHeap<[u64; 5]>                  │               │               │               │         │
      ├─ 1000000            449.7 ms      │ 450.1 ms      │ 449.8 ms      │ 449.9 ms      │ 3       │ 3
      ├─ 2000000            1.289 s       │ 1.397 s       │ 1.299 s       │ 1.328 s       │ 3       │ 3
      ├─ 3000000            2.327 s       │ 2.364 s       │ 2.344 s       │ 2.345 s       │ 3       │ 3
      ├─ 4000000            3.331 s       │ 3.404 s       │ 3.347 s       │ 3.361 s       │ 3       │ 3
      ├─ 5000000            4.416 s       │ 4.504 s       │ 4.428 s       │ 4.449 s       │ 3       │ 3
      ├─ 6000000            5.668 s       │ 5.75 s        │ 5.675 s       │ 5.698 s       │ 3       │ 3
      ├─ 7000000            6.812 s       │ 7.09 s        │ 6.886 s       │ 6.929 s       │ 3       │ 3
      ╰─ 8000000            8.034 s       │ 8.145 s       │ 8.051 s       │ 8.077 s       │ 3       │ 3
*/
#[divan::bench(
    types = [BinaryHeap<Entry>, SkewHeap<Entry>],
    args = [1000000, 2000000, 3000000, 4000000, 5000000, 6000000, 7000000, 8000000],
    sample_count = 3,
)]
fn push_pop_bench<H: Heap<Entry>>(bencher: Bencher, n: u64) {
    let mut heap = H::new();
    bencher.bench_local(|| push_pop(n, &mut heap));
}

fn push_pop<H: Heap<Entry>>(n: u64, heap: &mut H) {
    for _ in 0..n {
        heap.push(rand::random::<Entry>());
    }
    while !heap.is_empty() {
        let x = heap.pop();
        divan::black_box(x);
    }
}
