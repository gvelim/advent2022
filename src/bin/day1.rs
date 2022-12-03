use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;

fn main() {

    let fs = std::fs::read_to_string("./src/bin/day1_input.txt").unwrap_or_else(|e| panic!("{e}"));

<<<<<<< HEAD
    let mut out = fs.split("\n\n")
=======
    let out = fs.split("\n\n")
>>>>>>> 62d620e (Day 1 pass)
        .into_iter()
        .map(|e| e.split('\n'))
        .map(|v|
            v.filter_map(|e| u64::from_str(e).ok() ).collect::<Vec<u64>>()
        )
        .fold(BinaryHeap::new(), |mut out, v|{
            out.push(v.iter().sum::<u64>());
            out
        });
<<<<<<< HEAD
    println!("{:?}",out.iter().take(3).collect::<Vec<_>>());
    println!("{:?}",out.iter().take(3).sum::<u64>());
=======
    println!("Q1: {:?}",out.iter().take(3).collect::<Vec<_>>());
    println!("Q2: {:?}",out.iter().take(3).sum::<u64>());
>>>>>>> 62d620e (Day 1 pass)

}