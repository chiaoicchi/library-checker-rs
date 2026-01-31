use data_strux::segment_tree::{Action, LazySegmentTree, Monoid};
use std::io::{BufWriter, Write, read_to_string, stdin, stdout};

const MOD: u32 = 998_244_353;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();
    let mut stdout = BufWriter::new(stdout().lock());

    let n: usize = stdin.next().unwrap().parse().unwrap();
    let q: usize = stdin.next().unwrap().parse().unwrap();

    let ab: Vec<LenSum> = (0..n)
        .map(|_| LenSum(stdin.next().unwrap().parse().unwrap(), 1))
        .collect();

    let mut segment_tree = LazySegmentTree::<LenSum, Affine>::from_slice(&ab);

    for _ in 0..q {
        let t: u8 = stdin.next().unwrap().parse().unwrap();
        if t == 0 {
            let l: usize = stdin.next().unwrap().parse().unwrap();
            let r: usize = stdin.next().unwrap().parse().unwrap();
            let c: u32 = stdin.next().unwrap().parse().unwrap();
            let d: u32 = stdin.next().unwrap().parse().unwrap();
            segment_tree.range_apply(l..r, Affine(c, d));
        } else {
            let l: usize = stdin.next().unwrap().parse().unwrap();
            let r: usize = stdin.next().unwrap().parse().unwrap();
            let LenSum(ans, _) = segment_tree.range_fold(l..r);
            writeln!(stdout, "{}", ans).ok();
        }
    }
}

#[derive(Clone)]
struct LenSum(u32, u32);

impl Monoid for LenSum {
    fn id() -> Self {
        LenSum(0, 0)
    }

    fn op(&self, other: &Self) -> Self {
        let s = self.0 + other.0;
        let l = self.1 + other.1;
        LenSum(
            if s >= MOD { s - MOD } else { s },
            if l >= MOD { l - MOD } else { l },
        )
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
        let b = (*a0 as u64 * *b1 as u64 % MOD as u64) as u32 + b0;
        Affine(a, if b >= MOD { b - MOD } else { b })
    }
}

impl Action<LenSum> for Affine {
    fn act(&self, s: &LenSum) -> LenSum {
        let sum = (self.0 as u64 * s.0 as u64 % MOD as u64) as u32
            + (self.1 as u64 * s.1 as u64 % MOD as u64) as u32;
        LenSum(if sum >= MOD { sum - MOD } else { sum }, s.1)
    }
}
