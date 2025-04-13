# Day 8: Code

Below is the complete code for Day 8's solution, which analyzes a grid of trees to determine visibility and scenic scores.

## Full Solution

```rust,no_run,noplayground
{{#include ../../../src/bin/day8.rs}}
```

## Code Walkthrough

### Core Data Structures

#### Coordinate System

```rust,no_run,noplayground
{{#include ../../../src/bin/day8.rs:4:13}}
```

The `Coord` struct represents a position in the grid with x and y coordinates. The `From<(usize,usize)>` implementation allows easy conversion from coordinate tuples.

#### Grid Implementation

```rust,no_run,noplayground
{{#include ../../../src/bin/day8.rs:15:44}}
```

The `Grid<T>` struct is a generic container that stores a 2D grid as a flat vector. It provides methods for:

- Creating a new grid with default values
- Checking if coordinates are within bounds
- Accessing grid elements by coordinates (both immutably and mutably)

### Visibility Analysis

```rust,no_run,noplayground
{{#include ../../../src/bin/day8.rs:46:78}}
```

The `Visibility` struct manages determining which trees are visible:

- It keeps a reference to the forest grid and a boolean grid to track visibility
- `count_visible()` counts the number of visible trees
- `scan_visibility()` scans along provided coordinate sequences, marking trees as visible if they're taller than all previous trees in the scan

### Scenic Score Calculation

```rust,no_run,noplayground
{{#include ../../../src/bin/day8.rs:79:114}}
```

The `Scenic` struct handles calculating scenic scores:

- `scenic_score_dir()` calculates the viewing distance in a specific direction using an iterator that continues until it reaches the edge or a blocking tree
- `scenic_score()` combines the viewing distances in all four directions by multiplying them together

### Direction Scanning Utilities

```rust,no_run,noplayground
{{#include ../../../src/bin/day8.rs:138:}}
```

These utility functions generate coordinate sequences for scanning the grid in different directions:
- `left_to_right`: Scans each row from left to right
- `right_to_left`: Scans each row from right to left
- `top_to_bottom`: Scans each column from top to bottom
- `bottom_to_up`: Scans each column from bottom to top

### Main Function and Input Parsing

```rust,no_run,noplayground
{{#include ../../../src/bin/day8.rs:116:137}}
```

The main function:
1. Reads the input file
2. Parses it into a grid
3. For Part 1: Scans the grid from all four directions and counts the visible trees
4. For Part 2: Calculates the scenic score for every tree and finds the maximum

The `parse_forest` function converts the input string into a grid of tree heights.

## Implementation Notes

- **Generic Grid**: The solution uses a generic grid implementation that can store any type of data, making it flexible for different use cases
- **Fluent Interface**: The visibility scanning uses a fluent interface with method chaining for concise code
- **Iterator Usage**: The solution makes extensive use of iterators, including infinite iterators with `map_while` for clean, efficient code
- **Coordinate Handling**: The custom `Coord` type with `From` trait implementation makes coordinate handling safer and more expressive
