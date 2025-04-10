# Day 5: Code

Below is the complete code for Day 5's solution, which handles rearranging stacks of crates.

## Full Solution

```advent2022/src/bin/day5.rs#L1-78
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug,Copy,Clone)]
struct Move {
    count: usize,
    from: usize,
    to: usize
}
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
impl Move {
    fn parse_moves(moves:&str) -> Vec<Move> {
        moves.lines()
            .map(|line| Move::from_str(line).unwrap_or_else(|e| panic!("{e}")) )
            .collect()
    }
}
#[derive(Debug)]
struct Buckets {
    buckets: HashMap<usize,Vec<char>>,
    keys: Vec<usize>
}
impl Buckets {
    fn new(start: &str) -> Buckets {
        let buckets = start.lines()
            .rev()
            .map(|line| line.split("").filter_map(|e| e.chars().next()).collect::<Vec<_>>())
            .fold(HashMap::new(), |map, e| {
                e.into_iter()
                    .enumerate()
                    .filter(|(_, c)| c.is_alphanumeric())
                    .fold(map, |mut out, (key, val)| {
                        out.entry(key)
                            .or_insert(Vec::default())
                            .push(val);
                        out
                    })
            });
        let mut keys = buckets.keys().copied().collect::<Vec<_>>();
        keys.sort();
        Buckets {
            buckets,
            keys
        }
    }
    fn crate_mover9000(&mut self, m: Move) {
        let (from, to) = self.get_keys(m);
        (0..m.count)
            .for_each(|_|{
                if let Some(c) = self.buckets.get_mut(&from).expect("").pop() {
                    self.buckets.get_mut(&to).expect("").push(c)
                }
        });
    }
    fn crate_mover9001(&mut self, m: Move) {
        let (from, to) = self.get_keys(m);
        let v = (0..m.count)
            .fold(vec![],|mut out,_|{
                if let Some(c) = self.buckets.get_mut(&from).expect("").pop() { out.push(c) }
                out
            });
        self.buckets.get_mut(&to).expect("").extend(v.iter().rev());
    }
    fn scoop_top(&self) -> String {
        self.keys.iter()
            .filter_map(|key| self.buckets.get(key))
            .filter_map(|arr| arr.last().copied() )
            .fold(String::new(),|mut out,s| { out.push(s); out })
    }
    fn get_keys(&self, m:Move) -> (usize,usize) {
        (self.keys[m.from-1],self.keys[m.to-1])
    }
}

fn main() {

    let data = std::fs::read_to_string("src/bin/day5_input.txt").expect("Ops!");

    let [start,moves] = data.split("\n\n").collect::<Vec<_>>()[..] else { panic!("") };

    let mut buckets = Buckets::new(start);
    let moves = Move::parse_moves(moves);

    moves.iter().for_each(|&m| buckets.crate_mover9000(m) );
    println!("{:?}",buckets.scoop_top());

    moves.iter().for_each(|&m| buckets.crate_mover9001(m) );
    println!("{:?}",buckets.scoop_top());

}
```

## Code Walkthrough

### Data Structures

The solution uses two main structures:

1. **Move** - Represents a single move instruction:

```advent2022/src/bin/day5.rs#L5-10
#[derive(Debug,Copy,Clone)]
struct Move {
    count: usize,
    from: usize,
    to: usize
}
```

2. **Buckets** - Represents the stacks of crates:

```advent2022/src/bin/day5.rs#L36-39
#[derive(Debug)]
struct Buckets {
    buckets: HashMap<usize,Vec<char>>,
    keys: Vec<usize>
}
```

### Parsing

#### Parsing Move Instructions

The `FromStr` trait implementation for `Move` allows parsing strings like "move 1 from 2 to 1":

```advent2022/src/bin/day5.rs#L11-25
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

The helper method `parse_moves` processes multiple move instructions:

```advent2022/src/bin/day5.rs#L26-32
impl Move {
    fn parse_moves(moves:&str) -> Vec<Move> {
        moves.lines()
            .map(|line| Move::from_str(line).unwrap_or_else(|e| panic!("{e}")) )
            .collect()
    }
}
```

#### Parsing Initial Crate Configuration

The `new` method of `Buckets` parses the initial crate configuration:

```advent2022/src/bin/day5.rs#L40-63
fn new(start: &str) -> Buckets {
    let buckets = start.lines()
        .rev()                                      // Start from the bottom
        .map(|line| line.split("").filter_map(|e| e.chars().next()).collect::<Vec<_>>())
        .fold(HashMap::new(), |map, e| {
            e.into_iter()
                .enumerate()
                .filter(|(_, c)| c.is_alphanumeric())  // Keep only crate letters
                .fold(map, |mut out, (key, val)| {
                    out.entry(key)
                        .or_insert(Vec::default())
                        .push(val);                 // Add to stack
                    out
                })
        });
    let mut keys = buckets.keys().copied().collect::<Vec<_>>();
    keys.sort();                                  // Sort keys for consistent access
    Buckets {
        buckets,
        keys
    }
}
```

This method works by:
1. Reading the input in reverse order (bottom to top)
2. Splitting each line into characters
3. Filtering out non-alphanumeric characters (keeping only crate letters)
4. Building each stack based on character positions

### Crane Operations

#### CrateMover 9000: Moving One at a Time

```advent2022/src/bin/day5.rs#L64-72
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

This method moves crates one at a time, popping from the source stack and pushing to the destination.

#### CrateMover 9001: Moving Multiple at Once

```advent2022/src/bin/day5.rs#L73-81
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

This method moves multiple crates at once, preserving their order through a double-reversal process.

### Getting the Final Result

```advent2022/src/bin/day5.rs#L82-87
fn scoop_top(&self) -> String {
    self.keys.iter()
        .filter_map(|key| self.buckets.get(key))
        .filter_map(|arr| arr.last().copied() )
        .fold(String::new(),|mut out,s| { out.push(s); out })
}
```

This method retrieves the top crate from each stack and combines them into a string.

### Main Function

```advent2022/src/bin/day5.rs#L92-101
let data = std::fs::read_to_string("src/bin/day5_input.txt").expect("Ops!");

let [start,moves] = data.split("\n\n").collect::<Vec<_>>()[..] else { panic!("") };

let mut buckets = Buckets::new(start);
let moves = Move::parse_moves(moves);

moves.iter().for_each(|&m| buckets.crate_mover9000(m) );
println!("{:?}",buckets.scoop_top());

moves.iter().for_each(|&m| buckets.crate_mover9001(m) );
println!("{:?}",buckets.scoop_top());
```

The main function:
1. Reads the input file
2. Splits it into the initial configuration and move instructions
3. Creates the stacks and parses the moves
4. Applies the CrateMover 9000 rules and prints the result (Part 1)
5. Applies the CrateMover 9001 rules and prints the result (Part 2)

## Implementation Notes

- **Functional Programming Style**: The solution makes extensive use of iterators and functional programming patterns.
- **Key Transformation**: The `get_keys` method handles the conversion between 1-based indexing (in the input) and 0-based indexing (in the code).
- **Parsing Approach**: The solution parses the visual representation of the crates by reading from the bottom up and using character positions.
- **Double Reversal**: The CrateMover 9001 uses a double reversal technique to preserve the order of crates when moving multiple at once.