use rand::random;
use super::*;

fn sorted<T: Ord>(v: &[T]) -> bool {
    for (e, next) in v.iter().zip(v.iter().skip(1)) {
        if e > next {
            return false;
        }
    }
    true
}

macro_rules! test_sorted {
    ($name: ident, $sort: path) => {
        mk_test!($name,
                 [v=Vec<i32>],
                 {
                    let mut w: Vec<i32> = v
                 },
                 $sort(&mut w),
                 sorted(&w)
                );
    }
}

test_sorted!(insertion, stable::insertion::sort);
test_sorted!(merge, stable::merge::sort);
mk_test!(merge_then_insertion,
         [v=Vec<i32>, threshold=usize],
         {
            let mut w: Vec<i32> = v
         },
         stable::merge::insertion::sort(threshold, &mut w),
         sorted(&w)
);
mk_test!(counting,
         [v=Vec<u32>],
         {
             let k = *v.iter().max().unwrap_or(&0) as usize;
             let mut w: Vec<u32> = v
         },
         stable::counting::sort(k, &mut w, |x| *x as usize),
         sorted(&w)
);
test_sorted!(selection, unstable::selection::sort);
test_sorted!(quick, unstable::quick::sort);
mk_test!(quick_randomized,
         [v=Vec<i32>],
         {
             let mut w: Vec<i32> = v
         },
         unstable::quick::randomized::sort(&random, &mut w),
         sorted(&w)
);
test_sorted!(heap, unstable::heap::sort);
