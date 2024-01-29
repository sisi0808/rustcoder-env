#![allow(unused_macros)]
use proconio::{marker::Chars, *};
use std::cmp::*;
use std::collections::*;

#[allow(dead_code)]
const MOD: usize = 1_000_000_000 + 7;
const D: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

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
        h:usize,
        w:usize,
        s: [Chars;h],
    };

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                continue;
            }
            let mut flag = false;
            for (dx, dy) in D {
                let x = min(w as isize - 1, max(0, dx + j as isize));
                let y = min(h as isize - 1, max(0, dy + i as isize));
                if s[y as usize][x as usize] == '#' {
                    flag = true;
                }
            }
            if !flag {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
