# Day 5: Solution Explanation

## Approach

Day 5's problem involves parsing a complex input format and simulating moving crates between stacks using different rules. The solution involves three main parts:

1. **Parsing the input** - Extracting the initial crate configuration and move instructions
2. **Simulating crate movements** - Implementing both the CrateMover 9000 and CrateMover 9001 rules
3. **Reading the result** - Determining which crates end up on top of each stack

## Implementation Details

### Data Structures

The solution uses two main structures:

1. **Move** - Represents a single move instruction:

```rust
#[derive(Debug,Copy,Clone)]
struct Move {
    count: usize,   // Number of crates to move
    from: usize,    // Source stack
    to: usize,      // Destination stack
}
```

2. **Buckets** - Represents the stacks of crates:

```rust
#[derive(Debug)]
struct Buckets {
    buckets: HashMap<usize,Vec<char>>,  // Stacks of crates
    keys: Vec<usize>                     // Ordered list of stack IDs
}
```

The `Buckets` structure uses a `HashMap` to store each stack, with vectors representing the crates in each stack (with the top crate at the end of the vector). It also maintains an ordered list of keys to ensure consistent access to stacks.

### Parsing the Input

The input consists of two parts: the initial crate configuration and the move instructions.

#### Parsing the Initial Configuration

The initial crate configuration is parsed by starting from the bottom of the diagram and working upward:

```rust
fn new(start: &str) -> Buckets {
    let buckets = start.lines()
        .rev()                            // Start from the bottom of the diagram
        .map(|line| line.split("").filter_map(|e| e.chars().next()).collect::<Vec<_>>())
        .fold(HashMap::new(), |map, e| {
            e.into_iter()
                .enumerate()
                .filter(|(_, c)| c.is_alphanumeric())   // Keep only letters and numbers
                .fold(map, |mut out, (key, val)| {
                    out.entry(key)
                        .or_insert(Vec::default())
                        .push(val);                      // Add each crate to its stack
                    out
                })
        });
    let mut keys = buckets.keys().copied().collect::<Vec<_>>();
    keys.sort();                          // Sort keys for consistent access
    Buckets {
        buckets,
        keys
    }
}
```

By reading the input in reverse order, we can build each stack from bottom to top.

#### Parsing the Move Instructions

The move instructions are parsed using the `FromStr` trait:

```rust
impl FromStr for Move {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [_,count,_,from,_,to] = s.split(' ').collect::<Vec<_>>()[..] {
            Ok(
                Move {
                    count: usize::from_str(count)?,
                    from: usize::from_str(from)?,
                    to: usize::from_str(to)?,
                }
            )
        } else {
            unreachable!()
        }
    }
}
```

This parses strings like "move 1 from 2 to 1" into a `Move` structure with `count=1`, `from=2`, and `to=1`.

### Simulating Crate Movements

The solution implements two different crate-moving strategies:

#### CrateMover 9000: Moving One Crate at a Time

For the CrateMover 9000, crates are moved one at a time, so they end up in reverse order:

```rust
fn crate_mover9000(&mut self, m: Move) {
    let (from, to) = self.get_keys(m);
    (0..m.count)
        .for_each(|_|{
            if let Some(c) = self.buckets.get_mut(&from).expect("").pop() {
                self.buckets.get_mut(&to).expect("").push(c)
            }
    });
}
```

This simply pops a crate from the source stack and pushes it onto the destination stack, repeating for the specified number of crates.

#### CrateMover 9001: Moving Multiple Crates at Once

For the CrateMover 9001, multiple crates are moved at once, preserving their order:

```rust
fn crate_mover9001(&mut self, m: Move) {
    let (from, to) = self.get_keys(m);
    let v = (0..m.count)
        .fold(vec![],|mut out,_|{
            if let Some(c) = self.buckets.get_mut(&from).expect("").pop() { out.push(c) }
            out
        });
    self.buckets.get_mut(&to).expect("").extend(v.iter().rev());
}
```

This code:
1. Removes the specified number of crates from the source stack
2. Collects them in a temporary vector (in reverse order)
3. Extends the destination stack with the temporary vector (reversed again)

By applying a double reversal, the original order of the crates is preserved.

### Reading the Result

After all moves are applied, we need to read the top crate from each stack:

```rust
fn scoop_top(&self) -> String {
    self.keys.iter()
        .filter_map(|key| self.buckets.get(key))   // Get each stack
        .filter_map(|arr| arr.last().copied() )    // Get the top crate
        .fold(String::new(),|mut out,s| { out.push(s); out })  // Combine into a string
}
```

This iterates through all stacks in order, gets the top crate from each, and concatenates them into a string.

## Challenge Insights

### Input Parsing Complexity

The most challenging part of this problem is parsing the initial crate configuration, which is a visual representation of stacks rather than a straightforward data format. The solution handles this by:

1. Reading the diagram from bottom to top
2. Converting each line into a sequence of characters
3. Filtering out non-alphanumeric characters
4. Building up stacks based on the position of each character

### Mapping Between Visual and Logical Indexes

The input uses 1-based indexing for stacks, but our internal representation uses 0-based indexing. The `get_keys` method handles this conversion:

```rust
fn get_keys(&self, m:Move) -> (usize,usize) {
    (self.keys[m.from-1],self.keys[m.to-1])
}
```

### Different Movement Rules

Implementing two different movement rules shows how small changes in requirements can lead to significantly different behavior. The CrateMover 9000 causes a reversal of crate order, while the CrateMover 9001 preserves it.

## Alternative Approaches

### Direct Vector Manipulation

Instead of using a HashMap, we could use a Vec<Vec<char>> to represent the stacks directly:

```rust
struct Buckets {
    stacks: Vec<Vec<char>>
}
```

This would simplify some of the code but would make parsing the initial configuration more complex.

### Using a Stack Data Structure

We could use an explicit stack data structure for each pile of crates:

```rust
use std::collections::VecDeque;

struct Buckets {
    stacks: Vec<VecDeque<char>>
}
```

However, Rust's `Vec` already provides all the necessary stack operations (`push` and `pop`), so there's no need for a separate data structure.

## Time and Space Complexity

- **Time Complexity**: O(n * m), where n is the number of move instructions and m is the maximum number of crates moved in a single instruction.
- **Space Complexity**: O(c), where c is the total number of crates.

## Conclusion

This solution demonstrates how to parse complex, visually-oriented input and simulate two different sets of rules using appropriate data structures. The use of Rust's traits (like `FromStr`) and collections (like `HashMap` and `Vec`) makes the implementation clean and efficient.