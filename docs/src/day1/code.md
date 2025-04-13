# Day 1: Code

Below is the complete code for Day 1's solution. The solution uses a `BinaryHeap` to efficiently track the elves with the most calories.

## Full Solution

```rust,no_run,no_playground
{{#include ../../../src/bin/day1.rs}}
```

## Code Walkthrough

### Imports

```rust,no_run,no_playground
{{#include ../../../src/bin/day1.rs:1:2}}
```

The solution imports:
- `BinaryHeap` - A max-heap implementation for efficiently finding the largest elements
- `FromStr` - A trait for parsing strings into other types

### Input Parsing and Solution

```rust,no_run,no_playground
{{#include ../../../src/bin/day1.rs:6:16}}
```

The code:
1. Reads the input file as a string
2. Splits the input by double newlines (`\n\n`) to separate each elf's inventory
3. For each elf, splits their inventory by single newlines
4. Parses each line into a `u64` integer, filtering out any lines that can't be parsed
5. Collects each elf's calories into a vector
6. Uses `fold` to build a `BinaryHeap` containing the sum of calories for each elf

### Output

```rust,no_run,no_playground
{{#include ../../../src/bin/day1.rs:17:18}}
```

The code outputs:
1. For part 1: The top three calorie counts (the first one is the answer to part 1)
2. For part 2: The sum of the top three calorie counts

## Implementation Notes

- The solution leverages Rust's `BinaryHeap` which is a max-heap, automatically giving us the largest elements first
- Instead of sorting the entire list of elf calorie totals, this approach is more efficient because it directly gives us the largest values first
- The solution combines both part 1 and part 2 into a single processing pipeline
