pub fn sort<T: Ord + Copy>(v: &mut [T]) {
    let len = v.len();
    match len {
        0 => return,
        1 => return,
        _ => {
            let middle = len / 2;
            sort(&mut v[0 .. middle]);
            sort(&mut v[middle .. len]);
            merge(v, middle);
        }
    }
}

pub fn merge<T: Ord + Copy>(v: &mut [T], middle: usize) {
    let len = v.len();
    let mut w = Vec::<T>::new();
    for i in 0 .. len {
        w.push(v[i]);
    }

    let mut left = 0;
    let mut right = middle;
    for k in 0 .. len {
        if left < middle && (right == len || w[left] < w[right]) {
            v[k] = w[left];
            left += 1;
        } else {
            v[k] = w[right];
            right += 1;
        }
    }
}

pub mod insertion;
