# Day 3: Solution Explanation

## Approach

Day 3's problem involves finding common items across different sets and calculating their priorities. We need to:

1. **Part 1**: Find items that appear in both compartments of each rucksack
2. **Part 2**: Find the common item (badge) among each group of three elves

The key techniques we'll use are:

- String splitting to divide rucksacks into compartments
- HashSets for efficiently finding common elements
- Character mapping to calculate priorities

## Implementation Details

### Part 1: Finding Common Items in Compartments

The approach for Part 1 is:

1. Split each rucksack into two equal compartments
2. Find the characters that appear in both compartments
3. Calculate the priority of each common character
4. Sum the priorities

```rust
fn component_1(lines: &str) -> u32 {
    lines.lines()
        .map(|line| line.split_at(line.len()>>1))
        .map(|(compa, compb)| {
            compa.chars()
                .filter(|&c| compb.find(c).is_some())
                .collect::<HashSet<_>>()
        })
        .map(|set| set.into_iter().map(calculate_priority).sum::<u32>())
        .reduce(|sum, v| sum + v)
        .unwrap_or_else(|| unreachable!())
}
```

Key points about this implementation:

- `line.split_at(line.len()>>1)` divides the string into two equal halves
- `compb.find(c).is_some()` checks if character `c` appears in the second compartment
- We use a `HashSet` to ensure we count each common character only once

### Part 2: Finding Common Items Across Groups

The approach for Part 2 is:

1. Group the rucksacks into sets of three
2. For each group, find the characters that appear in all three rucksacks
3. Calculate the priority of each common character
4. Sum the priorities

```rust
fn component_2(lines:&str) -> u32 {
    lines.lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| {
            group.iter()
                .map(|a| a.chars().collect::<HashSet<_>>())
                .reduce(|a, b| a.intersection(&b).copied().collect::<HashSet<_>>())
                .unwrap_or_else(|| panic!("Ops!"))
        })
        .map(|set| set.into_iter().map(calculate_priority).sum::<u32>())
        .sum::<u32>()
}
```

Key points about this implementation:

- `.chunks(3)` splits the lines into groups of three
- We convert each rucksack into a `HashSet` of characters
- We use `reduce` with `intersection` to find characters common to all three rucksacks

### Priority Calculation

Both parts use the same logic to calculate priorities:

```rust
fn calculate_priority(c: char) -> u32 {
    match c {
        'a'..='z' => u32::from(c) - u32::from('a') + 1,   // 1-26
        'A'..='Z' => u32::from(c) - u32::from('A') + 27,  // 27-52
        _ => panic!("use only alphabetic characters")
    }
}
```

This function:
- Maps lowercase letters (a-z) to priorities 1-26
- Maps uppercase letters (A-Z) to priorities 27-52

## Optimization and Efficiency

### Time Complexity

- **Part 1**: O(n) where n is the total number of characters across all rucksacks
- **Part 2**: O(n) where n is the total number of characters across all rucksacks

The solution makes use of HashSets for efficient intersection operations.

### Space Complexity

- O(m) where m is the number of unique characters in the largest compartment/rucksack

### Alternative Approaches

#### Bitsets for Character Tracking

An alternative approach could use bitsets to track character presence:

```rust
fn using_bitsets(lines: &str) -> u32 {
    lines.lines()
        .map(|line| {
            let half_len = line.len() / 2;
            let first_half = &line[0..half_len];
            let second_half = &line[half_len..];
            
            let mut first_set = 0u64;
            let mut second_set = 0u64;
            
            for c in first_half.chars() {
                let bit = if c.is_lowercase() {
                    1u64 << (c as u8 - b'a')
                } else {
                    1u64 << (c as u8 - b'A' + 26)
                };
                first_set |= bit;
            }
            
            for c in second_half.chars() {
                let bit = if c.is_lowercase() {
                    1u64 << (c as u8 - b'a')
                } else {
                    1u64 << (c as u8 - b'A' + 26)
                };
                second_set |= bit;
            }
            
            let common = first_set & second_set;
            common.trailing_zeros() + 1
        })
        .sum()
}
```

This approach would be more memory-efficient but slightly more complex to implement.

## Conclusion

The solution uses Rust's powerful iterators and collection types to create a clean, functional implementation. The use of HashSets makes finding common elements efficient, while the string manipulation functions allow for straightforward parsing of the input.