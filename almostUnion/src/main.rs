mod alUnion;

use alUnion::*;
use std::io::{self, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("what");
    while input.trim().is_empty() {
        // println!("Waiting for input...");
        input.clear();
        stdin().read_line(&mut input).expect("what");
    }
    // input.clear();
    // stdin().read_line(&mut input).unwrap();
    // println!("input: {}", input);
    let mut nm: Vec<usize> = input
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Could not parse numbers!"))
        .collect();

    // println!("nm: {:?}", nm);

    input.clear();
    let mut union_thing: ThUnion = ThUnion::new(nm[0]);
    // let mut union_thing: AlUnion = AlUnion::new(nm[0]);
    // let mut union_thing: BetUnion = BetUnion::new(nm[0]);
    for _ in 0..nm[1] {
        stdin().read_line(&mut input).unwrap();
        let mut cmd: Vec<usize> = input
            .trim()
            .split(" ")
            .map(|x| x.parse().expect("Could not parse numbers!"))
            .collect();

        match cmd[0] {
            1 => union_thing.union(cmd[1], cmd[2]),
            2 => union_thing.move_val(cmd[1], cmd[2]),
            3 => {
                let (size, sum) = union_thing.get(cmd[1]);
                println!("{} {}", size, sum);
            }
            _ => (),
        }

        input.clear();
    }
}
