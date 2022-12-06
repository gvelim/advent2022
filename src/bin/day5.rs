use std::collections::{HashMap, VecDeque};

fn main() {

    let data =
        "    [D]    \n\
        [N] [C]    \n\
        [Z] [M] [P]\n 1   2   3 \n\n\
         move 1 from 2 to 1\n\
         move 3 from 1 to 3\n\
         move 2 from 2 to 1\n\
         move 1 from 1 to 2\n";

    // let data = std::fs::read_to_string("src/bin/day5_input.txt").expect("Ops!");

    let [start,moves] = data.split("\n\n").collect::<Vec<_>>()[..] else { panic!("") };

    let buckets = parse_starting_pos(start);
    println!("{:?}",buckets);

}

fn parse_starting_pos(start:&str) -> HashMap<usize,VecDeque<char>> {
    start.lines()
        .rev()
        .skip(1)
        .map(|line| line.split("").filter_map(|e| e.chars().next()).collect::<Vec<_>>())
        .fold( HashMap::new(),|map, e|{
            e.into_iter()
                .enumerate()
                .filter(|(_, c)| c.is_alphanumeric())
                .fold(map, |mut out, (key,val)| {
                    out.entry(key).or_insert(VecDeque::new()).push_back(val);
                    out
                })
        })
}