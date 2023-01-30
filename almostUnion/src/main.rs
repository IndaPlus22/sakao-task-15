mod alUnion;

use std::io::{self, stdin};
use alUnion::al_union;


fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input);
    let mut nm: Vec<usize> = input
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Could not parse numbers!"))
        .collect();

    input.clear();
    let mut union_thing: al_union = al_union::new(nm[0]);
    for i in 0..nm[1] {
        stdin().read_line(&mut input);
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
                println!("{}, {}", size, sum);
            },
            _ => ()
        }
    }
}
