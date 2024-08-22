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
   │  ├─ 1000000            222 ms        │ 238.2 ms      │ 235 ms        │ 231.8 ms      │ 3       │ 3
   │  ├─ 2000000            616.4 ms      │ 645.5 ms      │ 620.5 ms      │ 627.5 ms      │ 3       │ 3
   │  ├─ 3000000            1.129 s       │ 1.177 s       │ 1.159 s       │ 1.155 s       │ 3       │ 3
   │  ├─ 4000000            1.726 s       │ 1.806 s       │ 1.765 s       │ 1.766 s       │ 3       │ 3
   │  ├─ 5000000            2.478 s       │ 2.503 s       │ 2.478 s       │ 2.486 s       │ 3       │ 3
   │  ├─ 6000000            3.125 s       │ 3.18 s        │ 3.131 s       │ 3.145 s       │ 3       │ 3
   │  ├─ 7000000            3.887 s       │ 3.908 s       │ 3.891 s       │ 3.896 s       │ 3       │ 3
   │  ╰─ 8000000            4.563 s       │ 4.598 s       │ 4.597 s       │ 4.586 s       │ 3       │ 3
   ╰─ SkewHeap<[u64; 5]>                  │               │               │               │         │
      ├─ 1000000            793.4 ms      │ 840.3 ms      │ 804.8 ms      │ 812.8 ms      │ 3       │ 3
      ├─ 2000000            2.124 s       │ 2.232 s       │ 2.194 s       │ 2.183 s       │ 3       │ 3
      ├─ 3000000            3.842 s       │ 3.988 s       │ 3.959 s       │ 3.929 s       │ 3       │ 3
      ├─ 4000000            5.577 s       │ 5.708 s       │ 5.677 s       │ 5.654 s       │ 3       │ 3
      ├─ 5000000            7.402 s       │ 7.549 s       │ 7.486 s       │ 7.479 s       │ 3       │ 3
      ├─ 6000000            9.103 s       │ 9.444 s       │ 9.295 s       │ 9.281 s       │ 3       │ 3
      ├─ 7000000            11.34 s       │ 11.36 s       │ 11.35 s       │ 11.35 s       │ 3       │ 3
      ╰─ 8000000            13.09 s       │ 13.37 s       │ 13.26 s       │ 13.24 s       │ 3       │ 3
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
