# Day 3: Code

Below is the complete code for Day 3, which solves the Rucksack Reorganization problem.

## Full Solution

```advent2022/src/bin/day3.rs#L1-58
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
```

## Code Walkthrough

### Imports and Setup

```advent2022/src/bin/day3.rs#L1-13
use std::collections::HashSet;

fn main() {
    // Commented out test input
    
    let lines = std::fs::read_to_string("./src/bin/day3.txt").unwrap_or_else(|e| panic!("{e}"));

    println!("{:?}",component_1(&lines));
    println!("{:?}",component_2(&lines));
}
```

The solution imports the `HashSet` collection type which is used to efficiently find common elements. The main function reads the input file and calls the two component functions that solve parts 1 and 2 of the problem.

### Part 2: Finding Group Badges

```advent2022/src/bin/day3.rs#L15-37
fn component_2(lines:&str) -> u32 {
    lines.lines()
        .collect::<Vec<_>>()        // Convert lines to a vector
        .chunks(3)                  // Group lines into sets of three
        .map(|group| {
            group.iter()
                .map(|a| a.chars().collect::<HashSet<_>>())  // Convert each line to a character set
                .reduce(|a, b|
                    a.intersection(&b).copied().collect::<HashSet<_>>()  // Find common characters
                )
                .unwrap_or_else(|| panic!("Ops!"))
        })
        .map(|set|
            set.into_iter()
                .map(|c|
                    match c {  // Calculate character priority
                        'a'..='z' => u32::from(c) - u32::from('a') + 1,   // 1-26 for lowercase
                        'A'..='Z' => u32::from(c) - u32::from('A') + 27,  // 27-52 for uppercase
                        _ => panic!("use only alphabetic characters")
                    }
                )
                .sum::<u32>()  // Sum priorities of common characters in a group
        )
        .sum::<u32>()  // Sum results from all groups
}
```

This function handles Part 2 of the problem, finding the common item (badge) among each group of three elves.

The solution works by:
1. Grouping rucksacks into sets of three using `chunks(3)`
2. For each group, converting each rucksack into a `HashSet` of characters
3. Using `reduce` with `intersection` to find characters common to all three rucksacks
4. Calculating the priority of the common character
5. Summing all priorities

### Part 1: Finding Common Items Between Compartments

```advent2022/src/bin/day3.rs#L39-58
fn component_1(lines: &str) -> u32 {
    lines.lines()
        .map(|line| line.split_at( line.len()>>1 ) )  // Split each line in half
        .map(|(compa, compb)| {
            compa.chars()
                .filter(|&c| compb.find(c).is_some() )  // Find characters in both halves
                .collect::<HashSet<_>>()  // Collect unique common characters
        })
        .map(|set|
            set.into_iter()
                .map(|c|
                    match c {  // Calculate character priority
                        'a'..='z' => u32::from(c) - u32::from('a') + 1,   // 1-26 for lowercase
                        'A'..='Z' => u32::from(c) - u32::from('A') + 27,  // 27-52 for uppercase
                        _ => panic!("use only alphabetic characters")
                    }
                )
                .sum::<u32>()  // Sum priorities of common characters in a rucksack
        )
        .reduce(|sum, v| sum + v )  // Sum results from all rucksacks
        .unwrap_or_else(|| unreachable!())
}
```

This function handles Part 1 of the problem, finding items that appear in both compartments of each rucksack.

The solution works by:
1. Splitting each rucksack into two halves using `split_at`
2. Finding characters that appear in both halves using `filter`
3. Using a `HashSet` to ensure each common character is counted only once
4. Calculating the priority of each common character
5. Summing all priorities

## Implementation Notes

- **Bit Shift Operation**: `line.len()>>1` is a bit shift operation that divides the length by 2, efficiently splitting the rucksack into equal compartments.
- **HashSet Usage**: The use of HashSets eliminates duplicate characters in the results, ensuring each common character is counted exactly once.
- **Character Priority Calculation**: The solution uses character code arithmetic to calculate priorities, mapping 'a'-'z' to 1-26 and 'A'-'Z' to 27-52.
- **Functional Programming Style**: The implementation uses a functional programming style with method chaining, which makes the code concise and expressive.