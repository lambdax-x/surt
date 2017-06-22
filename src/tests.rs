extern crate rand;

use super::*;
use quickcheck::quickcheck;

fn sorted<T: Ord>(v: &[T]) -> bool {
    for (e, next) in v.iter().zip(v.iter().skip(1)) {
        if *e > *next {
            return false
        }
    }
    true
}

macro_rules! mk_test_sort {
    ($name: ident,
     $sort: path,
     $v: ident,
     $type: ty,
     ($($arg: expr),*),
     {$($expr:stmt);*}
    ) => {
        #[test]
        fn $name() {
            fn prop(mut $v: Vec<$type>) -> bool {
                $($expr);*;
                $sort($($arg),*);
                sorted(&$v)
            }
            quickcheck(prop as fn(_) -> _);
        }
    }
}

macro_rules! test_sort {
    ($name:ident, $sort:path) => {
        mk_test_sort!($name, $sort, v, i32, (&mut v), {});
    };
}

test_sort!(insertion, stable::insertion::sort);
test_sort!(merge, stable::merge::sort);
test_sort!(selection, unstable::selection::sort);
test_sort!(quick, unstable::quick::sort);
test_sort!(heap, unstable::heap::sort);
mk_test_sort!(counting, stable::counting::sort, v, u32,
              (max, &mut v), {
                  let max = *v.iter().max().unwrap_or(&0)
              });
mk_test_sort!(merge_then_insertion, stable::merge::insertion::sort, v, i32,
              (7, &mut v), {});
mk_test_sort!(quick_randomized, unstable::quick::randomized::sort, v, i32,
              (&rand::random, &mut v), {});
