macro_rules! next {
    ($node:expr, Left) => { ($node + 1) * 2 - 1};
    ($node:expr, Right) => { ($node + 1) * 2 };
}

macro_rules! last_parent {
    ($size:expr) => { $size / 2 - 1 };
}

pub fn sort<T: Ord>(v: &mut [T]) {
    let len = v.len();
    let mut h = Heap {
        v: v,
        size: len
    };
    h.sort();
}

struct Heap<'a, T: 'a + Ord> {
    v: &'a mut [T],
    size: usize
}

impl<'a, T: Ord> Heap<'a, T> {
    fn sort(&mut self) {
        self.build_max_heap();
        for node in (1 .. self.size).rev() {
            self.v.swap(0, node);
            self.size -= 1;
            self.max_heapify(0);
        }
    }

    fn build_max_heap(&mut self) {
        match self.size {
            0 | 1 => return,
            size => {
                let first = last_parent!(size);
                for node in (0 .. first + 1).rev() {
                    self.max_heapify(node);
                }
            }
        }
    }

    fn max_heapify(&mut self, node: usize) {
        let left = next!(node, Left);
        let right = next!(node, Right);
        let mut largest;

        if left < self.size && self.v[left] > self.v[node] {
            largest = left;
        } else {
            largest = node;
        }
        if right < self.size && self.v[right] > self.v[largest] {
            largest = right;
        }

        if largest != node {
            self.v.swap(node, largest);
            self.max_heapify(largest);
        }
    }
}
