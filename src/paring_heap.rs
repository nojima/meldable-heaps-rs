#![forbid(unsafe_code)]

use alloc::{boxed::Box, vec, vec::Vec};

pub struct ParingHeap<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> ParingHeap<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn push(&mut self, value: T) {
        self.root = Node::meld(self.root.take(), Node::singleton(value))
    }

    pub fn pop(&mut self) -> Option<T> {
        let root = self.root.take()?;
        let value = root.value;
        self.root = Node::meld_siblings(root.first_child);
        Some(value)
    }

    pub fn peek(&self) -> Option<&T> {
        self.root.as_ref().map(|node| &node.value)
    }

    pub fn meld(mut heap1: ParingHeap<T>, mut heap2: ParingHeap<T>) -> ParingHeap<T> {
        let root = Node::meld(heap1.root.take(), heap2.root.take());
        Self { root }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let mut stack = Vec::new();
        if let Some(ref node) = self.root {
            stack.push(node.as_ref());
        }
        Iter { stack }
    }
}

// We need to implement `drop` for ParingHeap because auto-generated `drop` would cause stack overflow
// (the depth of the tree can be O(n) in the worst case).
impl<T: Ord> Drop for ParingHeap<T> {
    fn drop(&mut self) {
        let Some(root) = self.root.take() else { return };
        let mut stack = vec![root];
        while let Some(mut node) = stack.pop() {
            let mut it = node.first_child.take();
            while let Some(mut child) = it {
                it = child.next_sibling.take();
                stack.push(child);
            }
            drop(node);
        }
    }
}

struct Node<T: Ord> {
    value: T,
    first_child: Option<Box<Node<T>>>,
    next_sibling: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn singleton(value: T) -> Option<Box<Node<T>>> {
        Some(Box::new(Self {
            value,
            first_child: None,
            next_sibling: None,
        }))
    }

    fn meld(root1: Option<Box<Node<T>>>, root2: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        let (root1, root2) = match (root1, root2) {
            (None, root2) => return root2,
            (root1, None) => return root1,
            (Some(r1), Some(r2)) => (r1, r2),
        };
        Some(Self::xmeld(root1, root2))
    }

    fn xmeld(mut root1: Box<Node<T>>, mut root2: Box<Node<T>>) -> Box<Node<T>> {
        // Ensure root1 <= root2
        if root1.value > root2.value {
            core::mem::swap(&mut root1, &mut root2);
        }

        // Connect root2 as the first child of root1
        root2.next_sibling = root1.first_child;
        root1.first_child = Some(root2);

        root1
    }

    fn meld_siblings(heaps: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        let mut it = heaps;
        let mut stack = None;
        // For each iteration, take 2 heaps from `heaps` and meld them.
        while let Some(mut heap1) = it {
            it = heap1.next_sibling.take();
            let mut melt = if let Some(mut heap2) = it {
                it = heap2.next_sibling.take();
                Self::xmeld(heap1, heap2)
            } else {
                heap1
            };
            // Push `melt` to `stack`.
            melt.next_sibling = stack;
            stack = Some(melt);
        }
        // Meld all heaps in the stack.
        let mut ret = None;
        while let Some(mut h) = stack {
            stack = h.next_sibling.take();
            ret = Self::meld(Some(h), ret);
        }
        ret
    }
}

struct Iter<'a, T: Ord> {
    stack: Vec<&'a Node<T>>,
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        let mut it = &node.first_child;
        while let Some(child) = it {
            it = &child.next_sibling;
            self.stack.push(&child);
        }
        Some(&node.value)
    }
}

#[cfg(test)]
mod tests {
    use alloc::collections::BinaryHeap;
    use alloc::vec;
    use alloc::vec::Vec;
    use core::cmp::Reverse;

    use crate::ParingHeap;

    #[test]
    fn basic_test() {
        let mut heap = ParingHeap::new();
        for x in [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9] {
            heap.push(x);
        }
        let mut actual = Vec::new();
        while !heap.is_empty() {
            let x = heap.pop().unwrap();
            actual.push(x);
        }
        let expected = vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 7, 8, 9, 9, 9];
        assert_eq!(expected, actual);
    }

    #[test]
    fn drop_test() {
        let mut heap = ParingHeap::new();
        for x in [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9] {
            heap.push(x);
        }
        drop(heap);
    }

    #[test]
    fn large_drop_test() {
        let n = 1000000;
        let mut heap = ParingHeap::new();
        for i in 0..n {
            heap.push(i);
        }
        drop(heap);
    }


    #[test]
    fn iter_test() {
        let mut heap = ParingHeap::new();
        for x in [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9] {
            heap.push(x);
        }
        let mut actual: Vec<_> = heap.iter().copied().collect();
        actual.sort();
        let expected = vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 7, 8, 9, 9, 9];
        assert_eq!(expected, actual);
    }

    #[test]
    fn randomized_test() {
        for _ in 0..1000 {
            let mut heap = ParingHeap::new();
            // BinaryHeap is max-heap. So, we need to push Reverse(x) to make it min-heap.
            let mut expected = BinaryHeap::new();
            for i in 0..100 {
                match rand::random::<u32>() % 2 {
                    0 => {
                        heap.push(i);
                        expected.push(Reverse(i));
                    }
                    1 => {
                        let actual_x = heap.pop();
                        let expected_x = expected.pop().map(|r| r.0);
                        assert_eq!(actual_x, expected_x);
                    }
                    _ => unreachable!(),
                }
                assert_eq!(expected.is_empty(), heap.is_empty());
                assert_eq!(expected.peek().map(|r| r.0), heap.peek().copied());
            }
        }
    }
}
