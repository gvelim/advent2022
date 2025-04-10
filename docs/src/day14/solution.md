# Day 14: Solution Explanation

## Approach

Day 14 involves simulating falling sand in a cave system with rock formations. The solution needs to handle several key aspects:

1. **Parsing the rock formations**: Converting input lines into coordinates for rock paths
2. **Representing the cave**: Creating a data structure to track materials (rock, sand, air) at each position
3. **Simulating sand movement**: Implementing the rules for sand falling and coming to rest
4. **Handling two scenarios**: Tracking sand units for both scenarios (with and without a floor)

The solution uses a grid-based approach with custom data types for the board, materials, and sand grains.

## Implementation Details

### Data Structures

The solution uses several key data structures:

#### Board

The `Board<T>` struct represents the cave system:

```rust
struct Board<T> {
    width: usize,
    height: usize,
    centre_x: usize,
    offset_x: usize,
    grid: HashMap<Coord, T>,
}
```

This structure uses a hashmap to store the material at each position, which is more memory-efficient than a full 2D array when most of the cave is air.

#### Material

An enum represents the different materials in the cave:

```rust
enum Material { Rock, Sand, Air }
```

#### Grain

The `Grain` struct represents a single unit of sand:

```rust
struct Grain {
    pos: Coord,
    settled: bool
}
```

### Parsing Rock Formations

The input is parsed into a series of rock paths:

```rust
fn parse_plines(input:&str) -> (Coord, Coord, Vec<Vec<Coord>>) {
    let mut br = Coord{ x: usize::MIN, y: usize::MIN };
    let mut tl = Coord{ x: usize::MAX, y: 0 };
    let plines =
        input.lines()
            .map(|line|{
                line.split(" -> ")
                    .map(|val| Coord::from_str(val).expect("Ops!"))
                    .inspect(|p|{
                        tl.x = std::cmp::min(tl.x, p.x);
                        br.x = std::cmp::max(br.x, p.x);
                        br.y = std::cmp::max(br.y, p.y);
                    })
                    .collect::<Vec<_>>()
            })
            .fold(vec![],|mut out, pline|{
                out.push(pline);
                out
            });
    (tl, br, plines)
}
```

This function:
1. Parses each line of the input into a sequence of coordinates
2. Tracks the bounding box of all coordinates (top-left and bottom-right)
3. Returns the bounding box and the list of rock paths

### Creating the Cave Board

The board is created based on the bounding box:

```rust
fn new(tl: Coord, br: Coord) -> Self {
    let width = br.x - tl.x + 1 + 200;
    let offset_x = if tl.x > 200 { tl.x - 100 } else { 0 };
    let centre_x = 500 - offset_x;
    Board {
        width,
        height: br.y + 3,
        centre_x,
        offset_x,
        grid: HashMap::new()
    }
}
```

The board is sized to include all rock formations plus some extra space for sand to accumulate. The `offset_x` value is used to make the board more memory-efficient by not starting from x=0 when all the action happens near x=500.

### Drawing Rock Formations

Rock formations are drawn on the board using the `Painter` helper:

```rust
fn rock_walls(board: &mut Board<Material>, points: &Vec<Coord>) {
    points.windows(2)
        .for_each(|w|{
            if let [a, b] = w {
                Painter::wall(board, *a, *b, Material::Rock);
            }
        })
}
```

This function takes a sequence of points and draws rock walls between each consecutive pair. The `wall` function handles drawing both horizontal and vertical lines:

```rust
fn wall(board: &mut Board<Material>, a: Coord, b: Coord, m: Material) {
    if a.x == b.x {
        // vertical wall
        for y in std::cmp::min(a.y, b.y)..=std::cmp::max(a.y, b.y) {
            *board.square_mut(Coord { x: a.x, y }).unwrap() = m;
        }
    } else if a.y == b.y {
        // horizontal wall
        for x in std::cmp::min(a.x, b.x)..=std::cmp::max(a.x, b.x) {
            *board.square_mut(Coord { x, y: a.y }).unwrap() = m;
        }
    }
}
```

### Simulating Sand Movement

The core of the solution is the sand simulation. A unit of sand falls according to specific rules until it comes to rest or falls into the abyss:

