use proconio::input;

fn judge(a: usize, b: usize) -> String {
    let c = [14, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    let a_idx = Some(c[a]);
    let b_idx = Some(c[b]);

    if a_idx > b_idx {
        "Alice".to_string()
    }
    else if a_idx < b_idx{
        "Bob".to_string()
    }
    else {
        "Draw".to_string()
    }
}

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    println!("{}", judge(a-1, b-1));
}
