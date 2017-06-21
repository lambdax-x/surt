pub fn sort<T: Ord>(v: &mut [T]) {
    match v.len() {
        0 | 1 => return,
        len => {
            for i in 0 .. len - 1 {
                let min = find_first_min(&v[i .. len]) + i;
                if min != i {
                    v.swap(i, min);
                }
            }
        }
    }
}

fn find_first_min<T: Ord>(v: & [T]) -> usize {
    let mut min = 0;
    for (i, e) in v.iter().enumerate() {
        if *e < v[min] {
            min = i;
        }
    }
    min
}
