use std::collections::BinaryHeap;

use divan::Bencher;
use meldable_heaps::{ParingHeap, SkewHeap};
use mimalloc::MiMalloc;
use rand::{rngs::SmallRng, Rng, SeedableRng};

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

/*
Benchmark results:

Timer precision: 10 ns
push_pop                    fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ random_push_pop_bench                  │               │               │               │         │
   ├─ BinaryHeap<[u64; 5]>                │               │               │               │         │
   │  ├─ 1000000            34.69 ms      │ 46.72 ms      │ 36.72 ms      │ 38.36 ms      │ 5       │ 5
   │  ├─ 2000000            72.33 ms      │ 93.88 ms      │ 77.55 ms      │ 79.21 ms      │ 5       │ 5
   │  ├─ 3000000            112.4 ms      │ 150.5 ms      │ 117.7 ms      │ 124.2 ms      │ 5       │ 5
   │  ├─ 4000000            146 ms        │ 182 ms        │ 154.7 ms      │ 157 ms        │ 5       │ 5
   │  ├─ 5000000            188.2 ms      │ 220.9 ms      │ 196 ms        │ 202.5 ms      │ 5       │ 5
   │  ├─ 6000000            224.8 ms      │ 321.1 ms      │ 242.7 ms      │ 254.9 ms      │ 5       │ 5
   │  ├─ 7000000            262.3 ms      │ 360.1 ms      │ 272.1 ms      │ 299.1 ms      │ 5       │ 5
   │  ╰─ 8000000            297.9 ms      │ 368.7 ms      │ 309.9 ms      │ 321.5 ms      │ 5       │ 5
   ├─ ParingHeap<[u64; 5]>                │               │               │               │         │
   │  ├─ 1000000            22.91 ms      │ 23.08 ms      │ 22.93 ms      │ 22.98 ms      │ 5       │ 5
   │  ├─ 2000000            49.07 ms      │ 60.65 ms      │ 51.28 ms      │ 52.4 ms       │ 5       │ 5
   │  ├─ 3000000            66.37 ms      │ 74.4 ms       │ 68.73 ms      │ 69.72 ms      │ 5       │ 5
   │  ├─ 4000000            92.76 ms      │ 98.41 ms      │ 94.74 ms      │ 95.3 ms       │ 5       │ 5
   │  ├─ 5000000            116.4 ms      │ 123.2 ms      │ 121 ms        │ 120.3 ms      │ 5       │ 5
   │  ├─ 6000000            138.5 ms      │ 151 ms        │ 144.4 ms      │ 144.5 ms      │ 5       │ 5
   │  ├─ 7000000            162.5 ms      │ 176.2 ms      │ 165.3 ms      │ 167.7 ms      │ 5       │ 5
   │  ╰─ 8000000            180 ms        │ 218.4 ms      │ 190.4 ms      │ 196.3 ms      │ 5       │ 5
   ╰─ SkewHeap<[u64; 5]>                  │               │               │               │         │
      ├─ 1000000            34.94 ms      │ 41.02 ms      │ 37.7 ms       │ 38.26 ms      │ 5       │ 5
      ├─ 2000000            62.8 ms       │ 75.32 ms      │ 68.49 ms      │ 69.75 ms      │ 5       │ 5
      ├─ 3000000            99.49 ms      │ 115 ms        │ 100.7 ms      │ 105.3 ms      │ 5       │ 5
      ├─ 4000000            131.6 ms      │ 165 ms        │ 142.5 ms      │ 147.5 ms      │ 5       │ 5
      ├─ 5000000            171.2 ms      │ 200.9 ms      │ 175.8 ms      │ 179.1 ms      │ 5       │ 5
      ├─ 6000000            207 ms        │ 223.3 ms      │ 215.1 ms      │ 214 ms        │ 5       │ 5
      ├─ 7000000            235 ms        │ 261.3 ms      │ 248.4 ms      │ 247.4 ms      │ 5       │ 5
      ╰─ 8000000            263.1 ms      │ 312.6 ms      │ 285.8 ms      │ 287.8 ms      │ 5       │ 5
*/
#[divan::bench(
    types = [BinaryHeap<Entry>, ParingHeap<Entry>, SkewHeap<Entry>],
    args = [1000000, 2000000, 3000000, 4000000, 5000000, 6000000, 7000000, 8000000],
    sample_count = 5,
)]
fn random_push_pop_bench<H: Heap<Entry>>(bencher: Bencher, n: u64) {
    let mut heap = H::new();
    bencher.bench_local(|| random_push_pop(n, &mut heap));
}

fn random_push_pop<H: Heap<Entry>>(n: u64, heap: &mut H) {
    let mut rng = SmallRng::seed_from_u64(2635249153387078803);
    for _ in 0..n {
        if rng.gen::<u8>() < 200 {
            heap.push(rng.gen::<Entry>());
        } else {
            divan::black_box(heap.pop());
        }
    }
}
