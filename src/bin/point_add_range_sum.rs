use data_strux::fenwick_tree::{FenwickTree, HasInverse, Monoid};
use std::io::{BufWriter, Write, read_to_string, stdin, stdout};

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();
    let mut stdout = BufWriter::new(stdout().lock());

    let n: usize = stdin.next().unwrap().parse().unwrap();
    let q: usize = stdin.next().unwrap().parse().unwrap();

    let a: Vec<Add> = (0..n)
        .map(|_| Add(stdin.next().unwrap().parse().unwrap()))
        .collect();

    let mut fenwick_tree = FenwickTree::<Add>::from_vec(a);

    for _ in 0..q {
        let t: u8 = stdin.next().unwrap().parse().unwrap();
        if t == 0 {
            let p: usize = stdin.next().unwrap().parse().unwrap();
            let x: u64 = stdin.next().unwrap().parse().unwrap();
            fenwick_tree.operate(p, Add(x));
        } else {
            let l: usize = stdin.next().unwrap().parse().unwrap();
            let r: usize = stdin.next().unwrap().parse().unwrap();
            writeln!(stdout, "{}", fenwick_tree.range_fold(l..r).0).ok();
        }
    }
}

#[derive(Clone, Copy)]
struct Add(u64);

impl Monoid for Add {
    fn id() -> Self {
        Add(0)
    }

    fn op(&self, other: &Self) -> Self {
        Add(self.0.wrapping_add(other.0))
    }
}

impl HasInverse for Add {
    fn inv(&self) -> Self {
        Add(self.0.wrapping_neg())
    }
}
