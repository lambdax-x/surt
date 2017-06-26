use test::Bencher;
use rand::random;
use super::*;

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

macro_rules! bench_algorithms {
    ($bench: ident, $v: ident) => {
        $bench!(insertion, $v, stable::insertion::sort(&mut $v));
        $bench!(merge, $v, stable::merge::sort(&mut $v));
        $bench!(merge_then_insertion,
                $v,
                stable::merge::insertion::sort(7, &mut $v)
        );
        $bench!(selection, $v, unstable::selection::sort(&mut $v));
        $bench!(quick, $v, unstable::quick::sort(&mut $v));
        $bench!(quick_randomized,
                $v,
                unstable::quick::randomized::sort(&random, &mut $v)
        );
        $bench!(heap, $v, unstable::heap::sort(&mut $v));
    }
}

mod sorted {
    use super::*;
    
    macro_rules! bench_sorted {
        ($name: ident, $v: ident, $sort: expr) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                let mut $v: Vec<u32> = vec_u32!(sorted, $v, 5000);
                b.iter(|| $sort)
            }
        }
    }
    
    bench_algorithms!(bench_sorted, v);
}
