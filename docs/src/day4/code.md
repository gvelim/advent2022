# Day 4: Code

Below is the complete code for Day 4's solution, which handles range containment and overlap checks.

## Full Solution

```advent2022/src/bin/day4.rs#L1-44
use std::ops::RangeInclusive;
use std::str::FromStr;

trait InclusiveRangeExt {
    fn is_subset(&self, other: &Self) -> bool;
    fn is_overlapping(&self, other: &Self) -> bool;
}

impl<T> InclusiveRangeExt for RangeInclusive<T>
    where T : PartialOrd {
    fn is_subset(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }
    fn is_overlapping(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

fn main() {

    let data = std::fs::read_to_string("src/bin/day4_input.txt").expect("Ops! Cannot read file");
    let pairs = data.lines()
        .map(|line|
            line.split(|c:char| c.is_ascii_punctuation())
                .map(|c| u32::from_str(c).unwrap_or_else(|e| panic!("{e}")) )
                .collect::<Vec<_>>()
        )
        .map(|pair| {
            let [a, b, c, d] = pair[..] else { panic!("") };
            ((a..=b), (c..=d))
        })
        .collect::<Vec<_>>();

    let out = pairs.iter()
        .filter(|(a,b)|
            a.is_subset(b) || b.is_subset(a)
        )
        .count();
    println!("Component 1 = {out}");

    let out = pairs.iter()
        .filter(|(a,b)|
            a.is_overlapping(b) || b.is_overlapping(a)
        )
        .count();
    println!("Component 2 = {out}");
}
```

## Code Walkthrough

### Extending Ranges with a Trait

```advent2022/src/bin/day4.rs#L4-7
trait InclusiveRangeExt {
    fn is_subset(&self, other: &Self) -> bool;
    fn is_overlapping(&self, other: &Self) -> bool;
}
```

The solution defines a trait to extend Rust's `RangeInclusive` type with two new methods for checking containment relationships:
- `is_subset` - Checks if one range is fully contained within another
- `is_overlapping` - Checks if two ranges overlap at all

### Implementing the Trait

```advent2022/src/bin/day4.rs#L9-16
impl<T> InclusiveRangeExt for RangeInclusive<T>
    where T : PartialOrd {
    fn is_subset(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }
    fn is_overlapping(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}
```

The trait is implemented generically for any `RangeInclusive<T>` where `T` supports partial ordering. This allows the solution to work with ranges of any comparable type, not just integers.

### Parsing Input

```advent2022/src/bin/day4.rs#L20-31
let data = std::fs::read_to_string("src/bin/day4_input.txt").expect("Ops! Cannot read file");
let pairs = data.lines()
    .map(|line|
        line.split(|c:char| c.is_ascii_punctuation())
            .map(|c| u32::from_str(c).unwrap_or_else(|e| panic!("{e}")) )
            .collect::<Vec<_>>()
    )
    .map(|pair| {
        let [a, b, c, d] = pair[..] else { panic!("") };
        ((a..=b), (c..=d))
    })
    .collect::<Vec<_>>();
```

The parsing involves several steps:
1. Read the input file as a string
2. Split each line into parts using punctuation characters (hyphens and commas)
3. Convert each part to a `u32` number
4. Group the numbers into pairs of ranges using Rust's inclusive range syntax `a..=b`

### Part 1: Checking Subset Relationships

```advent2022/src/bin/day4.rs#L33-37
let out = pairs.iter()
    .filter(|(a,b)|
        a.is_subset(b) || b.is_subset(a)
    )
    .count();
```

This part counts pairs where one range fully contains the other by applying the `is_subset` method and checking in both directions.

### Part 2: Checking Overlap Relationships

```advent2022/src/bin/day4.rs#L39-43
let out = pairs.iter()
    .filter(|(a,b)|
        a.is_overlapping(b) || b.is_overlapping(a)
    )
    .count();
```

This part counts pairs where the ranges overlap at all by applying the `is_overlapping` method and checking in both directions.

## Implementation Notes

- **Trait Extensions**: This solution demonstrates Rust's powerful trait system by extending an existing type with new functionality.
- **Generic Programming**: The trait implementation works with any ordered type, not just the specific integers used in this problem.
- **Pattern Matching**: The solution uses Rust's pattern matching to destructure the parsed values into range pairs.
- **Error Handling**: The solution uses `expect` and `unwrap_or_else` for error handling, though a more robust solution might handle errors more gracefully.

The implementation is concise and idiomatic, leveraging Rust's type system and functional programming features to solve the problem elegantly.