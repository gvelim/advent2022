# Day 9: Solution Explanation

## Approach

Day 9 involves simulating the motion of a rope with multiple knots. The solution breaks down into several key components:

1. **Representing coordinates**: We need a way to represent positions in 2D space
2. **Modeling the rope**: We need to model a chain of connected knots
3. **Implementing movement rules**: We need to implement how knots move in relation to each other
4. **Tracking unique positions**: We need to track unique positions visited by the tail knot

The solution uses a combination of custom data structures and simulation logic to model the rope's behavior.

## Implementation Details

### Coordinate System

First, we define a `Coord` struct to represent positions in 2D space:

```rust
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize
}
```

This struct includes several derived traits:
- `Debug`, `Copy`, and `Clone` for convenience
- `PartialEq` and `Eq` for equality comparisons
- `Hash` to allow using coordinates as keys in a HashSet

We also implement the `Sub` trait to make it easy to calculate the distance between two coordinates:

```rust
impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
```

And a conversion from tuples for convenience:

```rust
impl From<(isize,isize)> for Coord {
    fn from(pos: (isize, isize)) -> Self {
        Coord{x:pos.0, y:pos.1}
    }
}
```

### Movement Commands

We define an enum to represent the four possible movement directions:

```rust
#[derive(Debug, Copy, Clone)]
enum Command {
    Left,
    Right,
    Up,
    Down
}
```

And a struct to represent a movement step with a direction and distance:

```rust
#[derive(Debug, Copy, Clone)]
struct Step {
    cmd: Command,
    units: isize
}
```

### Modeling Rope Links

Each knot in the rope is modeled as a `Link` that knows its position and how to move:

```rust
#[derive(Debug, Copy, Clone)]
struct Link {
    pos: Coord
}
```

The `Link` struct has methods for different types of movement:

```rust
impl Link {
    fn new(pos:Coord) -> Link {
        Link { pos }
    }
    
    // Move directly in a cardinal direction
    fn move_to(&mut self, cmd: Command) -> Coord {
        match cmd {
            Command::Left => self.pos.x -= 1,
            Command::Right => self.pos.x += 1,
            Command::Up => self.pos.y += 1,
            Command::Down => self.pos.y -= 1
        }
        self.position()
    }
    
    // Move relative to another link based on physical constraints
    fn move_relative(&mut self, front: &Link) -> Coord {
        let dist = front.position() - self.position();
        let (dx,dy) = match (dist.x, dist.y) {
            // overlapping
            (0, 0) => (0, 0),
            // touching up/left/down/right
            (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
            // touching diagonally
            (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
            // need to move up/left/down/right
            (0, 2) => (0, 1),
            (0, -2) => (0, -1),
            (2, 0) => (1, 0),
            (-2, 0) => (-1, 0),
            // need to move to the right diagonally
            (2, 1) => (1, 1),
            (2, -1) => (1, -1),
            // need to move to the left diagonally
            (-2, 1) => (-1, 1),
            (-2, -1) => (-1, -1),
            // need to move up/down diagonally
            (1, 2) => (1, 1),
            (-1, 2) => (-1, 1),
            (1, -2) => (1, -1),
            (-1, -2) => (-1, -1),
            // need to move diagonally
            (-2, -2) => (-1, -1),
            (-2, 2) => (-1, 1),
            (2, -2) => (1, -1),
            (2, 2) => (1, 1),
            _ => panic!("unhandled case: tail - head = {dist:?}"),
        };
        self.pos.x += dx;
        self.pos.y += dy;
        self.position()
    }
    
    fn position(&self) -> Coord {
        self.pos
    }
}
```

The `move_relative` method is the heart of the solution. It implements the physical constraint that if a knot is too far from the knot in front of it, it must move to maintain proximity. The method calculates the relative position and then determines the appropriate movement using pattern matching.

### Modeling the Rope Chain

The entire rope is modeled as a chain of links:

```rust
struct Chain {
    links: Vec<Link>
}

impl Chain {
    fn new(pos:Coord, size:usize) -> Chain {
        Chain {
            links: vec![Link::new(pos); size]
        }
    }
    
    fn move_to(&mut self, cmd: Command) -> Coord {
        self.links[0].move_to(cmd);
        self.links
            .iter_mut()
            .reduce(|front,tail|{
                tail.move_relative(front);
                tail
            })
            .unwrap()
            .position()
    }
}
```

