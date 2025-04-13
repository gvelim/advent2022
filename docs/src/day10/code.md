# Day 10: Code

Below is the complete code for Day 10's solution, which simulates a CPU and CRT display.

## Full Solution

```rust,no_run,noplayground
{{#include ../../../src/bin/day10.rs}}
```

## Code Walkthrough

### Data Types and Instruction Set

```rust,no_run,noplayground
{{#include ../../../src/bin/day10.rs:4:24}}
```

The code defines the core types for the CPU simulation:

- `Cycles` is a type alias for `usize` to represent clock cycles
- `InstructionSet` is an enum of the possible instructions (`Noop` and `AddX`)
- `Instruction` combines an operation with the number of cycles it takes
- `Register` is a simple wrapper around an `isize` value

The `result` method on `Instruction` returns the value that should be added to the X register after execution.

### CPU Implementation

```rust,no_run,noplayground
{{#include ../../../src/bin/day10.rs:27:67}}
```

The `CPU` struct models a simple processor with:

- An X register storing a single value
- An instruction buffer for the currently executing instruction
- A counter for the remaining execution cycles
- An instruction pointer to iterate through the program

The key methods are:

- `execute()` - Processes one cycle of the current instruction, decrements the cycle counter, and returns whether execution is still in progress
- `tick()` - Advances the CPU by one cycle, either continuing execution or fetching a new instruction
- `reg_x()` - Returns the current value of the X register

### CRT Implementation

```rust,no_run,noplayground
{{#include ../../../src/bin/day10.rs:69:88}}
```

The `CRT` struct implements a simple display:

- `width` defines how many pixels are in each row
- `clock` tracks the current pixel position
- `draw()` prints a pixel based on whether the sprite (positioned at `pos`) overlaps with the current pixel
- `tick()` advances the CRT clock after drawing a pixel

### Instruction Parsing

```rust,no_run,noplayground
{{#include ../../../src/bin/day10.rs:90:110}}
```

The `parse_instructions` function converts the input text to a list of instructions:

1. It splits each line and matches the instruction type
2. For `noop`, it creates an instruction with 1 execution cycle
3. For `addx`, it parses the value and creates an instruction with 2 execution cycles
4. It uses `fold` to build a vector of instructions while also calculating the total number of cycles

### Main Function

```rust,no_run,noplayground
{{#include ../../../src/bin/day10.rs:112:}}
```

The main function ties everything together:

1. It defines the specific cycles at which to sample the signal (20, 60, 100, etc.)
2. It initializes the CRT and CPU
3. It parses the instructions and loads them into the CPU
4. It creates a range for all cycles and maps each cycle to:
   - Advance the CPU
   - Update the CRT
   - Return the cycle number and register value
5. It filters for the specific cycles we want to sample
6. It calculates the signal strength (cycle number × register value) for each sampled cycle
7. It sums all signal strengths and prints the result

The Part 2 output (the eight capital letters) is printed directly by the CRT during simulation.

## Implementation Notes

- **State Machine Design**: The CPU is implemented as a state machine that processes instructions cycle-by-cycle
- **Separation of Concerns**: The CPU and CRT are separate components with their own state and behavior
- **Pipeline Simulation**: The instruction execution follows a simple pipeline pattern with fetch and execute stages
- **Functional Programming**: The code uses functional programming patterns like `map`, `filter`, and `fold` for concise data processing
