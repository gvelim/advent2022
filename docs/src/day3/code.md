# Day 3: Code

Below is the complete code for Day 3, which solves the Rucksack Reorganization problem.

## Full Solution

```rust,no_run,noplayground
{{#include ../../../src/bin/day3.rs}}
```

## Code Walkthrough

### Imports and Setup

```rust,no_run,noplayground
{{#include ../../../src/bin/day3.rs::15}}
```

The solution imports the `HashSet` collection type which is used to efficiently find common elements. The main function reads the input file and calls the two component functions that solve parts 1 and 2 of the problem.

### Part 1: Finding Common Items Between Compartments

```rust,no_run,noplayground
{{#include ../../../src/bin/day3.rs:43:}}
```

This function handles Part 1 of the problem, finding items that appear in both compartments of each rucksack.

The solution works by:
1. Splitting each rucksack into two halves using `split_at`
2. Finding characters that appear in both halves using `filter`
3. Using a `HashSet` to ensure each common character is counted only once
4. Calculating the priority of each common character
5. Summing all priorities

### Part 2: Finding Group Badges

```rust,no_run,noplayground
{{#include ../../../src/bin/day3.rs:17:41}}
```

This function handles Part 2 of the problem, finding the common item (badge) among each group of three elves.

The solution works by:
1. Grouping rucksacks into sets of three using `chunks(3)`
2. For each group, converting each rucksack into a `HashSet` of characters
3. Using `reduce` with `intersection` to find characters common to all three rucksacks
4. Calculating the priority of the common character
5. Summing all priorities

## Implementation Notes

- **Bit Shift Operation**: `line.len()>>1` is a bit shift operation that divides the length by 2, efficiently splitting the rucksack into equal compartments.
- **HashSet Usage**: The use of HashSets eliminates duplicate characters in the results, ensuring each common character is counted exactly once.
- **Character Priority Calculation**: The solution uses character code arithmetic to calculate priorities, mapping 'a'-'z' to 1-26 and 'A'-'Z' to 27-52.
- **Functional Programming Style**: The implementation uses a functional programming style with method chaining, which makes the code concise and expressive.