The `move_to` method moves the head knot directly and then propagates the movement through the chain using `reduce`. This elegantly handles the chain of dependencies where each knot's movement depends on the knot in front of it.

### Game Simulation

The overall simulation is handled by the `Game` struct:

```rust
struct Game {
    rope: Chain,
    unique: HashSet<Coord>
}

impl Game {
    fn new(rope: Chain) -> Game {
        Game { rope, unique: HashSet::new() }
    }
    
    fn unique_positions(&self) -> usize {
        self.unique.len()
    }
    
    fn run(&mut self, input: &Vec<Step>) -> &Self{
        for step in input {
            (0..step.units).all(|_| {
                self.unique.insert(
                    self.rope.move_to(step.cmd)
                );
                true
            });
        }
        self
    }
}
```

The `Game` struct:
- Manages the rope chain
- Tracks unique positions visited by the tail knot using a `HashSet`
- Provides a `run` method to simulate all the movement steps

### Parsing Input

The input is parsed into a sequence of `Step` values:

```rust
fn parse_commands(input: &str) -> Vec<Step> {
    input.lines()
        .map(|line| line.split(' '))
        .map(|mut s| {
            let cmd = match s.next() {
                Some("R") => Command::Right,
                Some("U") => Command::Up,
                Some("D") => Command::Down,
                Some("L") => Command::Left,
                _ => panic!("Woohaaaa!")
            };
            (cmd, isize::from_str(s.next().unwrap()).unwrap())
        })
        .fold(vec![], |mut out, (cmd, units)| {
            out.push(Step{ cmd, units });
            out
        })
}
```

### Main Solution

The main solution creates two games - one with a 2-knot rope for Part 1 and one with a 10-knot rope for Part 2:

```rust
fn main() {
    let data = std::fs::read_to_string("src/bin/day9_input.txt").expect("");
    let cmds = parse_commands(data.as_str());

    println!("2 Link Chain - Unique points: {}",
             Game::new(Chain::new((0, 0).into(), 2))
                 .run(&cmds)
                 .unique_positions()
    );
    println!("10 Links Chain - Unique points: {}",
             Game::new(Chain::new((0, 0).into(), 10))
                 .run(&cmds)
                 .unique_positions()
    );
}
```

## Algorithm Analysis

### Time Complexity

- Parsing the input: O(n) where n is the number of lines in the input
- Simulating the rope: O(n * m * k) where:
  - n is the number of steps
  - m is the maximum number of units in any step
  - k is the number of knots in the rope

### Space Complexity

- Storing the rope: O(k) where k is the number of knots
- Storing unique positions: O(p) where p is the number of unique positions visited

## Alternative Approaches

### Simplified Movement Logic

The `move_relative` method uses a detailed pattern match to handle all possible relative positions. An alternative approach could use a more general formula:

```rust
fn move_relative_simplified(&mut self, front: &Link) -> Coord {
    let dist = front.position() - self.position();
    
    // If Manhattan distance <= 1 or diagonal distance = 1, don't move
    if (dist.x.abs() <= 1 && dist.y.abs() <= 1) {
        return self.position();
    }
    
    // Otherwise, move in the direction of the front knot
    self.pos.x += dist.x.signum();
    self.pos.y += dist.y.signum();
    self.position()
}
```

This approach is more concise but less explicit about the movement rules.

### Alternative Coordinate Representation

Instead of a custom `Coord` struct, we could use tuples:

```rust
type Coord = (isize, isize);

// Calculate distance
fn distance(a: Coord, b: Coord) -> Coord {
    (a.0 - b.0, a.1 - b.1)
}
```

This would be simpler but less expressive and type-safe.

## Conclusion

This solution demonstrates a clean approach to simulating physical constraints in a chain of connected objects. The use of custom types for coordinates and links, along with the implementation of movement rules, creates a readable and maintainable solution. The approach generalizes well from the 2-knot rope in Part 1 to the 10-knot rope in Part 2 without requiring significant changes to the code.