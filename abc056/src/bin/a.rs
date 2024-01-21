use proconio::input;

fn main() {
    input! {
        a: char,
        b: char,
    }

    if a == 'H' && b == 'H'{
        println!("H");
    } else if a == 'H' && b == 'D'{
        println!("D");
    } else if a == 'D' && b == 'H'{
        println!("D");
    } else if a == 'D' && b == 'D'{
        println!("H");
    }
}
