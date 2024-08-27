use std::collections::BinaryHeap;

use divan::Bencher;
use meldable_heaps::{ParingHeap, SkewHeap};
use mimalloc::MiMalloc;

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

#[rustfmt::skip]
impl<T: Ord> Heap<T> for BinaryHeap<T> {
    fn new() -> Self { Self::new() }
    fn push(&mut self, value: T) { self.push(value) }
    fn pop(&mut self) -> Option<T> { self.pop() }
    fn is_empty(&self) -> bool { self.is_empty() }
}

#[rustfmt::skip]
impl<T: Ord> Heap<T> for ParingHeap<T> {
    fn new() -> Self { Self::new() }
    fn push(&mut self, value: T) { self.push(value) }
    fn pop(&mut self) -> Option<T> { self.pop() }
    fn is_empty(&self) -> bool { self.is_empty() }
}

#[rustfmt::skip]
impl<T: Ord> Heap<T> for SkewHeap<T> {
    fn new() -> Self { Self::new() }
    fn push(&mut self, value: T) { self.push(value) }
    fn pop(&mut self) -> Option<T> { self.pop() }
    fn is_empty(&self) -> bool { self.is_empty() }
}

type Entry = [u64; 5];

/*
Benchmark results:

Timer precision: 10 ns
push_pop                    fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ push_pop_bench                         │               │               │               │         │
   ├─ BinaryHeap<[u64; 5]>                │               │               │               │         │
   │  ├─ 1000000            219.2 ms      │ 247 ms        │ 238.5 ms      │ 234.9 ms      │ 3       │ 3
   │  ├─ 2000000            551.6 ms      │ 581 ms        │ 558.2 ms      │ 563.6 ms      │ 3       │ 3
   │  ├─ 3000000            1.025 s       │ 1.128 s       │ 1.071 s       │ 1.074 s       │ 3       │ 3
   │  ├─ 4000000            1.504 s       │ 1.532 s       │ 1.532 s       │ 1.523 s       │ 3       │ 3
   │  ├─ 5000000            2.036 s       │ 2.136 s       │ 2.044 s       │ 2.072 s       │ 3       │ 3
   │  ├─ 6000000            2.62 s        │ 2.678 s       │ 2.65 s        │ 2.649 s       │ 3       │ 3
   │  ├─ 7000000            3.16 s        │ 3.242 s       │ 3.222 s       │ 3.208 s       │ 3       │ 3
   │  ╰─ 8000000            3.807 s       │ 3.851 s       │ 3.841 s       │ 3.833 s       │ 3       │ 3
   ├─ ParingHeap<[u64; 5]>                │               │               │               │         │
   │  ├─ 1000000            343.4 ms      │ 347.6 ms      │ 345.2 ms      │ 345.4 ms      │ 3       │ 3
   │  ├─ 2000000            973.3 ms      │ 1.002 s       │ 991.1 ms      │ 989 ms        │ 3       │ 3
   │  ├─ 3000000            1.725 s       │ 1.733 s       │ 1.728 s       │ 1.728 s       │ 3       │ 3
   │  ├─ 4000000            2.519 s       │ 2.533 s       │ 2.531 s       │ 2.527 s       │ 3       │ 3
   │  ├─ 5000000            3.323 s       │ 3.405 s       │ 3.354 s       │ 3.361 s       │ 3       │ 3
   │  ├─ 6000000            4.248 s       │ 4.332 s       │ 4.263 s       │ 4.281 s       │ 3       │ 3
   │  ├─ 7000000            5.182 s       │ 5.359 s       │ 5.29 s        │ 5.277 s       │ 3       │ 3
   │  ╰─ 8000000            6.235 s       │ 6.248 s       │ 6.236 s       │ 6.24 s        │ 3       │ 3
   ╰─ SkewHeap<[u64; 5]>                  │               │               │               │         │
      ├─ 1000000            496.4 ms      │ 526.6 ms      │ 499.6 ms      │ 507.5 ms      │ 3       │ 3
      ├─ 2000000            1.404 s       │ 1.43 s        │ 1.422 s       │ 1.419 s       │ 3       │ 3
      ├─ 3000000            2.345 s       │ 2.425 s       │ 2.386 s       │ 2.385 s       │ 3       │ 3
      ├─ 4000000            3.402 s       │ 3.432 s       │ 3.404 s       │ 3.413 s       │ 3       │ 3
      ├─ 5000000            4.568 s       │ 4.58 s        │ 4.571 s       │ 4.573 s       │ 3       │ 3
      ├─ 6000000            5.681 s       │ 5.82 s        │ 5.693 s       │ 5.731 s       │ 3       │ 3
      ├─ 7000000            6.931 s       │ 7.012 s       │ 6.955 s       │ 6.966 s       │ 3       │ 3
      ╰─ 8000000            8.246 s       │ 8.358 s       │ 8.282 s       │ 8.295 s       │ 3       │ 3
*/
#[divan::bench(
    types = [BinaryHeap<Entry>, ParingHeap<Entry>, SkewHeap<Entry>],
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
