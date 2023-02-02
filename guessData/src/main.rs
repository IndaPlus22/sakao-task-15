use std::io::prelude::*;
use std::io::{self, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input);

    let mut is_stack = false;
    let mut is_queue = false;
    let mut is_prioq = false;

    let mut n: usize = input.trim().parse().unwrap();
    let mut i: usize = 0;
    let mut datastr: Vec<usize> = Vec::with_capacity(n);

    input.clear();
    while stdin().read_line(&mut input).unwrap() > 0 {
        let tmps = input.trim().split(" ").collect::<Vec<&str>>();
        if tmps.len() == 1 {
            // fro the last data
            check_data(is_stack, is_queue, is_prioq);

            n = tmps[0].parse().unwrap();
            datastr = Vec::with_capacity(n);
            i = 0;
        } else {
            let val = tmps[1].parse().unwrap();
            println!("tmps: {:?}", tmps);
            if tmps[0] == "1" {
                println!("added: {}", val);
                datastr.push(val);
            } else {
                if datastr.len() == 0 || (val != datastr[datastr.len() - 1]
                    && datastr[0] != val
                    && datastr.iter().max().unwrap() != &val)
                {
                    is_stack = false;
                    is_queue = false;
                    is_prioq = false;
                } else {
                    if val == datastr[datastr.len() - 1] {
                        is_stack = true;
                        // datastr.remove(datastr.len() - 1);
                    }
                    if datastr[0] == val {
                        is_queue = true;
                        // datastr.remove(0);
                    }
                    if datastr.iter().max().unwrap() == &val {
                        is_prioq = true;
                        // datastr.remove(datastr.iter().position(|x| x == &val).unwrap());
                    }
                }
                
            }
            println!("datastr: {:?}", datastr);
            i += 1;
        }

        input.clear();
    }
}

fn check_data(s: bool, q: bool, p: bool) {
    if !s && !q && !p {
        println!("impossible");
    } else if s as u8 + q as u8 + p as u8 >= 2 {
        println!("not sure");
    } else if s {
        println!("stack");
    } else if q {
        println!("queue");
    } else {
        println!("priority queue");
    }
}
