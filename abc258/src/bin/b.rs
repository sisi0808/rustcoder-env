#![allow(unused_macros)]
use proconio::*;
use std::cmp::*;
use std::collections::*;

#[allow(dead_code)]
const MOD: usize = 1_000_000_000 + 7;

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

// const dx = [0, 1 , 0, -1];
// const dy = [-1, 0 , 1, 0];
const DX: [i32; 8] = [0, 1, 1, 1, 0, -1, -1, -1];
const DY: [i32; 8] = [-1, -1, 0, 1, 1, 1, 0, -1];

fn main() {
    input! {
        n: i32,
          a: [proconio::marker::Chars;n],
    }

    let mut ans = 0;
    rep!(i, n, {
        rep!(j, n, {
            rep!(k, 8, {
                let mut tmp: i64 = 0;
                rep!(l, n, {
                    tmp *= 10;
                    let x = ((i as i32) + DX[k] * (l as i32) + n) % n;
                    let y = ((j as i32) + DY[k] * (l as i32) + n) % n;
                    tmp += a[y as usize][x as usize] as i64 - '0' as i64;
                });
                ans = max(ans, tmp);
            });
        });
    });
    println!("{}", ans);
}
