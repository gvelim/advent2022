# Day 14: Code

Below is the complete code explanation for Day 14's solution, which simulates falling sand in a cave system with rock formations.

## Code Structure

The solution is quite extensive and uses several key components:

1. A `Board<T>` struct to represent the cave grid
2. A `Material` enum for different types of material (rock, sand, air)
3. A `Grain` struct to track individual sand units
4. A `Painter` helper to draw rock formations
5. Simulation logic for falling sand
6. Visualization components using bracket-lib

## Key Components

### Board and Materials

The cave is represented by a `Board` struct with a hashmap grid:

```rust,no_run,noplayground
{{#include ../../../src/bin/day14.rs:461:467}}
```

The materials in the cave are represented by an enum:

```rust,no_run,noplayground
{{#include ../../../src/bin/day14.rs:307:310}}
```

### Sand Grain Representation

Each unit of sand is represented by a `Grain` struct:

```rust,no_run,noplayground
{{#include ../../../src/bin/day14.rs:378:381}}
```

### Parsing Rock Formations

The input is parsed into rock formations:

```rust,no_run,noplayground
{{#include ../../../src/bin/day14.rs:10:30}}
```

### Drawing Rock Walls

Rock walls are drawn between consecutive points:

```rust,no_run,noplayground
{{#include ../../../src/bin/day14.rs:429:434}}
```

### Sand Movement Simulation

The core of the solution is the sand movement logic:

```rust,no_run,noplayground
{{#include ../../../src/bin/day14.rs:391:414}}
```

### Running the Simulation

The simulation runs until a specified condition is met:

```rust,no_run,noplayground
{{#include ../../../src/bin/day14.rs:340:360}}
```

### Managing the Floor (Part 2)

A floor is added for Part 2:

```rust,no_run,noplayground
{{#include ../../../src/bin/day14.rs:331:339}}
```

### Counting Sand Grains

The solution counts sand grains at rest:

```rust,no_run,noplayground
{{#include ../../../src/bin/day14.rs:312:316}}
```

### Main Function

The main function sets up the simulation and runs both parts of the problem:

```rust,no_run,noplayground
{{#include ../../../src/bin/day14.rs:32:62}}
```

## Visualization

The solution includes a visualization component using bracket-lib:

```rust,no_run,noplayground
{{#include ../../../src/bin/day14.rs:64:84}}
```

## Implementation Notes

- **Grid Representation**: The solution uses a hashmap for the grid, which is memory-efficient for sparse grids
- **Flexible Simulation**: The `run` method takes a closure parameter to allow different stopping conditions
- **Visualization**: The solution includes a real-time visualization of the falling sand
- **Movement Logic**: Sand follows specific rules with a priority order of movement directions

The code elegantly handles both parts of the problem using a comprehensive simulation of the physical process described in the problem.
