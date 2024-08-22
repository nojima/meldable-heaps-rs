/// `SkewHeap` is a priority queue implemented with skew heaps.
/// `SkewHeap` is a **min-heap**, which means that the minimum element is popped first.
pub struct SkewHeap<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> SkewHeap<T> {
    /// Constructs a empty `SkewHeap`.
    /// O(1) time.
    pub fn new() -> Self {
        SkewHeap { root: None }
    }

    /// Returns the number of elements in the heap.
    /// O(1) time.
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Inserts a value into the heap.
    /// O(log n) amortized time.
    pub fn push(&mut self, value: T) {
        self.root = Node::meld(self.root.take(), Node::singleton(value));
    }

    /// Removes the minimum element from the heap and returns it, or `None` if it is empty.
    /// O(log n) amortized time.
    pub fn pop(&mut self) -> Option<T> {
        let root = self.root.take()?;
        self.root = Node::meld(root.left, root.right);
        Some(root.value)
    }

    /// Returns a reference to the minimum element in the heap, or `None` if it is empty.
    /// O(1) time.
    pub fn peek(&self) -> Option<&T> {
        self.root.as_ref().map(|node| &node.value)
    }

    /// Melds two heaps into a single heap.
    /// O(log n) amortized time.
    pub fn meld(heap1: SkewHeap<T>, heap2: SkewHeap<T>) -> SkewHeap<T> {
        let root = Node::meld(heap1.root, heap2.root);
        Self { root }
    }

    /// Returns an iterator that visits all elements in the heap, in arbitrary order.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let mut stack = Vec::new();
        if let Some(ref root) = self.root {
            stack.push(root.as_ref());
        }
        Iter { stack }
    }
}

struct Node<T: Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn singleton(value: T) -> Option<Box<Node<T>>> {
        Some(Box::new(Self {
            value,
            left: None,
            right: None,
        }))
    }

    // `meld` implements `imeld` function from Sleator and Tarjan's paper:
    // https://www.cs.cmu.edu/~sleator/papers/Adjusting-Heaps.htm
    fn meld(root1: Option<Box<Node<T>>>, root2: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        let (mut root1, mut root2) = match (root1, root2) {
            (None, root2) => return root2,
            (root1, None) => return root1,
            (Some(r1), Some(r2)) => (r1, r2),
        };

        // Ensure root1 <= root2
        if root1.value > root2.value {
            std::mem::swap(&mut root1, &mut root2);
        }

        // Skew root
        std::mem::swap(&mut root1.left, &mut root1.right);

        // Setup loop variables
        let mut parent = &mut root1;
        let mut opt_node1 = parent.left.take();
        let mut node2 = root2;

        while let Some(mut node1) = opt_node1 {
            // Ensure node1 <= node2
            if node1.value > node2.value {
                std::mem::swap(&mut node1, &mut node2);
            }

            // Skew `node1`
            std::mem::swap(&mut node1.left, &mut node1.right);

            // Make `node1` the left child of `parent`
            parent.left = Some(node1);

            // Update loop variables
            parent = parent.left.as_mut().unwrap();
            opt_node1 = parent.left.take();
        }

        parent.left = Some(node2);

        Some(root1)
    }
}

struct Iter<'a, T: Ord> {
    stack: Vec<&'a Node<T>>,
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        if let Some(left) = &node.left {
            self.stack.push(left);
        }
        if let Some(right) = &node.right {
            self.stack.push(right);
        }
        Some(&node.value)
    }
}

#[cfg(test)]
mod tests {
    use std::{cmp::Reverse, collections::BinaryHeap};

    use crate::SkewHeap;

    #[test]
    fn basic_test() {
        let mut heap = SkewHeap::new();
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
    fn iter_test() {
        let mut heap = SkewHeap::new();
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
            let mut heap = SkewHeap::new();
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
