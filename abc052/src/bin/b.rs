use proconio::*;
use std::cmp::*;
use std::collections::*;

macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}

macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min!($($z),*);
        if $x < y {
            $x
        } else {
            y
        }
    }}
}

fn main() {
    input! {
        _n: i32,
        s: String
    }

    let mut ans = 0;
    let mut tmp = 0;
    for c in s.chars(){
        dbg!(c);
        if c == 'I'{
            tmp+=1;
        }
        else{
            tmp-=1;
        }
        ans = max(ans, tmp);
    }
    print!("{}", ans);
}
