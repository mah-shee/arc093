#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [isize; n]
    }
    let mut b: Vec<isize> = vec![0];
    b.append(&mut a);
    b.push(0);

    let mut total = 0;
    for i in 0..n + 1 {
        total += (b[i + 1] - b[i]).abs();
    }
    for i in 1..n + 1 {
        println!(
            "{}",
            total + (b[i - 1] - b[i + 1]).abs()
                - ((b[i - 1] - b[i]).abs() + (b[i] - b[i + 1]).abs())
        );
    }
}
