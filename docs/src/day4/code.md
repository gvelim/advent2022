# Day 4: Code

Below is the complete code for Day 4's solution, which handles range containment and overlap checks.

## Full Solution

```rust,no_run,noplayground
{{#include ../../../src/bin/day4.rs}}
```

## Code Walkthrough

### Extending Ranges with a Trait

```rust,no_run,noplayground
{{#include ../../../src/bin/day4.rs:4:7}}
```

The solution defines a trait to extend Rust's `RangeInclusive` type with two new methods for checking containment relationships:
- `is_subset` - Checks if one range is fully contained within another
- `is_overlapping` - Checks if two ranges overlap at all

### Implementing the Trait

```rust,no_run,noplayground
{{#include ../../../src/bin/day4.rs:9:17}}
```

The trait is implemented generically for any `RangeInclusive<T>` where `T` supports partial ordering. This allows the solution to work with ranges of any comparable type, not just integers.

### Parsing Input

```rust,no_run,noplayground
{{#include ../../../src/bin/day4.rs:21:32}}
```

The parsing involves several steps:
1. Read the input file as a string
2. Split each line into parts using punctuation characters (hyphens and commas)
3. Convert each part to a `u32` number
4. Group the numbers into pairs of ranges using Rust's inclusive range syntax `a..=b`

### Part 1: Checking Subset Relationships

```rust,no_run,noplayground
{{#include ../../../src/bin/day4.rs:34:38}}
```

This part counts pairs where one range fully contains the other by applying the `is_subset` method and checking in both directions.

### Part 2: Checking Overlap Relationships

```rust,no_run,noplayground
{{#include ../../../src/bin/day4.rs:41:45}}
```

This part counts pairs where the ranges overlap at all by applying the `is_overlapping` method and checking in both directions.

## Implementation Notes

- **Trait Extensions**: This solution demonstrates Rust's powerful trait system by extending an existing type with new functionality.
- **Generic Programming**: The trait implementation works with any ordered type, not just the specific integers used in this problem.
- **Pattern Matching**: The solution uses Rust's pattern matching to destructure the parsed values into range pairs.
- **Error Handling**: The solution uses `expect` and `unwrap_or_else` for error handling, though a more robust solution might handle errors more gracefully.

The implementation is concise and idiomatic, leveraging Rust's type system and functional programming features to solve the problem elegantly.
