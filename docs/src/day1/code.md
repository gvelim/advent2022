# Day 1: Code

Below is the complete code for Day 1's solution. The solution uses a `BinaryHeap` to efficiently track the elves with the most calories.

## Full Solution

```advent2022/src/bin/day1.rs#L1-24
use std::collections::BinaryHeap;
use std::str::FromStr;

fn main() {

    let fs = std::fs::read_to_string("./src/bin/day1_input.txt").unwrap_or_else(|e| panic!("{e}"));

    let out = fs.split("\n\n")
        .map(|e| e.split('\n'))
        .map(|v|
            v.filter_map(|e| u64::from_str(e).ok() ).collect::<Vec<u64>>()
        )
        .fold(BinaryHeap::new(), |mut out, v|{
            out.push(v.iter().sum::<u64>());
            out
        });
    println!("Q1: {:?}",out.iter().take(3).collect::<Vec<_>>());
    println!("Q2: {:?}",out.iter().take(3).sum::<u64>());

}
```

## Code Walkthrough

### Imports

```advent2022/src/bin/day1.rs#L1-2
use std::collections::BinaryHeap;
use std::str::FromStr;
```

The solution imports:
- `BinaryHeap` - A max-heap implementation for efficiently finding the largest elements
- `FromStr` - A trait for parsing strings into other types

### Input Parsing and Solution

```advent2022/src/bin/day1.rs#L6-15
    let fs = std::fs::read_to_string("./src/bin/day1_input.txt").unwrap_or_else(|e| panic!("{e}"));

    let out = fs.split("\n\n")
        .map(|e| e.split('\n'))
        .map(|v|
            v.filter_map(|e| u64::from_str(e).ok() ).collect::<Vec<u64>>()
        )
        .fold(BinaryHeap::new(), |mut out, v|{
            out.push(v.iter().sum::<u64>());
            out
        });
```

The code:
1. Reads the input file as a string
2. Splits the input by double newlines (`\n\n`) to separate each elf's inventory
3. For each elf, splits their inventory by single newlines
4. Parses each line into a `u64` integer, filtering out any lines that can't be parsed
5. Collects each elf's calories into a vector
6. Uses `fold` to build a `BinaryHeap` containing the sum of calories for each elf

### Output

```advent2022/src/bin/day1.rs#L16-17
    println!("Q1: {:?}",out.iter().take(3).collect::<Vec<_>>());
    println!("Q2: {:?}",out.iter().take(3).sum::<u64>());
```

The code outputs:
1. For part 1: The top three calorie counts (the first one is the answer to part 1)
2. For part 2: The sum of the top three calorie counts

## Implementation Notes

- The solution leverages Rust's `BinaryHeap` which is a max-heap, automatically giving us the largest elements first
- Instead of sorting the entire list of elf calorie totals, this approach is more efficient because it directly gives us the largest values first
- The solution combines both part 1 and part 2 into a single processing pipeline