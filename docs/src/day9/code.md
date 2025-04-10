# Day 9: Code

Below is the complete code for Day 9's solution, which simulates the motion of a rope with multiple knots.

## Full Solution

```advent2022/src/bin/day9.rs#L1-156
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Sub;
use std::str::FromStr;
use std::vec;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize
}
impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl From<(isize,isize)> for Coord {
    fn from(pos: (isize, isize)) -> Self {
        Coord{x:pos.0, y:pos.1}
    }
}
#[derive(Debug, Copy, Clone)]
enum Command {
    Left,
    Right,
    Up,
    Down
}
#[derive(Debug, Copy, Clone)]
struct Step {
    cmd: Command,
    units: isize
}

#[derive(Debug, Copy, Clone)]
struct Link {
    pos: Coord
}
impl Link {
    fn new(pos:Coord) -> Link {
        Link { pos }
    }
    fn move_to(&mut self, cmd: Command) -> Coord {
        match cmd {
            Command::Left => self.pos.x -= 1,
            Command::Right => self.pos.x += 1,
            Command::Up => self.pos.y += 1,
            Command::Down => self.pos.y -= 1
        }
        self.position()
    }
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
                    self.rope.move_to( step.cmd )
                );
                true
            });
        }
        self
    }
}

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
            out.push( Step{ cmd, units });
            out
        })

}

fn main() {
//     let data = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2".to_string();
//     let data = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n".to_string();

    let data = std::fs::read_to_string("src/bin/day9_input.txt").expect("");

    let cmds = parse_commands(data.as_str());

    println!("2 Link Chain - Unique points: {}",
             Game::new( Chain::new((0, 0).into(), 2))
                 .run( &cmds )
                 .unique_positions()
    );
    println!("10 Links Chain - Unique points: {}",
             Game::new( Chain::new((0, 0).into(), 10))
                 .run( &cmds )
                 .unique_positions()
    );
}
```

## Code Walkthrough

### Coordinate System

```advent2022/src/bin/day9.rs#L7-23
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize
}
impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl From<(isize,isize)> for Coord {
    fn from(pos: (isize, isize)) -> Self {
        Coord{x:pos.0, y:pos.1}
    }
}
```

The `Coord` struct represents positions in 2D space. It includes:
- `x` and `y` coordinates as signed integers
- Implementation of `Sub` to calculate the distance between two coordinates
- A conversion from tuples for convenience
- Several derived traits, including `Hash` to allow using coordinates in a `HashSet`

### Movement Commands

```advent2022/src/bin/day9.rs#L24-36
#[derive(Debug, Copy, Clone)]
enum Command {
    Left,
    Right,
    Up,
    Down
}
#[derive(Debug, Copy, Clone)]
struct Step {
    cmd: Command,
    units: isize
}
```

These types represent movement commands:
- `Command` is an enum for the four possible directions
- `Step` combines a command with a distance

### Rope Link Implementation

```advent2022/src/bin/day9.rs#L38-95
#[derive(Debug, Copy, Clone)]
struct Link {
    pos: Coord
}
impl Link {
    fn new(pos:Coord) -> Link {
        Link { pos }
    }
    fn move_to(&mut self, cmd: Command) -> Coord {
        match cmd {
            Command::Left => self.pos.x -= 1,
            Command::Right => self.pos.x += 1,
            Command::Up => self.pos.y += 1,
            Command::Down => self.pos.y -= 1
        }
        self.position()
    }
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

The `Link` struct represents a single knot in the rope:
- `pos` is the current position of the knot
- `move_to` moves the knot directly in a cardinal direction
- `move_relative` implements the physical constraints of the rope, moving the knot based on its relation to the knot in front of it
- `position` returns the current position

The `move_relative` method is particularly detailed, handling all possible relative positions through pattern matching.

### Rope Chain Implementation

```advent2022/src/bin/day9.rs#L97-115
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

The `Chain` struct represents the entire rope:
- `links` is a vector of `Link` objects
- `new` creates a chain of a specified size, with all links starting at the same position
- `move_to` moves the head link directly and then propagates the movement through the chain

The `reduce` operation in `move_to` elegantly handles the chain of movement dependencies.

### Game Simulation

```advent2022/src/bin/day9.rs#L117-136
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
                    self.rope.move_to( step.cmd )
                );
                true
            });
        }
        self
    }
}
```

The `Game` struct manages the simulation:
- `rope` is the rope chain being simulated
- `unique` is a HashSet of coordinates visited by the tail
- `unique_positions` returns the number of unique positions visited
- `run` simulates all the movement steps and tracks unique tail positions

### Parsing Input

```advent2022/src/bin/day9.rs#L138-155
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
            out.push( Step{ cmd, units });
            out
        })

}
```

The `parse_commands` function converts the input text into a vector of `Step` objects by:
1. Splitting each line into parts
2. Converting the first part to a `Command`
3. Converting the second part to a distance
4. Creating a `Step` with the command and distance

### Main Function

```advent2022/src/bin/day9.rs#L157-173
fn main() {
//     let data = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2".to_string();
//     let data = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n".to_string();

    let data = std::fs::read_to_string("src/bin/day9_input.txt").expect("");

    let cmds = parse_commands(data.as_str());

    println!("2 Link Chain - Unique points: {}",
             Game::new( Chain::new((0, 0).into(), 2))
                 .run( &cmds )
                 .unique_positions()
    );
    println!("10 Links Chain - Unique points: {}",
             Game::new( Chain::new((0, 0).into(), 10))
                 .run( &cmds )
                 .unique_positions()
    );
}
```

The main function:
1. Reads the input file
2. Parses it into commands
3. For Part 1: Creates a game with a 2-link chain and runs the simulation
4. For Part 2: Creates a game with a 10-link chain and runs the simulation
5. Prints the number of unique positions visited by the tail in each case

## Implementation Notes

- **Pattern Matching**: The solution makes extensive use of pattern matching, especially in the `move_relative` method
- **Functional Approach**: The solution uses functional programming techniques like `map`, `reduce`, and method chaining
- **Trait Implementations**: Custom traits like `Sub` and trait derivations make the code more expressive and type-safe
- **Type Safety**: Custom types like `Coord`, `Command`, and `Step` provide type safety and clarity