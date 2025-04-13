# Day 9: Code

Below is the complete code for Day 9's solution, which simulates the motion of a rope with multiple knots.

## Full Solution

```rust,no_run,noplayground
{{#include ../../../src/bin/day9.rs}}
```

## Code Walkthrough

### Coordinate System

```rust,no_run,noplayground
{{#include ../../../src/bin/day9.rs:7:26}}
```

The `Coord` struct represents positions in 2D space. It includes:
- `x` and `y` coordinates as signed integers
- Implementation of `Sub` to calculate the distance between two coordinates
- A conversion from tuples for convenience
- Several derived traits, including `Hash` to allow using coordinates in a `HashSet`

### Movement Commands

```rust,no_run,noplayground
{{#include ../../../src/bin/day9.rs:27:39}}
```

These types represent movement commands:
- `Command` is an enum for the four possible directions
- `Step` combines a command with a distance

### Rope Link Implementation

```rust,no_run,noplayground
{{#include ../../../src/bin/day9.rs:40:97}}
```

The `Link` struct represents a single knot in the rope:
- `pos` is the current position of the knot
- `move_to` moves the knot directly in a cardinal direction
- `move_relative` implements the physical constraints of the rope, moving the knot based on its relation to the knot in front of it
- `position` returns the current position

The `move_relative` method is particularly detailed, handling all possible relative positions through pattern matching.

### Rope Chain Implementation

```rust,no_run,noplayground
{{#include ../../../src/bin/day9.rs:98:119}}
```

The `Chain` struct represents the entire rope:
- `links` is a vector of `Link` objects
- `new` creates a chain of a specified size, with all links starting at the same position
- `move_to` moves the head link directly and then propagates the movement through the chain

The `reduce` operation in `move_to` elegantly handles the chain of movement dependencies.

### Game Simulation

```rust,no_run,noplayground
{{#include ../../../src/bin/day9.rs:121:144}}
```

The `Game` struct manages the simulation:
- `rope` is the rope chain being simulated
- `unique` is a HashSet of coordinates visited by the tail
- `unique_positions` returns the number of unique positions visited
- `run` simulates all the movement steps and tracks unique tail positions

### Parsing Input

```rust,no_run,noplayground
{{#include ../../../src/bin/day9.rs:145:163}}
```

The `parse_commands` function converts the input text into a vector of `Step` objects by:
1. Splitting each line into parts
2. Converting the first part to a `Command`
3. Converting the second part to a distance
4. Creating a `Step` with the command and distance

### Main Function

```rust,no_run,noplayground
{{#include ../../../src/bin/day9.rs:165:}}
```

The main function:
1. Reads the input file
2. Parses it into commands
3. For Part 1: Creates a game with a 2-link chain and runs the simulation
4. For Part 2: Creates a game with a 10-link chain and runs the simulation
5. Prints the number of unique positions visited by the tail in each case

## Implementation Notes

- **Pattern Matching**: The solution makes extensive use of pattern matching, especially in the `move_relative` method
- **Functional Approach**: The solution uses functional programming techniques like `map`, `reduce`, and method chaining
- **Trait Implementations**: Custom traits like `Sub` and trait derivations make the code more expressive and type-safe
- **Type Safety**: Custom types like `Coord`, `Command`, and `Step` provide type safety and clarity
