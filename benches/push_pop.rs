use std::collections::BinaryHeap;

use divan::Bencher;
use meldable_heaps::{LeftistHeap, ParingHeap, SkewHeap};
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

#[rustfmt::skip]
impl<T: Ord> Heap<T> for LeftistHeap<T> {
    fn new() -> Self { Self::new() }
    fn push(&mut self, value: T) { self.push(value) }
    fn pop(&mut self) -> Option<T> { self.pop() }
    fn is_empty(&self) -> bool { self.is_empty() }
}

type Entry = [u64; 5];

/*
Benchmark results:

push_pop_bench                          │               │               │               │         │
├─ BinaryHeap<[u64; 5]>                 │               │               │               │         │
│  ├─ 1000000             188.1 ms      │ 214.2 ms      │ 192.6 ms      │ 198.3 ms      │ 3       │ 3
│  ├─ 2000000             527.2 ms      │ 557.2 ms      │ 555.4 ms      │ 546.6 ms      │ 3       │ 3
│  ├─ 3000000             994.4 ms      │ 1.017 s       │ 995.7 ms      │ 1.002 s       │ 3       │ 3
│  ├─ 4000000             1.445 s       │ 1.49 s        │ 1.487 s       │ 1.474 s       │ 3       │ 3
│  ├─ 5000000             1.983 s       │ 2.016 s       │ 2.015 s       │ 2.005 s       │ 3       │ 3
│  ├─ 6000000             2.5 s         │ 2.617 s       │ 2.53 s        │ 2.549 s       │ 3       │ 3
│  ├─ 7000000             3.086 s       │ 3.151 s       │ 3.119 s       │ 3.119 s       │ 3       │ 3
│  ╰─ 8000000             3.696 s       │ 3.805 s       │ 3.716 s       │ 3.739 s       │ 3       │ 3
├─ LeftistHeap<[u64; 5]>                │               │               │               │         │
│  ├─ 1000000             305.1 ms      │ 311.9 ms      │ 311.4 ms      │ 309.5 ms      │ 3       │ 3
│  ├─ 2000000             768.4 ms      │ 789.8 ms      │ 784.8 ms      │ 781 ms        │ 3       │ 3
│  ├─ 3000000             1.302 s       │ 1.34 s        │ 1.304 s       │ 1.316 s       │ 3       │ 3
│  ├─ 4000000             1.894 s       │ 1.898 s       │ 1.898 s       │ 1.897 s       │ 3       │ 3
│  ├─ 5000000             2.494 s       │ 2.539 s       │ 2.5 s         │ 2.511 s       │ 3       │ 3
│  ├─ 6000000             3.149 s       │ 3.192 s       │ 3.184 s       │ 3.175 s       │ 3       │ 3
│  ├─ 7000000             3.79 s        │ 3.945 s       │ 3.859 s       │ 3.865 s       │ 3       │ 3
│  ╰─ 8000000             4.522 s       │ 4.562 s       │ 4.534 s       │ 4.539 s       │ 3       │ 3
├─ ParingHeap<[u64; 5]>                 │               │               │               │         │
│  ├─ 1000000             330.6 ms      │ 340.6 ms      │ 336.6 ms      │ 335.9 ms      │ 3       │ 3
│  ├─ 2000000             926.7 ms      │ 947.2 ms      │ 944.6 ms      │ 939.5 ms      │ 3       │ 3
│  ├─ 3000000             1.627 s       │ 1.658 s       │ 1.628 s       │ 1.638 s       │ 3       │ 3
│  ├─ 4000000             2.395 s       │ 2.412 s       │ 2.408 s       │ 2.405 s       │ 3       │ 3
│  ├─ 5000000             3.219 s       │ 3.282 s       │ 3.265 s       │ 3.255 s       │ 3       │ 3
│  ├─ 6000000             4.107 s       │ 4.182 s       │ 4.15 s        │ 4.147 s       │ 3       │ 3
│  ├─ 7000000             4.983 s       │ 5.13 s        │ 5.028 s       │ 5.047 s       │ 3       │ 3
│  ╰─ 8000000             6.094 s       │ 6.218 s       │ 6.156 s       │ 6.156 s       │ 3       │ 3
╰─ SkewHeap<[u64; 5]>                   │               │               │               │         │
   ├─ 1000000             472.3 ms      │ 477.2 ms      │ 473.5 ms      │ 474.4 ms      │ 3       │ 3
   ├─ 2000000             1.26 s        │ 1.311 s       │ 1.295 s       │ 1.288 s       │ 3       │ 3
   ├─ 3000000             2.212 s       │ 2.232 s       │ 2.227 s       │ 2.224 s       │ 3       │ 3
   ├─ 4000000             3.292 s       │ 3.338 s       │ 3.307 s       │ 3.312 s       │ 3       │ 3
   ├─ 5000000             4.327 s       │ 4.454 s       │ 4.376 s       │ 4.386 s       │ 3       │ 3
   ├─ 6000000             5.805 s       │ 6.279 s       │ 5.832 s       │ 5.972 s       │ 3       │ 3
   ├─ 7000000             6.767 s       │ 7.14 s        │ 6.784 s       │ 6.897 s       │ 3       │ 3
   ╰─ 8000000             7.94 s        │ 8.305 s       │ 7.993 s       │ 8.079 s       │ 3       │ 3
*/
#[divan::bench(
    types = [BinaryHeap<Entry>, ParingHeap<Entry>, SkewHeap<Entry>, LeftistHeap<Entry>],
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

