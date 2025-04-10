# Parsing Input

Efficient input parsing is essential for many Advent of Code challenges. This page discusses common parsing patterns used throughout the solutions.

## String Split Patterns

Many inputs can be parsed using Rust's powerful string splitting methods:

```rust
// Split by empty lines (groups of lines)
let groups = input.split("\n\n");

// Split by lines
let lines = input.lines();

// Split line by delimiter
let parts = line.split(',');
```

## Parsing Numeric Values

Converting strings to numbers is a common operation:

```rust
// Parse string to integer
let num: i32 = "42".parse().unwrap();

// Parse with error handling
let num = "42".parse::<i32>().unwrap_or(0);

// Convert line of numbers separated by spaces
let numbers: Vec<i32> = line
    .split_whitespace()
    .filter_map(|s| s.parse().ok())
    .collect();
```

## Complex Formats with Regular Expressions

For more complex formats, regular expressions are invaluable:

```rust
use regex::Regex;

// Example: parsing "move 3 from 2 to 1"
let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
if let Some(caps) = re.captures(line) {
    let count = caps[1].parse::<usize>().unwrap();
    let from = caps[2].parse::<usize>().unwrap();
    let to = caps[3].parse::<usize>().unwrap();
    // Use the parsed values
}
```

## Structured Data Parsing

For converting input into structured data types:

```rust
struct Point {
    x: i32,
    y: i32,
}

// Parse a line like "x=10, y=20"
fn parse_point(line: &str) -> Point {
    let parts: Vec<&str> = line.split(',').collect();
    let x_part = parts[0].trim().strip_prefix("x=").unwrap();
    let y_part = parts[1].trim().strip_prefix("y=").unwrap();
    
    Point {
        x: x_part.parse().unwrap(),
        y: y_part.parse().unwrap(),
    }
}
```

## Custom Parsers

For very complex inputs, building custom parsers or using parser combinator libraries like `nom` can be effective:

```rust
use nom::{IResult, character::complete::digit1, combinator::map_res};

// Parse a decimal number
fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}
```

## Best Practices

1. **Handle errors gracefully** - Use `filter_map` or proper error handling instead of `.unwrap()` in production code
2. **Parse once, use many times** - Parse input into appropriate data structures first, then process
3. **Use type annotations** - Be explicit about types to help with debugging
4. **Consider performance** - For large inputs, avoid inefficient parsing methods