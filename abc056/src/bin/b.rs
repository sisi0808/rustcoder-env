use std::collections::*;
use proconio::*;
use std::cmp::*;

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
        w: i32,
        a: i32,
        b: i32,
    }
    println!("{}", max!{0, a-(b+w), b-(a+w)});

}
