use test::Bencher;
use rand::random;
use super::*;

macro_rules! algorithms {
    ($entry: ident, $vec_generator: expr) => {
        $entry!(rust_sort,
                v,
                $vec_generator,
                v.sort()
        );
        $entry!(insertion,
                v,
                $vec_generator,
                stable::insertion::sort(&mut v)
        );
        $entry!(merge,
                v,
                $vec_generator,
                stable::merge::sort(&mut v)
        );
        $entry!(merge_then_insertion,
                v,
                $vec_generator,
                stable::merge::insertion::sort(64, &mut v)
        );
        $entry!(selection,
                v,
                $vec_generator,
                unstable::selection::sort(&mut v));
        $entry!(quick,
                v,
                $vec_generator,
                unstable::quick::sort(&mut v)
        );
        $entry!(quick_randomized,
                v,
                $vec_generator,
                unstable::quick::randomized::sort(&random, &mut v)
        );
        $entry!(heap,
                v,
                $vec_generator,
                unstable::heap::sort(&mut v)
        );
        $entry!(couting,
                v,
                $vec_generator,
                stable::counting::sort(*v.iter().max().unwrap_or(&0) as usize,
                                       &mut v,
                                       |x| *x as usize
                )
        );
    }
}

macro_rules! vec_sizes {
    ($entry: ident) => {
        $entry!(small, 20);
        $entry!(medium, 200);
        $entry!(large, 2000);
    }
}

macro_rules! vec_types {
    ($entry: ident, $size: expr) => {
        $entry!(constant, vec![0 ; $size]);
        $entry!(sorted, (0 .. $size).collect());
        $entry!(reversed, (0 .. $size).rev().collect());
        $entry!(small_cycle, (0 .. 10).cycle().take($size).collect());
    }
}

macro_rules! mk_vec_size_mod {
    ($name: ident, $size: expr) => {
        mod $name {
            use super::*;
            vec_types!(mk_vec_type_mod, $size);
        }
    }
}

macro_rules! mk_vec_type_mod {
    ($name: ident, $vec_generator: expr) => {
        mod $name {
            use super::*;
            algorithms!(mk_bench_u32, $vec_generator);
        }
    }
}

macro_rules! mk_bench_u32 {
    ( $name: ident
    , $v: ident
    , $vec_generator: expr
    , $sort: expr
    ) => {
        mk_bench!($name,
                  [let mut $v: Vec<u32> = $vec_generator],
                  $sort
        );
    }
}

macro_rules! mk_bench {
    ($name: ident,
     [$($stmt: stmt),*],
     $expr: expr
    ) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            $($stmt);*;
            b.iter(|| $expr)
        }
    }
}

vec_sizes!(mk_vec_size_mod);
