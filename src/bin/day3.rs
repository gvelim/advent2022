use std::collections::HashSet;

fn main() {
    // let lines = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
    // jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
    // PmmdzqPrVvPwwTWBwg\n\
    // wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
    // ttgJtRGJQctTZtZT\n\
    // CrZsJsPPZsGzwwsLwLmpwMDw";

    let lines = std::fs::read_to_string("./src/bin/day3.txt").unwrap_or_else(|e| panic!("{e}"));

    println!("{:?}",component_1(&lines));
    println!("{:?}",component_2(&lines));
}

fn component_2(lines:&str) -> u32 {
    lines.lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| {
            group.iter()
                .map(|a| a.chars().collect::<HashSet<_>>())
                .reduce(|a, b|
                    a.intersection(&b).copied().collect::<HashSet<_>>()
                )
                .unwrap_or_else(|| panic!("Ops!"))
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
                .sum::<u32>()
        )
        .sum::<u32>()
}

fn component_1(lines: &str) -> u32 {
    lines.lines()
        .map(|line| line.split_at( line.len()>>1 ) )
        .map(|(compa, compb)| {
            compa.chars()
                .filter(|&c| compb.find(c).is_some() )
                .collect::<HashSet<_>>()
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
                .sum::<u32>()
        )
        .reduce(|sum, v| sum + v )
        .unwrap_or_else(|| unreachable!())
}