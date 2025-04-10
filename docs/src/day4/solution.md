# Day 4: Solution Explanation

## Approach

Day 4's problem involves working with ranges and determining relationships between them. We need to check:

1. **Part 1**: Whether one range fully contains the other (subset relationship)
2. **Part 2**: Whether two ranges overlap at all (intersection relationship)

The core of the solution is to extend Rust's `RangeInclusive` type with functionality to check for these two conditions.

## Implementation Details

### Range Extension Trait

The most elegant part of this solution is defining a trait to extend the functionality of Rust's built-in `RangeInclusive` type:

```rust
trait InclusiveRangeExt {
    fn is_subset(&self, other: &Self) -> bool;
    fn is_overlapping(&self, other: &Self) -> bool;
}
```

This trait adds two methods to `RangeInclusive`:
- `is_subset` - Checks if the other range is fully contained within this range
- `is_overlapping` - Checks if this range overlaps with the other range at all

### Implementing the Trait

The implementation uses the `contains` method that's built into `RangeInclusive`:

```rust
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

The generic implementation works for any type `T` that can be compared (`PartialOrd`), which includes the integers we're using in this problem.

### Parsing the Input

The input consists of pairs of ranges in the format `a-b,c-d`. We parse this into pairs of `RangeInclusive<u32>`:

```rust
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

The parsing works by:
1. Splitting each line by punctuation characters (hyphens and commas)
2. Converting each part to a `u32`
3. Creating a pair of ranges using the inclusive range syntax `a..=b`

### Solving Part 1: Full Containment

With our ranges parsed and the extension trait implemented, solving Part 1 is straightforward:

```rust
let out = pairs.iter()
    .filter(|(a,b)|
        a.is_subset(b) || b.is_subset(a)
    )
    .count();
```

We check each pair to see if either range is a subset of the other, and count the number of pairs that satisfy this condition.

### Solving Part 2: Overlapping

Similarly, for Part 2, we count pairs where ranges overlap at all:

```rust
let out = pairs.iter()
    .filter(|(a,b)|
        a.is_overlapping(b) || b.is_overlapping(a)
    )
    .count();
```

## Alternative Solutions

### Direct Range Comparison

Instead of using a trait extension, we could have compared range endpoints directly:

```rust
// Check if range a fully contains range b
fn is_subset(a: &(u32, u32), b: &(u32, u32)) -> bool {
    a.0 <= b.0 && a.1 >= b.1
}

// Check if ranges a and b overlap
fn is_overlapping(a: &(u32, u32), b: &(u32, u32)) -> bool {
    a.0 <= b.1 && a.1 >= b.0
}
```

This approach would use tuples instead of ranges, which is simpler but less expressive.

### Using Set Operations

Another approach could model ranges as sets and use set operations:

```rust
use std::collections::HashSet;

fn range_to_set(start: u32, end: u32) -> HashSet<u32> {
    (start..=end).collect()
}

fn is_subset(a: &HashSet<u32>, b: &HashSet<u32>) -> bool {
    a.is_subset(b) || b.is_subset(a)
}

fn is_overlapping(a: &HashSet<u32>, b: &HashSet<u32>) -> bool {
    !a.is_disjoint(b)
}
```

However, this would be less efficient for large ranges due to the memory required to store every integer in each range.

## Time and Space Complexity

- **Time Complexity**: O(n) where n is the number of range pairs, since we process each pair once with constant-time operations.
- **Space Complexity**: O(n) to store the parsed pairs.

## Conclusion

This solution demonstrates how Rust's trait system can be used to extend existing types with new functionality. By using trait extensions, we achieve an elegant and readable solution that clearly expresses the problem's domain concepts.