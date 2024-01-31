#![allow(unused_macros)]
use std::collections::*;
use proconio::*;
use std::cmp::*;

#[allow(dead_code)]
const INF: usize = 1_000_000_000_000_000_000;
const MOD: usize = 1_000_000_000 + 7;
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
    input!{
        n: usize,
        m: usize,
        a: [usize; m],
    };

    let mut v: Vec<isize> = vec![0;n+2];
    v[0] = 1;
    for &idx in &a{
        v[idx] = -1;
    }

    for i in 0..n{
        if v[i] == -1{
            continue;
        }
        if v[i + 1] != -1{
            v[i+1] += v[i];
            v[i+1] %= MOD as isize;
        }
        if v[i + 2] != -1{
            v[i+2] += v[i];
            v[i+2] %= MOD as isize;
        }

    }
    println!("{}", v[n]);

}
