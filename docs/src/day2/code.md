# Day 2: Code

Below is the complete code for Day 2's solution, which implements Rock Paper Scissors with two different interpretations of the strategy guide.

## Full Solution

```rust,no_run,noplayground
{{#include ../../../src/bin/day2.rs}}
```

## Code Walkthrough

### Core Types

The solution uses three main types:

1. **Move Enum**: Represents Rock, Paper, or Scissors with their score values:

```rust,no_run,noplayground
{{#include ../../../src/bin/day2.rs:3:4}}
```

2. **Outcome Enum**: Represents the possible outcomes of a round:

```rust,no_run,noplayground
{{#include ../../../src/bin/day2.rs:44:45}}
```

3. **Round Struct**: Represents a round of Rock Paper Scissors:

```rust,no_run,noplayground
{{#include ../../../src/bin/day2.rs:63:64}}
```

### Game Logic

The solution implements several key methods:

1. **Determining Win Conditions**:

```rust,no_run,noplayground
{{#include ../../../src/bin/day2.rs:16:23}}
```

2. **Determining Game Outcomes**:

```rust,no_run,noplayground
{{#include ../../../src/bin/day2.rs:24:32}}
```

3. **Deriving Moves Based on Desired Outcome**:

```rust,no_run,noplayground
{{#include ../../../src/bin/day2.rs:33:41}}
```

### Processing Input

The solution processes the input file and calculates scores for both strategies in a single pass:

```rust,no_run,noplayground
{{#include ../../../src/bin/day2.rs:79:93}}
```

### Alternative Approach

The commented-out functions at the end show an alternative approach using direct pattern matching for each input combination:

```rust,no_run,noplayground
{{#include ../../../src/bin/day2.rs:95:}}
```

This approach is more direct but less flexible than modeling the game with proper types.
