# Day 15: Code

Below is the complete code for Day 15's solution, which analyzes sensor coverage to find positions where beacons cannot be present.

## Full Solution

```rust,no_run,noplayground
{{#include ../../../src/bin/day15.rs}}
```

## Code Walkthrough

### Core Data Structures

```rust,no_run,noplayground
{{#include ../../../src/bin/day15.rs:134:143}}
```

The `Coord` struct represents a 2D coordinate with x and y values. It implements several traits to make it comparable, hashable, and printable.

```rust,no_run,noplayground
{{#include ../../../src/bin/day15.rs:104:109}}
```

The `Sensor` struct contains information about a sensor's position, its closest beacon's position, and the Manhattan distance between them.

```rust,no_run,noplayground
{{#include ../../../src/bin/day15.rs:48:50}}
```

The `Area` struct is a container for all sensors in the input.

### Sensor Coverage Calculation

```rust,no_run,noplayground
{{#include ../../../src/bin/day15.rs:110:123}}
```

This method calculates the x-coordinate range that a sensor can cover at a specific y-coordinate. It:
1. Calculates the vertical distance to the target line
2. If this distance is within the sensor's range, calculates the horizontal range
3. Returns the range, or `None` if the line is out of range

### Analyzing Sensor Coverage on a Row

```rust,no_run,noplayground
{{#include ../../../src/bin/day15.rs:75:101}}
```

This method aggregates coverage from all sensors on a specific row:
1. Collects ranges from all sensors that cover the specified row
2. Sorts the ranges by their start position
3. Merges overlapping ranges using a `reduce` operation
4. Returns a vector of non-overlapping ranges representing total coverage

### Finding Beacons on a Row

```rust,no_run,noplayground
{{#include ../../../src/bin/day15.rs:72:74}}
```

This method identifies all beacons located on a specific row.

### Parsing Input

```rust,no_run,noplayground
{{#include ../../../src/bin/day15.rs:52:71}}
```

This method parses the input text into `Sensor` objects by:
1. Splitting each line into parts
2. Filtering out non-numeric parts
3. Converting numeric strings to integers
4. Constructing sensors with their positions, beacon positions, and distances

### Main Function

```rust,no_run,noplayground
{{#include ../../../src/bin/day15.rs:21:46}}
```

The main function:

1. Reads and parses the input file
2. For Part 1:
   - Gets the sensor coverage on row 2000000
   - Identifies beacons already on that row
   - Calculates the number of positions that cannot contain a beacon
3. For Part 2:
   - Checks each row in the search area (0 to 4000000)
   - Finds a row where the coverage is split with a gap of exactly one position
   - Calculates the tuning frequency of the distress beacon

The key insight for Part 2 is that the distress beacon must be in a position that is just outside the range of multiple sensors, which appears as a gap in the coverage.

## Implementation Notes

- **Range Representation**: The solution uses `RangeInclusive<isize>` to represent coverage ranges efficiently
- **Merge Algorithm**: Overlapping ranges are merged, significantly reducing the number of ranges needed to represent coverage
- **Efficient Searching**: The solution for Part 2 efficiently finds the gap by examining rows with split coverage rather than checking every position
