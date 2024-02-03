use std::io::{self};

fn main() {
    let mut input = String::new();

    let mut total: i32 = 0;

    let lines = io::stdin().lines();

    for line in lines {
        let line = line.unwrap();
        if line.len() == 0 {
            break;
        }
        let num: i32 = line.trim().parse().unwrap_or(0);
        total += num;
    }

    println!("total: {}", total);
}