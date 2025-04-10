# Utilities and Helpers

This section contains reusable utility functions and helper methods that are common across multiple Advent of Code solutions.

## Common Utilities

Advent of Code solutions often require similar operations, such as reading input files, parsing data structures, or performing mathematical operations. This section documents the utilities available in this project.

### File Reading

```rust
fn read_input(filename: &str) -> String {
    std::fs::read_to_string(filename)
        .expect("Failed to read input file")
}
```

### Iterator Helpers

```rust
// Split a collection into chunks of a specific size
fn chunks<T>(vec: Vec<T>, chunk_size: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    vec.chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect()
}

// Find the product of all elements in an iterator
fn product<I>(iter: I) -> I::Item
where
    I: Iterator,
    I::Item: std::ops::Mul<Output = I::Item> + From<u8>,
{
    iter.fold(I::Item::from(1), |acc, x| acc * x)
}
```

### 2D Grid Helpers

```rust
// Transpose a 2D grid
fn transpose<T>(grid: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    if grid.is_empty() {
        return vec![];
    }
    
    let cols = grid[0].len();
    let mut result = vec![vec![]; cols];
    
    for row in grid {
        for (i, item) in row.into_iter().enumerate() {
            result[i].push(item);
        }
    }
    
    result
}

// Rotate a 2D grid 90 degrees clockwise
fn rotate_clockwise<T>(grid: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    if grid.is_empty() {
        return vec![];
    }
    
    let rows = grid.len();
    let cols = grid[0].len();
    let mut result = vec![vec![]; cols];
    
    for i in 0..cols {
        let mut new_row = vec![];
        for j in (0..rows).rev() {
            new_row.push(grid[j][i].clone());
        }
        result[i] = new_row;
    }
    
    result
}
```

### Math Helpers

```rust
// Calculate the Greatest Common Divisor (GCD)
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Calculate the Least Common Multiple (LCM)
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

// Calculate the LCM of a list of numbers
fn lcm_of_list(numbers: &[usize]) -> usize {
    numbers.iter().fold(1, |acc, &num| lcm(acc, num))
}

// Calculate the Manhattan distance between two points
fn manhattan_distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}
```

## How to Use These Utilities

These utility functions can be imported into your solution files as needed. If they're part of a common module, you can import them like this:

```rust
use advent2022::utils::{read_input, lcm_of_list, manhattan_distance};
```

For more specific implementations or domain-specific utilities, refer to the relevant solution files where they might be defined locally.