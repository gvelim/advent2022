{{REWRITTEN_CODE}}
# Day 5: Code

Below is the complete code for Day 5's solution, which handles rearranging stacks of crates.

## Full Solution

```rust,no_run,noplayground
{{#include ../../../src/bin/day5.rs}}
```

## Code Walkthrough

### Data Structures

The solution uses two main structures:

1. **Move** - Represents a single move instruction:

```rust,no_run,noplayground
{{#include ../../../src/bin/day5.rs:5:10}}
```

2. **Buckets** - Represents the stacks of crates:

```rust,no_run,noplayground
{{#include ../../../src/bin/day5.rs:34:38}}
```

### Parsing

#### Parsing Move Instructions

The `FromStr` trait implementation for `Move` allows parsing strings like "move 1 from 2 to 1":

```rust,no_run,noplayground
{{#include ../../../src/bin/day5.rs:11:26}}
```

The helper method `parse_moves` processes multiple move instructions:

```rust,no_run,noplayground
{{#include ../../../src/bin/day5.rs:27:32}}
```

#### Parsing Initial Crate Configuration

The `new` method of `Buckets` parses the initial crate configuration:

```rust,no_run,noplayground
{{#include ../../../src/bin/day5.rs:40:61}}
```

This method works by:
1. Reading the input in reverse order (bottom to top)
2. Splitting each line into characters
3. Filtering out non-alphanumeric characters (keeping only crate letters)
4. Building each stack based on character positions

### Crane Operations

#### CrateMover 9000: Moving One at a Time

```rust,no_run,noplayground
{{#include ../../../src/bin/day5.rs:62:70}}
```

This method moves crates one at a time, popping from the source stack and pushing to the destination.

#### CrateMover 9001: Moving Multiple at Once

```rust,no_run,noplayground
{{#include ../../../src/bin/day5.rs:71:79}}
```

This method moves multiple crates at once, preserving their order through a double-reversal process.

### Getting the Final Result

```rust,no_run,noplayground
{{#include ../../../src/bin/day5.rs:80:85}}
```

This method retrieves the top crate from each stack and combines them into a string.

### Main Function

```rust,no_run,noplayground
{{#include ../../../src/bin/day5.rs:91:}}
```

The main function:
1. Reads the input file
2. Splits it into the initial configuration and move instructions
3. Creates the stacks and parses the moves
4. Applies the CrateMover 9000 rules and prints the result (Part 1)
5. Applies the CrateMover 9001 rules and prints the result (Part 2)

## Implementation Notes

- **Functional Programming Style**: The solution makes extensive use of iterators and functional programming patterns.
- **Key Transformation**: The `get_keys` method handles the conversion between 1-based indexing (in the input) and 0-based indexing (in the code).
- **Parsing Approach**: The solution parses the visual representation of the crates by reading from the bottom up and using character positions.
- **Double Reversal**: The CrateMover 9001 uses a double reversal technique to preserve the order of crates when moving multiple at once.
