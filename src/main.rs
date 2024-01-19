use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        b: char,
    }
    let mut map = HashMap::new();
    map.insert('A', 'T');
    map.insert('T', 'A');
    map.insert('C', 'G');
    map.insert('G', 'C');

    println!("{}", map.get(&b).unwrap());
}
