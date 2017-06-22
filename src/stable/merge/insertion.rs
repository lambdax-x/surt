use stable::merge;
use stable::insertion;

pub fn sort<T: Ord + Copy>(threshold: usize, v: &mut [T]) {
    match v.len() {
        0 | 1 => return,
        len if len <= threshold => insertion::sort(v),
        len => {
            let middle = len / 2;
            sort(threshold, &mut v[0 .. middle]);
            sort(threshold, &mut v[middle .. len]);
            merge::merge(v, middle);
        }
    }
}
