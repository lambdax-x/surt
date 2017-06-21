use stable::merge;
use stable::insertion;

pub fn sort<T: Ord + Copy>(v: &mut [T], threshold: usize) {
    match v.len() {
        0 | 1 => return,
        len if len <= threshold => insertion::sort(v),
        len => {
            let middle = len / 2;
            sort(&mut v[0 .. middle], threshold);
            sort(&mut v[middle .. len], threshold);
            merge::merge(v, middle);
        }
    }
}
