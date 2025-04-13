# Day 12: Code

Below is the complete code for Day 12's solution, which implements a path-finding algorithm to navigate a heightmap.

## Full Solution

```rust,no_run,noplayground
{{#include ../../../src/bin/day12.rs}}
```

## Code Walkthrough

### Core Data Structures

```rust,no_run,noplayground
{{#include ../../../src/bin/day12.rs:257:257}}
```

The solution uses an `ElevationGrid` wrapper around a generic `Grid<u8>` to represent the heightmap. Elevation values are stored as bytes, with special values for the start and end positions.

```rust,no_run,noplayground
{{#include ../../../src/bin/day12.rs:182:186}}
```

The `PathSearch` struct manages the breadth-first search algorithm, tracking:
- A queue of coordinates to explore
- A grid marking visited positions and their parent positions
- A vector to store the final path

### Input Parsing

```rust,no_run,noplayground
{{#include ../../../src/bin/day12.rs:158:180}}
```

The parsing function:
1. Creates a grid of the appropriate size
2. Processes each character in the input:
   - 'S' (start) is mapped to elevation 0 and its position is stored
   - 'E' (end) is mapped to elevation 27 and its position is stored
   - Letters 'a' to 'z' are mapped to values 1 to 26
3. Returns the grid and the start and end positions

### Breadth-First Search Implementation

```rust,no_run,noplayground
{{#include ../../../src/bin/day12.rs:275:316}}
```

The BFS algorithm:
1. Initializes a `PathSearch` instance with the grid dimensions
2. Adds the start position to the queue
3. Processes positions from the queue until finding one that satisfies the goal condition
4. For each position, visits neighboring positions that satisfy the elevation constraint
5. When the goal is reached, reconstructs the path by following parent pointers

### Elevation Reversal for Part 2

```rust,no_run,noplayground
{{#include ../../../src/bin/day12.rs:262:269}}
```

This method reverses the elevation values, which allows solving Part 2 by starting from the end position and searching for the closest square with elevation 'a'.

### Path Visualization

```rust,no_run,noplayground
{{#include ../../../src/bin/day12.rs:270:274}}
```

This method creates a new grid highlighting only the cells in the path, then prints it to the console.

### Interactive Visualization

The solution includes a sophisticated interactive visualization using the bracket-lib library. This allows exploring the map and watching the path-finding algorithm in action.

```rust,no_run,noplayground
{{#include ../../../src/bin/day12.rs:32:49}}
```

This setup creates a visualization window with multiple layers and implements an interactive application with different levels.

## Main Solution Flow

```rust,no_run,noplayground
{{#include ../../../src/bin/day12.rs:11:29}}
```

The main solution:

1. Parses the input into a grid and identifies start and end positions
2. For Part 1:
   - Finds the shortest path from start to end
   - Visualizes the path
3. For Part 2:
   - Reverses elevation values
   - Finds the shortest path from end to any position with elevation 'a'
   - Visualizes the path

## Implementation Notes

- **Goal Function**: The solution uses a closure as a goal function, making it flexible for both parts
- **Path Reconstruction**: The algorithm reconstructs the path by storing parent pointers in the `visited` grid
- **Interactive Visualization**: The solution includes a sophisticated visualization using bracket-lib
- **Functional Programming Style**: The code makes extensive use of iterators and functional programming patterns
