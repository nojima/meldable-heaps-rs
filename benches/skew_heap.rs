use divan::Bencher;
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

Timer precision: 10 ns
skew_heap                   fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ push_pop_bench                         │               │               │               │         │
   ├─ BinaryHeap<[u64; 5]>                │               │               │               │         │
   │  ├─ 100000             11.82 ms      │ 13.25 ms      │ 12.14 ms      │ 12.18 ms      │ 20      │ 20
   │  ├─ 200000             26.98 ms      │ 36.36 ms      │ 27.14 ms      │ 28.2 ms       │ 20      │ 20
   │  ├─ 300000             42.91 ms      │ 51.76 ms      │ 43.09 ms      │ 44.17 ms      │ 20      │ 20
   │  ├─ 400000             61.6 ms       │ 87.01 ms      │ 65.19 ms      │ 67.37 ms      │ 20      │ 20
   │  ├─ 500000             81.55 ms      │ 111.3 ms      │ 89.29 ms      │ 91.71 ms      │ 20      │ 20
   │  ├─ 600000             102.3 ms      │ 136.3 ms      │ 110.7 ms      │ 112.9 ms      │ 20      │ 20
   │  ├─ 700000             123.1 ms      │ 139.7 ms      │ 127.7 ms      │ 129.2 ms      │ 20      │ 20
   │  ╰─ 800000             148.6 ms      │ 215.2 ms      │ 155.9 ms      │ 161.7 ms      │ 20      │ 20
   ╰─ SkewHeap<[u64; 5]>                  │               │               │               │         │
      ├─ 100000             28.4 ms       │ 33.02 ms      │ 29.05 ms      │ 29.22 ms      │ 20      │ 20
      ├─ 200000             68.43 ms      │ 93.57 ms      │ 69.36 ms      │ 71.66 ms      │ 20      │ 20
      ├─ 300000             110.9 ms      │ 136.9 ms      │ 114.3 ms      │ 118.8 ms      │ 20      │ 20
      ├─ 400000             171.7 ms      │ 229.7 ms      │ 182.2 ms      │ 189.7 ms      │ 20      │ 20
      ├─ 500000             246.2 ms      │ 296.1 ms      │ 261.5 ms      │ 264.2 ms      │ 20      │ 20
      ├─ 600000             340.5 ms      │ 386.5 ms      │ 362.4 ms      │ 363.4 ms      │ 20      │ 20
      ├─ 700000             436.7 ms      │ 509.4 ms      │ 452.5 ms      │ 458.2 ms      │ 20      │ 20
      ╰─ 800000             549.8 ms      │ 640.1 ms      │ 562.2 ms      │ 572.6 ms      │ 20      │ 20
*/
#[divan::bench(
    types = [BinaryHeap<Entry>, SkewHeap<Entry>],
    args = [100000, 200000, 300000, 400000, 500000, 600000, 700000, 800000],
    sample_count = 20,
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
