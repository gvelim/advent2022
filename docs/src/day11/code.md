# Day 11: Code

Below is the complete code for Day 11's solution, which simulates monkeys passing items with worry levels.

## Full Solution

```rust,no_run,noplayground
{{#include ../../../src/bin/day11.rs}}
```

## Code Walkthrough

### Data Types and Structures

```rust,no_run,noplayground
{{#include ../../../src/bin/day11.rs:45:61}}
```

The solution defines:

- `WorryType` as `u64` to handle large worry levels
- An `Operation` enum to represent addition or multiplication operations
- A `Monkey` struct with properties for:
  - `name`: The monkey's index
  - `items`: A queue of worry levels for items the monkey is holding
  - `op`: The operation the monkey performs on items
  - `test`: The divisibility test value
  - `send`: A tuple with indices of monkeys to throw to (true case, false case)
  - `inspect`: A counter for the number of inspections

### Monkey Behavior

```rust,no_run,noplayground
{{#include ../../../src/bin/day11.rs:62:115}}
```

The `Monkey` implementation includes methods for:

- `parse_text`: Parsing all monkeys from the input
- `catch`: Adding an item to the monkey's queue
- `throw`: Determining which monkey to throw to based on the test
- `observe`: Processing a single item:
  - Incrementing the inspection counter
  - Taking an item from the front of the queue
  - Applying modulo to manage worry levels
  - Applying the operation to update the worry level
  - Determining which monkey to throw to
- `observe_all`: Processing all items a monkey is holding
- `inspections`: Returning the inspection count

### Parsing Logic

```rust,no_run,noplayground
{{#include ../../../src/bin/day11.rs:116:190}}
```

The parsing logic includes:

- A `Default` implementation for `Monkey` providing initial values
- An implementation of `FromStr` for parsing monkey specifications
- Logic for parsing each line of the monkey description based on field names
- Special handling for operations that reference "old" (the current worry level)

### Main Simulation

```rust,no_run,noplayground
{{#include ../../../src/bin/day11.rs:6:43}}
```

The main simulation logic:

1. Reads and parses the input
2. Calculates the product of all test divisors to manage worry levels
3. Creates queues for passing items between monkeys
4. Runs the simulation for 10,000 rounds:
   - For each monkey, processes all items it's holding
   - Updates worry levels and determines target monkeys
   - Uses queues to pass items between monkeys
5. Sorts monkeys by inspection count and calculates the "monkey business" level

## Implementation Notes

- **Chinese Remainder Theorem**: The solution uses modular arithmetic with `div_product` to keep worry levels manageable while preserving divisibility properties
- **Queue-based Communication**: Items are passed between monkeys using queues, allowing each monkey to process all its items before moving to the next monkey
- **Functional Programming Style**: The code uses functional programming patterns like `map`, `fold`, and method chaining
