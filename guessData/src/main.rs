use std::io::{self, stdin};
use std::io::prelude::*;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input);
    let n: usize = input.trim().parse().unwrap();
    input.clear();

    while stdin().read_line(&mut input).unwrap() > 0 {
        
    }
}
