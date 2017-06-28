use test::Bencher;
use rand::random;
use super::*;

macro_rules! mk_bench {
    ($name: ident,
     [$($stmt: stmt),+],
     $expr: expr
    ) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            $($stmt);+;
            b.iter(|| $expr)
        }
    }
}

macro_rules! algorithms {
    ($entry: ident) => {
        $entry!(rust_sort, v, v.sort());
        $entry!(insertion, v, stable::insertion::sort(&mut v));
        $entry!(merge, v, stable::merge::sort(&mut v));
        $entry!(merge_then_insertion,
                v,
                stable::merge::insertion::sort(7, &mut v)
        );
        $entry!(selection, v, unstable::selection::sort(&mut v));
        $entry!(quick, v, unstable::quick::sort(&mut v));
        $entry!(quick_randomized,
                v,
                unstable::quick::randomized::sort(&random, &mut v)
        );
        $entry!(heap, v, unstable::heap::sort(&mut v));
        $entry!(couting,
                v,
                stable::counting::sort
                ( *v.iter().max().unwrap_or(&0) as usize
                , &mut v
                , |x| *x as usize
                )
        );
    }
}

macro_rules! mk_bench_vec_u32 {
    ( $mode: ident
    , $v: ident
    , $size: expr
    , $name: ident
    , $sort: expr
    ) => {
        mk_bench!($name,
                  [let mut $v: Vec<u32> = vec_u32!($mode, $v, $size)],
                  $sort
        );
    }
}

macro_rules! vec_u32 {
    (sorted, $v: ident, $size: expr) => {
        (0..).take($size).collect()
    };
    (reversed, $v: ident, $size: expr) => {
        (0 .. $size).rev().collect()
    };
    (constant, $v: ident, $size: expr) => {
        (0 .. 1).cycle().take($size).collect()
    };
}

macro_rules! mk_benches_size_mod {
    ($algorithms: ident, $bench: ident) => {
        mod $bench {
            use super::*;
            $algorithms!($bench);
        }
    }
}

macro_rules! mk_benches_order_mod {
    ($algorithms: ident, $mode: ident) => {
        mod $mode {
            use super::*;

            macro_rules! small {
                ( $name: ident
                , $v: ident
                , $sort: expr
                ) => {
                    mk_bench_vec_u32!($mode, $v, 20, $name, $sort);
                }
            }

            macro_rules! large {
                ( $name: ident
                , $v: ident
                , $sort: expr
                ) => {
                    mk_bench_vec_u32!($mode, $v, 5000, $name, $sort);
                }
            }

            mk_benches_size_mod!($algorithms, small);
            mk_benches_size_mod!($algorithms, large);
        }
    }
}

mk_benches_order_mod!(algorithms, sorted);
mk_benches_order_mod!(algorithms, reversed);
mk_benches_order_mod!(algorithms, constant);
