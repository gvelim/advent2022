{{REWRITTEN_CODE}}
# Day 7: Code

Below is the complete code for Day 7's solution, which parses terminal output to build a directory tree and analyze directory sizes.

## Full Solution

```rust,no_run,noplayground
{{#include ../../../src/bin/day7.rs:1:138}}
```

## Code Walkthrough

### Data Types

The solution defines several types to represent the file system and terminal output:

```rust,no_run,noplayground
{{#include ../../../src/bin/day7.rs:5:19}}
```

These enums represent:
- `ResultType`: Either a file (with name and size) or a directory (with name)
- `CommandType`: Either a change directory command or a list command
- `LineType`: Either a command or a result

### Path Representation

```rust,no_run,noplayground
{{#include ../../../src/bin/day7.rs:20:29}}
```

The `Path` struct encapsulates a string representing a file path and provides methods to create and append to paths.

### Directory Tree

```rust,no_run,noplayground
{{#include ../../../src/bin/day7.rs:30:40}}
```

The directory tree consists of:
- `Node`: Represents a directory with its parent, contents, and direct size
- `Tree`: Contains a map of paths to nodes and a list of total sizes

### Directory Tree Methods

```rust,no_run,noplayground
{{#include ../../../src/bin/day7.rs:41:60}}
```

These methods provide functionality to:
- Get a list of child directories
- Get the direct size of a directory
- Take the list of total sizes

### Parsing Terminal Output

```rust,no_run,noplayground
{{#include ../../../src/bin/day7.rs:61:90}}
```

This method builds a directory tree by processing terminal commands:
- For `cd ..` commands, it moves up to the parent directory
- For other `cd` commands, it creates a new directory if needed and moves into it
- For result lines, it adds files or directories to the current directory's contents

### Calculating Total Sizes

```rust,no_run,noplayground
{{#include ../../../src/bin/day7.rs:91:100}}
```

This recursive method calculates the total size of each directory by adding its direct size to the total sizes of its subdirectories.

### Creating the Line Iterator

```rust,no_run,noplayground
{{#include ../../../src/bin/day7.rs:103:120}}
```

This creates an iterator that converts terminal output lines into `LineType` values by parsing each line based on its format.

### Main Function

```rust,no_run,noplayground
{{#include ../../../src/bin/day7.rs:122:}}
```

The main function:
1. Reads the terminal output from a file
2. Creates an iterator to parse the output
3. Builds a directory tree using the parsed commands
4. Calculates the total size of each directory
5. For Part 1: Finds directories smaller than 100,000 and sums their sizes
6. For Part 2: Finds the smallest directory that would free enough space when deleted

## Implementation Notes

- **RefCell Usage**: The solution uses a `RefCell` to store the list of total sizes, allowing it to be modified during the recursive calculation
- **Path Representation**: Paths are represented as strings for simplicity, but with a custom wrapper type for safety
- **Tree Structure**: The directory tree uses a map-based representation with explicit parent references, making it easy to navigate up and down the tree
- **Functional Approach**: The solution makes extensive use of iterators and functional programming patterns
