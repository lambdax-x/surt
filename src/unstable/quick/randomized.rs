use unstable::quick;

pub fn sort<F, T: Ord>(rand: &F, v: &mut [T])
where F: Fn() -> usize {
    match v.len() {
        0  | 1 => return,
        len => {
            let p = randomized_partition(rand, v); 
            sort(rand, &mut v[0 .. p]);
            sort(rand, &mut v[p + 1 .. len]);
        }
    }
}

fn randomized_partition<F, T: Ord>(rand: &F, v: &mut [T]) -> usize
where F: Fn() -> usize {
   let len = v.len();
   let r = rand() % len;
   let last = len - 1;
   v.swap(r, last);
   quick::partition(v)
}