random_push_pop_bench                   │               │               │               │         │
├─ BinaryHeap<[u64; 5]>                 │               │               │               │         │
│  ├─ 1000000             32.63 ms      │ 46.66 ms      │ 36.42 ms      │ 37.66 ms      │ 5       │ 5
│  ├─ 2000000             67.71 ms      │ 86.06 ms      │ 72.56 ms      │ 74.41 ms      │ 5       │ 5
│  ├─ 3000000             108 ms        │ 145.8 ms      │ 113.7 ms      │ 119.7 ms      │ 5       │ 5
│  ├─ 4000000             146.7 ms      │ 176.1 ms      │ 152.6 ms      │ 156.5 ms      │ 5       │ 5
│  ├─ 5000000             188.7 ms      │ 218.9 ms      │ 201.9 ms      │ 200.9 ms      │ 5       │ 5
│  ├─ 6000000             220.3 ms      │ 305.3 ms      │ 233.5 ms      │ 246.8 ms      │ 5       │ 5
│  ├─ 7000000             257.4 ms      │ 334.9 ms      │ 274 ms        │ 284.1 ms      │ 5       │ 5
│  ╰─ 8000000             307.5 ms      │ 400 ms        │ 318.2 ms      │ 331.2 ms      │ 5       │ 5
├─ LeftistHeap<[u64; 5]>                │               │               │               │         │
│  ├─ 1000000             30.59 ms      │ 31.52 ms      │ 31.26 ms      │ 31.2 ms       │ 5       │ 5
│  ├─ 2000000             61.23 ms      │ 63.37 ms      │ 62.73 ms      │ 62.52 ms      │ 5       │ 5
│  ├─ 3000000             91.64 ms      │ 96.05 ms      │ 93.44 ms      │ 93.53 ms      │ 5       │ 5
│  ├─ 4000000             125.8 ms      │ 139.7 ms      │ 131.5 ms      │ 131.8 ms      │ 5       │ 5
│  ├─ 5000000             153.1 ms      │ 169.7 ms      │ 166.7 ms      │ 162.9 ms      │ 5       │ 5
│  ├─ 6000000             184.3 ms      │ 211.2 ms      │ 195.9 ms      │ 198.9 ms      │ 5       │ 5
│  ├─ 7000000             227.4 ms      │ 251 ms        │ 242.7 ms      │ 239.4 ms      │ 5       │ 5
│  ╰─ 8000000             256.4 ms      │ 289.9 ms      │ 273.2 ms      │ 273.5 ms      │ 5       │ 5
├─ ParingHeap<[u64; 5]>                 │               │               │               │         │
│  ├─ 1000000             23.65 ms      │ 28.91 ms      │ 28.39 ms      │ 27.51 ms      │ 5       │ 5
│  ├─ 2000000             47.12 ms      │ 56.89 ms      │ 52.25 ms      │ 52.54 ms      │ 5       │ 5
│  ├─ 3000000             74.35 ms      │ 88.75 ms      │ 81.01 ms      │ 81.35 ms      │ 5       │ 5
│  ├─ 4000000             95.74 ms      │ 115.4 ms      │ 102.8 ms      │ 104.9 ms      │ 5       │ 5
│  ├─ 5000000             117.4 ms      │ 145.6 ms      │ 135.4 ms      │ 133.9 ms      │ 5       │ 5
│  ├─ 6000000             145.6 ms      │ 172.4 ms      │ 147.6 ms      │ 152.1 ms      │ 5       │ 5
│  ├─ 7000000             175.2 ms      │ 241 ms        │ 195.3 ms      │ 201.6 ms      │ 5       │ 5
│  ╰─ 8000000             190.5 ms      │ 229.2 ms      │ 207.2 ms      │ 209.4 ms      │ 5       │ 5
╰─ SkewHeap<[u64; 5]>                   │               │               │               │         │
   ├─ 1000000             33.67 ms      │ 38.21 ms      │ 37.67 ms      │ 36.98 ms      │ 5       │ 5
   ├─ 2000000             64.52 ms      │ 76.14 ms      │ 70.64 ms      │ 71.08 ms      │ 5       │ 5
   ├─ 3000000             101 ms        │ 116.1 ms      │ 104.4 ms      │ 107.1 ms      │ 5       │ 5
   ├─ 4000000             130.9 ms      │ 164.9 ms      │ 136.3 ms      │ 142.1 ms      │ 5       │ 5
   ├─ 5000000             168.9 ms      │ 203.2 ms      │ 169.6 ms      │ 176.7 ms      │ 5       │ 5
   ├─ 6000000             200 ms        │ 228.8 ms      │ 206.5 ms      │ 210.9 ms      │ 5       │ 5
   ├─ 7000000             233.9 ms      │ 259.5 ms      │ 239 ms        │ 244.7 ms      │ 5       │ 5
   ╰─ 8000000             266.6 ms      │ 307 ms        │ 291.1 ms      │ 288 ms        │ 5       │ 5
*/
#[divan::bench(
    types = [BinaryHeap<Entry>, ParingHeap<Entry>, SkewHeap<Entry>, LeftistHeap<Entry>],
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
