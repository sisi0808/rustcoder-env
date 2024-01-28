#![allow(unused_macros)]
use proconio::*;
use std::cmp::*;
use std::collections::*;

const INF: isize = 1_000_000_000 + 7;
#[allow(dead_code)]
const MOD: usize = 1_000_000_000 + 7;
#[allow(dead_code)]
const D: [(isize, isize); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

macro_rules! rep {
    ($n: expr, $body: block) => {
        for _ in 0..$n {
            $body
        }
    };
    ($i: ident, $n: expr, $body: block) => {
        for $i in 0..$n {
            $body
        }
    };
    ($i: ident, $a: expr, $b: expr, $body: block) => {
        for $i in $a..=$b {
            $body
        }
    };
    ($i: ident, $a: expr, $b: expr, $c: expr, $body: block) => {
        let mut $i = $a;
        while $i <= $b {
            $body
            $i += $c;
        }
    };
}

macro_rules! yn {
    ($val:expr) => {
        if $val {
            println!("Yes");
        } else {
            println!("No");
        }
    };
}

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
        a: isize,
        b: isize,
        c: isize,
        x: isize,
        y: isize,
    };

    let mut ans = INF;
    for a_cnt in 0..=x {
        let b_cnt = max(0, y - (x - a_cnt));
        let c_cnt = (x - a_cnt) * 2;
        let tmp = a_cnt * a + b_cnt * b + c_cnt * c;
        ans = min(ans, tmp);
    }
    for b_cnt in 0..=y {
        let a_cnt = max(0, x - (y - b_cnt));
        let c_cnt = (y - b_cnt) * 2;
        let tmp = a_cnt * a + b_cnt * b + c_cnt * c;
        ans = min(ans, tmp);
    }
    println!("{}", ans);
}
