use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let const_mod = 1_000_000_007;
    let mut ans = 1;
    for i in 1..=n {
        ans *= i;
        ans %= const_mod;
    }
    println!("{}", ans);
}
