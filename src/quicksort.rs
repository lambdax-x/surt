pub fn sort<T: Ord>(v: &mut [T]) {
    match v.len() {
        0 => return,
        1 => return,
        len => {
            let p = partition(v); 
            sort(&mut v[0 .. p]);
            sort(&mut v[p + 1 .. len]);
        }
    }
}

fn partition<T: Ord>(v: &mut [T]) -> usize {
    let p = v.len() - 1;
    let mut next_swap: usize = 0;
    for j in 0 .. p {
        if v[j] <= v[p] {
            v.swap(next_swap, j);
            next_swap += 1;
        }
    }
    v.swap(next_swap, p);
    next_swap
}
