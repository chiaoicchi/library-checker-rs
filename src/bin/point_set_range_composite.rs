use data_strux::segment_tree::{Monoid, SegmentTree};
use std::io::{BufWriter, Write, read_to_string, stdin, stdout};

const MOD: u32 = 998_244_353;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();
    let mut stdout = BufWriter::new(stdout().lock());

    let n: usize = stdin.next().unwrap().parse().unwrap();
    let q: usize = stdin.next().unwrap().parse().unwrap();

    let ab: Vec<Affine> = (0..n)
        .map(|_| {
            Affine(
                stdin.next().unwrap().parse().unwrap(),
                stdin.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut segment_tree = SegmentTree::<Affine>::from_slice(&ab);

    for _ in 0..q {
        let t: u8 = stdin.next().unwrap().parse().unwrap();
        if t == 0 {
            let p: usize = stdin.next().unwrap().parse().unwrap();
            let c: u32 = stdin.next().unwrap().parse().unwrap();
            let d: u32 = stdin.next().unwrap().parse().unwrap();
            segment_tree.set(p, Affine(c, d));
        } else {
            let l: usize = stdin.next().unwrap().parse().unwrap();
            let r: usize = stdin.next().unwrap().parse().unwrap();
            let x: u64 = stdin.next().unwrap().parse().unwrap();
            let Affine(a, b) = segment_tree.range_fold(l..r);
            let ans = (a as u64 * x % MOD as u64) as u32 + b;
            writeln!(stdout, "{}", if ans >= MOD { ans - MOD } else { ans }).ok();
        }
    }
}

#[derive(Clone)]
struct Affine(u32, u32);

impl Monoid for Affine {
    fn id() -> Self {
        Affine(1, 0)
    }

    fn op(&self, other: &Self) -> Self {
        let Affine(a0, b0) = self;
        let Affine(a1, b1) = other;
        let a = (*a0 as u64 * *a1 as u64 % MOD as u64) as u32;
        let b = (*a1 as u64 * *b0 as u64 % MOD as u64) as u32 + b1;
        Affine(a, if b >= MOD { b - MOD } else { b })
    }
}
