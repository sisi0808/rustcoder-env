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
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }
    println!("{}", max(a * b, c * d));
}
