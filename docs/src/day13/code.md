# Day 13: Code

Below is the complete code for Day 13's solution, which parses and compares nested lists according to specific rules.

## Full Solution

```rust,no_run,noplayground
{{#include ../../../src/bin/day13.rs}}
```

## Code Walkthrough

### Data Structure for Packets

```rust,no_run,noplayground
{{#include ../../../src/bin/day13.rs:59:62}}
```

The solution uses an enum `ListItem` to represent the nested list structure of packets:
- `N(u8)` represents a number (limited to u8 for this problem)
- `L(Vec<ListItem>)` represents a list containing other items (which can be numbers or lists)

This recursive structure can represent any valid packet in the problem.

### Parsing Packets

```rust,no_run,noplayground
{{#include ../../../src/bin/day13.rs:73:117}}
```

The `FromStr` implementation uses a custom scanner to parse the input string into a `ListItem`:

1. It creates a `Scanner` with a peekable iterator over the input characters
2. The `parse_list` method recursively builds the list structure by:
   - Creating a new list when encountering `[`
   - Accumulating digits for numbers
   - Inserting numbers when reaching a comma or closing bracket
   - Breaking when reaching the end of the list
3. The method returns the parsed `ListItem`

### Item Insertion Helper

```rust,no_run,noplayground
{{#include ../../../src/bin/day13.rs:63:72}}
```

This helper method adds an item to a list or updates a number.

### Comparison Logic

```rust,no_run,noplayground
{{#include ../../../src/bin/day13.rs:126:159}}
```

The `Ord` implementation defines how to compare two `ListItem` values:

1. **List vs. List**: Compare elements one by one until finding a difference or reaching the end of a list
2. **List vs. Number**: Convert the number to a single-item list and retry comparison
3. **Number vs. List**: Convert the number to a single-item list and retry comparison
4. **Number vs. Number**: Use the built-in number comparison

This implements the comparison rules specified in the problem.

### Additional Trait Implementations

```rust,no_run,noplayground
{{#include ../../../src/bin/day13.rs:118:124}}
```

```rust,no_run,noplayground
{{#include ../../../src/bin/day13.rs:161:165}}
```

These implementations ensure that `ListItem` supports all the comparison operators and can be used in sorting operations.

### Debug Display

```rust,no_run,noplayground
{{#include ../../../src/bin/day13.rs:167:175}}
```

This implementation formats `ListItem` values for debugging, using Rust's `debug_list` for nice formatting of lists.

### Part 1: Finding Correctly Ordered Pairs

```rust,no_run,noplayground
{{#include ../../../src/bin/day13.rs:7:18}}
```

This function processes the input for Part 1:
1. Splits the input by double newlines to get pairs of packets
2. Parses each packet into a `ListItem`
3. Uses the `lt` comparison to check if pairs are in the right order
4. Keeps 1-based indices of correctly ordered pairs
5. Sums these indices

### Part 2: Sorting and Finding Divider Packets

```rust,no_run,noplayground
{{#include ../../../src/bin/day13.rs:20:44}}
```

This function processes the input for Part 2:
1. Defines the two divider packets (`[[2]]` and `[[6]]`)
2. Parses all packets from the input and adds the divider packets
3. Sorts all packets using the comparison logic
4. Finds the 1-based indices of the divider packets
5. Multiplies these indices to get the decoder key

### Main Function

```rust,no_run,noplayground
{{#include ../../../src/bin/day13.rs:46:57}}
```

The main function reads the input file and runs both parts of the problem.

## Implementation Notes

- **Recursive Data Structure**: The solution uses a recursive enum to represent the nested packet structure
- **Custom Parser**: The parser handles the specific format of the input without relying on external libraries
- **Trait Implementations**: The comparison logic is cleanly implemented using Rust's trait system
- **Functional Style**: The solution uses a functional programming style with iterators and method chaining
