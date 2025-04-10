# Day 1: Solution Explanation

## Approach

Day 1's problem requires us to parse a list of calorie values grouped by elves, calculate the total calories per elf, and then find either the maximum value (part 1) or the sum of the top three values (part 2).

### Step 1: Parse the Input

The input format consists of groups of numbers separated by blank lines. Each group represents the food items carried by a single elf. We need to:

1. Split the input by blank lines to get each elf's inventory
2. For each elf's inventory, parse the individual calorie values and sum them

### Step 2: Find the Maximum (Part 1)

Once we have the total calories for each elf, we simply find the maximum value among them.

### Step 3: Find the Sum of Top Three (Part 2)

To find the sum of the top three values:

1. Sort the list of calorie sums in descending order
2. Take the first three elements
3. Sum them

## Implementation Details

### Parsing the Input

We use Rust's string splitting capabilities to parse the input:

```rust
fn parse_input(input: &str) -> Vec<u32> {
    input
        .split("\n\n") // Split by blank lines to get each elf's inventory
        .map(|elf| {
            elf.lines() // Split each elf's inventory by lines
                .filter_map(|line| line.parse::<u32>().ok()) // Parse each line to a number
                .sum() // Sum the calories for each elf
        })
        .collect() // Collect into a vector of total calories per elf
}
```

### Solving Part 1

Finding the maximum value is straightforward:

```rust
fn part1(calories: &[u32]) -> u32 {
    *calories.iter().max().unwrap_or(&0)
}
```

### Solving Part 2

For part 2, we sort the values and sum the top three:

```rust
fn part2(calories: &[u32]) -> u32 {
    let mut sorted = calories.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order
    sorted.iter().take(3).sum() // Sum the top three values
}
```

## Time and Space Complexity

- **Time Complexity**: O(n log n) where n is the number of elves, due to the sorting operation in part 2
- **Space Complexity**: O(n) for storing the calorie totals for each elf

## Alternative Approaches

### Using a Priority Queue

Instead of sorting the entire list for part 2, we could use a min-heap of size 3 to keep track of the top three values:

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn part2_with_heap(calories: &[u32]) -> u32 {
    let mut heap = BinaryHeap::new();
    
    for &calorie in calories {
        heap.push(Reverse(calorie));
        if heap.len() > 3 {
            heap.pop();
        }
    }
    
    heap.into_iter().map(|Reverse(cal)| cal).sum()
}
```

This approach has a time complexity of O(n log 3) ≈ O(n), which is more efficient than sorting the entire list.