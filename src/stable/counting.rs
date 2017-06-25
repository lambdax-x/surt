pub fn sort<T: Copy, F>(k: usize, v: &mut [T], rank: F)
where F: Fn(&T) -> usize {
    let count_len = k + 1;
    let mut count: Vec<usize> = vec![0; count_len];

    {
        let v_ranked = v.iter().map(|x| rank(x));
        for i in v_ranked {
            count[i] += 1;
        }
    }
    /// count[i]: number of occurences of i in v.map(rank)

    for i in 1 .. count_len {
        count[i] += count[i - 1];
    }
    /// count[i]: number of occurences less or equal than i in v.map(rank)

    let mut w: Vec<Option<T>> = vec![None; v.len()];
    {
        let v_ranked = v.iter().map(|x| rank(x));
        for (e, i) in v.iter().zip(v_ranked).rev() {
            w[count[i] - 1] = Some(e.clone());
            count[i] -= 1;
        }
    }

    for (to, from) in v.iter_mut().zip(w.iter()) {
        match *from {
            Some(a) => *to = a,
            _ => {}
        }
    }
}
