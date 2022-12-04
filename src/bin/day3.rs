use std::collections::HashSet;

fn main() {
    // let test = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
    // jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
    // PmmdzqPrVvPwwTWBwg\n\
    // wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
    // ttgJtRGJQctTZtZT\n\
    // CrZsJsPPZsGzwwsLwLmpwMDw";

    let lines = std::fs::read_to_string("./src/bin/day3.txt").unwrap_or_else(|e| panic!("{e}"));

    println!("{:?}",component_1(&lines));

    let count = lines.lines().count() / 3;
}

fn component_1(lines: &str) -> u32 {
    lines.lines()
        .map(|line| line.split_at( line.len()>>1 ) )
        .map(|(compa, compb)| {
            compa.chars().filter(|&c| compb.find(c).is_some() ).collect::<HashSet<_>>()
        })
        .map(|set|
            set.into_iter()
                .map(|c|
                    match c {
                        'a'..='z' => u32::from(c) - u32::from('a') + 1,
                        'A'..='Z' => u32::from(c) - u32::from('A') + 27,
                        _ => panic!("use only alphabetic characters")
                    }
                )
                .collect::<Vec<u32>>()
        )
        .fold(0,|sum, v|
            sum + v.iter().sum::<u32>()
        )
}