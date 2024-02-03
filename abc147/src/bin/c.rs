#![allow(unused_macros)]
use std::collections::*;
use itertools::Itertools;
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
    };
    let mut a = vec![];
    for _ in 0..n {
        input!{
            a_size: usize,
            xy: [(usize, usize); a_size],
        };
        a.push(xy);
    }

    let mut ans = 0;
    'out: for b in 0..(1<<n){
        let honest_list = (0..n).map(|x| if (1 << x) & b > 0 { 1 } else { 0 }).collect_vec();
        for flag in 0..n{
            // 正直者のとき
            if (1 << flag) & b > 0{
                for &(x, y) in &a[flag]{
                    if y != honest_list[x-1]{
                        continue 'out
                    }
                }
            }
        }
        ans = max(ans, honest_list.iter().filter(|&&x| x == 1).count());
    }
    println!("{ans}");

}
