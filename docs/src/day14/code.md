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

```advent2022/src/bin/day14.rs#L461-467
struct Board<T> {
    width: usize,
    height: usize,
    centre_x: usize,
    offset_x: usize,
    grid: HashMap<Coord, T>,
}
```

The materials in the cave are represented by an enum:

```advent2022/src/bin/day14.rs#L307-310
enum Material { Rock, Sand, Air }
impl Default for Material {
    fn default() -> Self { Material::Air }
}
```

### Sand Grain Representation

Each unit of sand is represented by a `Grain` struct:

```advent2022/src/bin/day14.rs#L378-381
struct Grain {
    pos: Coord,
    settled: bool
}
```

### Parsing Rock Formations

The input is parsed into rock formations:

```advent2022/src/bin/day14.rs#L10-30
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

### Drawing Rock Walls

Rock walls are drawn between consecutive points:

```advent2022/src/bin/day14.rs#L429-434
fn rock_walls(board: &mut Board<Material>, points: &Vec<Coord>) {
    points.windows(2)
        .for_each(|w|{
            if let [a, b] = w {
                Painter::wall(board, *a, *b, Material::Rock);
            }
        })
}
```

### Sand Movement Simulation

The core of the solution is the sand movement logic:

```advent2022/src/bin/day14.rs#L391-410
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

### Running the Simulation

The simulation runs until a specified condition is met:

```advent2022/src/bin/day14.rs#L340-360
fn run<F>(&mut self, start: Coord, check_goal: F) where F: Fn(&Grain) -> bool {

    loop {
        let mut grain = Grain::release_grain(start);

        // let the grain fall until it either (a) settles or (b) falls off the board
        while grain.fall(self).is_some() {};

        // Have we reached an end state ?
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

### Managing the Floor (Part 2)

A floor is added for Part 2:

```advent2022/src/bin/day14.rs#L331-339
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

### Counting Sand Grains

The solution counts sand grains at rest:

```advent2022/src/bin/day14.rs#L312-316
fn grains_at_rest(&self) -> usize {
    self.grid.values()
        .filter(|&s| Material::Sand.eq(s) )
        .count()
}
```

### Main Function

The main function sets up the simulation and runs both parts of the problem:

```advent2022/src/bin/day14.rs#L32-50
fn main() -> BResult<()> {

    // let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9".to_string();
    let input = std::fs::read_to_string("src/bin/day14_input.txt").expect("ops!");

    // parse the board's wall layout
    let (tl, br, plines) = parse_plines(input.as_str());

    let mut board = Board::new(tl, br);

    // paint layout on the board
    plines.into_iter()
        .for_each(|pline|
            Painter::rock_walls(&mut board, &pline)
        );

    // run the sand simulation until we reach the abyss, that is, grain stopped but not settled
    let start = (board.centre_x, 0).into();
    board.run(
        start, |g| !g.is_settled()
    );
    println!("Scenario 1: Grains Rest: {}", board.grains_at_rest() - 1);
```

## Visualization

The solution includes a visualization component using bracket-lib:

```advent2022/src/bin/day14.rs#L361-375
fn draw(&self, ctx: &mut BTerm) {
    for y in 0..self.height {
        for x in 0..self.width {
            ctx.set(x, y, BLACK, BLACK, 32);
        }
    }
    self.grid.iter().for_each(|(Coord{x,y},s)| {
        match s {
            Material::Rock => ctx.set(*x - self.offset_x, *y, GRAY50, GRAY30, to_cp437('#')),
            Material::Sand => ctx.set(*x - self.offset_x, *y, GOLD1, BLACK, to_cp437('o')),
            _ => {}
        }
    });
    ctx.set(self.centre_x, 0, GREEN1, BLACK, to_cp437('+'));
}
```

## Implementation Notes

- **Grid Representation**: The solution uses a hashmap for the grid, which is memory-efficient for sparse grids
- **Flexible Simulation**: The `run` method takes a closure parameter to allow different stopping conditions
- **Visualization**: The solution includes a real-time visualization of the falling sand
- **Movement Logic**: Sand follows specific rules with a priority order of movement directions

The code elegantly handles both parts of the problem using a comprehensive simulation of the physical process described in the problem.