```rust
fn fall(&mut self, board: &Board<Material>) -> Option<()> {
    // Try to move down
    let down = Coord { x: self.pos.x, y: self.pos.y + 1 };
    if let Some(Material::Air) = board.square(down) {
        self.pos = down;
        return Some(());
    }
    
    // Try to move down-left
    let down_left = Coord { x: self.pos.x - 1, y: self.pos.y + 1 };
    if let Some(Material::Air) = board.square(down_left) {
        self.pos = down_left;
        return Some(());
    }
    
    // Try to move down-right
    let down_right = Coord { x: self.pos.x + 1, y: self.pos.y + 1 };
    if let Some(Material::Air) = board.square(down_right) {
        self.pos = down_right;
        return Some(());
    }
    
    // Can't move further
    if board.in_bounds(self.pos) {
        self.settled = true;
        Some(())
    } else {
        None
    }
}
```

This method tries to move the sand grain in the priority order: down, down-left, down-right. If no move is possible, the grain comes to rest.

### Running the Simulation

The `run` method simulates falling sand until a specified condition is met:

```rust
fn run<F>(&mut self, start: Coord, check_goal: F) where F: Fn(&Grain) -> bool {
    loop {
        let mut grain = Grain::release_grain(start);

        // let the grain fall until it either (a) settles or (b) falls off the board
        while grain.fall(self).is_some() {};

        // Have we reached an end state?
        // we use a closure that passes the stopped grain
        // for checking whether (a) it has fallen in the abyss or (b) reached the starting position
        if check_goal(&grain) {
            // Mark settled grain position on the board
            *self.square_mut(grain.pos).unwrap() = Material::Sand;
            break
        }

        // Mark settled grain position on the board
        *self.square_mut(grain.pos).unwrap() = Material::Sand;
    }
}
```

The method takes a closure `check_goal` that determines when to stop the simulation. This allows for different stopping conditions for Part 1 and Part 2.

### Adding a Floor (Part 2)

For Part 2, a floor is added at the bottom of the cave:

```rust
fn toggle_floor(&mut self) {
    let height = self.height-1;
    let left = Coord { x: self.offset_x, y: height };
    let right = Coord { x: self.offset_x + self.width - 1, y : height };
    match self.square(left) {
        Some(Material::Rock) => Painter::wall(self, left, right, Material::Air),
        _ => Painter::wall(self, left, right, Material::Rock)
    }
}
```

This adds a horizontal rock wall at the bottom of the cave, simulating the floor described in Part 2.

### Counting Sand Grains

The solution counts the number of sand grains at rest:

```rust
fn grains_at_rest(&self) -> usize {
    self.grid.values()
        .filter(|&s| Material::Sand.eq(s))
        .count()
}
```

### Solving the Problem

The solution solves both parts of the problem:

```rust
// Part 1: Count sand units until one falls into the abyss
board.run(start, |g| !g.is_settled());
println!("Scenario 1: Grains Rest: {}", board.grains_at_rest() - 1);

// Reset for Part 2
board.empty_sand();

// Part 2: Add floor and count until source is blocked
board.toggle_floor();
board.run(start, |g| g.pos.eq(&start));
println!("Scenario 2: Grains Rest: {}", board.grains_at_rest());
```

For Part 1, the simulation stops when a grain fails to settle (falls into the abyss).
For Part 2, the simulation stops when a grain settles at the source position, blocking further sand.

## Visualization

The solution includes a visualization component using the bracket-lib library, allowing you to see the sand falling in real-time.

## Algorithm Analysis

### Time Complexity

- **Parsing**: O(n) where n is the number of coordinates in the input
- **Sand Simulation**: O(s u00d7 h) where s is the number of sand grains and h is the height of the cave
- **Overall**: O(s u00d7 h) since the sand simulation dominates

### Space Complexity

- **Board Storage**: O(r + s) where r is the number of rock positions and s is the number of sand positions
- **Path Storage**: O(n) for storing the rock paths

## Alternative Approaches

### Array-Based Grid

Instead of using a hashmap for the grid, we could use a 2D array:

```rust
struct ArrayBoard {
    width: usize,
    height: usize,
    grid: Vec<Vec<Material>>
}
```

This would have faster access times (O(1) vs. hashmap's average O(1) but worst-case O(n)), but would use more memory for sparse caves.

### Scan Lines

Another approach would be to use a scan line algorithm to more efficiently determine where sand will come to rest without simulating each step:

```rust
fn calculate_rest_position(board: &Board, start: Coord) -> Option<Coord> {
    // Find the first rock/sand below the start position
    // Check if sand can flow left or right
    // Return the final rest position
}
```

This could be faster for certain scenarios but would be more complex to implement correctly, especially for Part 2.

## Conclusion

This solution demonstrates a comprehensive approach to physical simulation in a grid-based environment. The use of a hashmap for the grid provides memory efficiency, while the simulation logic accurately captures the problem's constraints. The solution is also flexible enough to handle both parts of the problem with minimal changes.