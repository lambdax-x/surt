pub fn sort_with(max: u32, v: &mut [u32]) {
    let count_len = (max + 1) as usize;
    let mut count: Vec<usize> = vec![0; count_len];

    for e in v.iter() {
        let i = *e as usize;
        count[i] += 1;
    }

    for i in 1 .. count_len {
        count[i] += count[i - 1];
    }

    let mut w = vec![0; v.len()];

    for e in v.iter().rev() {
        let i = *e as usize;
        w[count[i] - 1] = *e;
        count[i] -= 1;
    }

    for (to, from) in v.iter_mut().zip(w.iter()) {
        *to = *from;
    }
}
