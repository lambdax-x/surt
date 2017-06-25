macro_rules! mk_test {
    ($name: ident,
     [$($input: ident = $t: ty),*],
     {$($expr: stmt);*},
     $sort: expr,
     $check: expr
    ) => {
        #[quickcheck]
        fn $name($($input: $t),*) -> bool{
            $($expr);*;
            $sort;
            $check
        }
    }
}

use super::*;
mod sorted;
mod stability;
