use std::cmp::Ordering;
use super::stable;

fn is_stable<T: Ord>(v: &[Indexed<T>]) -> bool {
    for (e, next) in v.iter().zip(v.iter().skip(1)) {
        if e.stable_cmp(next) == Ordering::Greater {
            return false;
        }
    }
    true
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Indexed<T: Ord> {
    v: T,
    index: usize
}

impl<T: Ord> PartialOrd for Indexed<T> {
    fn partial_cmp(&self, other: &Indexed<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for Indexed<T> {
    fn cmp(&self, other: &Indexed<T>) -> Ordering {
        self.v.cmp(&other.v)
    }
}

impl<T: Ord> Indexed<T> {
    fn stable_cmp(&self, other: &Indexed<T>) -> Ordering {
        match self.v.cmp(&other.v) {
            Ordering::Equal if self.index > other.index => Ordering::Greater,
            o => o
        }
    }
}

macro_rules! index_vec {
    ($v: ident) => {
        (0..).zip($v.iter())
            .map(|(i, x)| Indexed {
                v: *x,
                index: i
            })
            .collect()
    }
}

macro_rules! test_stable {
    ($name: ident, $sort: path) => {
        mk_test!($name,
                 [v=Vec<i32>],
                 {
                     let mut w: Vec<Indexed<i32>> = index_vec!(v)
                 },
                 $sort(&mut w),
                 is_stable(&w)
                );
    }
}

test_stable!(insertion_stable, stable::insertion::sort);
test_stable!(merge_sorted, stable::merge::sort);
test_stable!(merge_stable, stable::merge::sort);
mk_test!(merge_then_insertion,
         [v=Vec<i32>, threshold=usize],
         {
            let mut w: Vec<Indexed<i32>> = index_vec!(v)
         },
         stable::merge::insertion::sort(threshold, &mut w),
         is_stable(&w)
);
mk_test!(counting,
         [v=Vec<u32>],
         {
             let k = *v.iter().max().unwrap_or(&0) as usize;
             let mut w: Vec<Indexed<u32>> = index_vec!(v)
         },
         stable::counting::sort(k, &mut w, |ranked| ranked.v as usize),
         is_stable(&w)
);
