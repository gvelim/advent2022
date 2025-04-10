# Day 12: Code

Below is the complete code for Day 12's solution, which implements a path-finding algorithm to navigate a heightmap.

## Full Solution

```advent2022/src/bin/day12.rs#L1-324
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use bracket_lib::prelude::*;
use advent2022:{
    Grid, Coord,
    app::{App, AppLevel, State}
};

fn main() -> BResult<()> {

    let input = std::fs::read_to_string("src/bin/day12_input.txt").expect("ops!");

    // parse elevations onto a grid
    let (mut grid,start, target) = parse_elevation(input.as_str());

    // find path with closure fn() goal set at reaching the target coordinate
    let path = grid.shortest_path(start,|cs| cs.eq(&target));

    // visualise path produced
    grid.visualise_path(path);

    // reverse the elevation so E(0) and S(27)
    grid.reverse_elevation();

    // find path with closure fn() goal set as reaching elevation(26) = a
    let path = grid.shortest_path(target, |cs| 26.eq(grid.0.square(cs).unwrap()));

    // visualise path produced
    grid.visualise_path(path);
    grid.reverse_elevation();

    let mut ctx = BTermBuilder::simple(160,120)?
        .with_simple_console(grid.width(),grid.height(), "terminal8x8.png")
        .with_simple_console_no_bg(grid.width(),grid.height(), "terminal8x8.png")
        .with_simple_console_no_bg(grid.width(),grid.height(), "terminal8x8.png")
        .with_fps_cap(640f32)
        .with_title("Day12: Path Search")
        .build()?;

    let ps = PathSearch::init(&grid);
    let mut app = App::init(GStore { grid, target, start, ps } , Level::MENU);

    app.register_level(Level::MENU, Menu);
    app.register_level(Level::LEVEL1, ExerciseOne);
    app.register_level(Level::LEVEL2, ExerciseTwo);

    ctx.set_active_console(1);
    app.store().grid.draw(&mut ctx);
    main_loop(ctx, app)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash )]
pub enum Level { MENU, LEVEL1, LEVEL2 }

struct GStore {
    grid: ElevationGrid,
    target: Coord,
    start: Coord,
    ps: PathSearch,
}

struct Menu;
impl AppLevel for Menu {
    type GStore = GStore;
    type GLevel = Level;
    fn init(&mut self, _: &mut BTerm, _: &mut Self::GStore) -> (Self::GLevel, State) {
        (Level::MENU, State::RUN)
    }
    fn run(&mut self, ctx: &mut BTerm, _: &mut Self::GStore) -> (Self::GLevel, State) {
        ctx.set_active_console(3);
        match ctx.key {
            Some(VirtualKeyCode::Key1) => { ctx.cls(); (Level::LEVEL1, State::INIT) },
            Some(VirtualKeyCode::Key2) => { ctx.cls(); (Level::LEVEL2, State::INIT) },
            Some(VirtualKeyCode::Q) => (Level::MENU, State::FINISH),
            _ => {
                ctx.print_centered( 42, "Press '1' : Lowest to highest elevation");
                ctx.print_centered( 44, "Press '2' : Highest to lowest elevation ");
                ctx.print_centered( 46, "Press 'Q' to exit");
                (Level::MENU, State::RUN)
            }
        }
    }
    fn term(&mut self, ctx: &mut BTerm, _: &mut Self::GStore) -> (Self::GLevel, State) {
        ctx.quit();
        (Level::MENU, State::FINISH)
    }
}

struct ExerciseOne;
impl AppLevel for ExerciseOne {
    type GStore = GStore;
    type GLevel = Level;
    fn init(&mut self, _: &mut BTerm, store: &mut Self::GStore) -> (Self::GLevel, State) {
        store.ps.reset();
        store.ps.queue.push_back(store.start);
        (Level::LEVEL1, State::RUN)
    }
    fn run(&mut self, ctx: &mut BTerm, store: &mut Self::GStore) -> (Self::GLevel, State) {
        ctx.set_active_console(2);
        match store.ps.tick(&store.grid, |cs| cs.eq(&store.target)) {
            None => {
                ctx.cls();
                store.ps.draw(ctx);
                ctx.set(store.target.x,store.target.y, BLUE, BLACK, to_cp437('\u{2588}'));
                (Level::LEVEL1, State::RUN)
            }
            Some(target) => {
                store.ps.queue.clear();
                store.ps.queue.push_back(target);
                ctx.cls();
                store.ps.draw(ctx);
                (Level::LEVEL1, State::FINISH)
            }
        }
    }
    fn term(&mut self, ctx: &mut BTerm, _: &mut Self::GStore) -> (Self::GLevel, State) {
        ctx.set_active_console(3);
        ctx.print_centered(10, "Path Found !!");
        (Level::MENU, State::INIT)
    }
}

struct ExerciseTwo;
impl AppLevel for ExerciseTwo {
    type GStore = GStore;
    type GLevel = Level;
    fn init(&mut self, _: &mut BTerm, store: &mut Self::GStore) -> (Self::GLevel, State) {
        store.ps.reset();
        store.ps.queue.push_back(store.target);
        store.grid.reverse_elevation();
        (Level::LEVEL2, State::RUN)
    }
    fn run(&mut self, ctx: &mut BTerm, store: &mut Self::GStore) -> (Self::GLevel, State) {
        ctx.set_active_console(2);
        match store.ps.tick(&store.grid, |cs| 26.eq(store.grid.0.square(cs).unwrap())) {
            None => {
                ctx.cls();
                store.ps.draw(ctx);
                (Level::LEVEL2, State::RUN)
            }
            Some(target) => {
                store.ps.queue.clear();
                store.ps.queue.push_back(target);
                ctx.cls();
                store.ps.draw(ctx);
                store.grid.reverse_elevation();
                (Level::LEVEL2, State::FINISH)
            }
        }
    }
    fn term(&mut self, ctx: &mut BTerm, _: &mut Self::GStore) -> (Self::GLevel, State) {
        ctx.set_active_console(3);
        ctx.print_centered(10, "Path Found !!");
        (Level::MENU, State::INIT)
    }
}

fn parse_elevation(data: &str) -> (ElevationGrid, Coord, Coord) {
    let width = data.lines().next().unwrap().len();
    let height = data.lines().count();
    let mut grid = Grid::new(width,height);
    let (mut start, mut finish) = ((0,0).into(),(0,0).into());

    for (y,line) in data.lines().enumerate() {
        for (x, val) in line.bytes().enumerate() {
            match val {
                b'S' => {
                    start = (x, y).into();
                    *grid.square_mut(start).unwrap() = 0;
                },
                b'E' => {
                    finish = (x, y).into();
                    *grid.square_mut(finish).unwrap() = b'z'-b'a'+2;
                }
                _ => *grid.square_mut((x, y).into()).unwrap() = val - b'a' + 1
            }
        }
    }
    (ElevationGrid(grid), start, finish)
}

struct PathSearch {
    queue: VecDeque<Coord>,
    visited: Grid<(bool,Option<Coord>)>,
    path: Vec<Coord>
}
impl PathSearch {
    fn init(grid: &ElevationGrid) -> PathSearch {
        PathSearch {
            queue: VecDeque::<Coord>::new(),
            visited: Grid::new(grid.width(), grid.height()),
            path: Vec::<_>::new()
        }
    }
    fn reset(&mut self) {
        self.queue.clear();
        self.visited.grid.iter_mut().for_each(|val| *val = (false, None) );
        self.path.clear();
    }
    fn tick<F>(&mut self, grid: &ElevationGrid, goal: F) -> Option<Coord> where F: Fn(Coord)->bool {
        let Some(cs) = self.queue.pop_front() else { return None };

        // position matches target
        if goal(cs) {
            return Some(cs);
        }
        // mark square as visited
        self.visited.square_mut(cs).unwrap().0 = true;

        let &square = grid.0.square(cs).unwrap();

        // evaluate neighbour squares and
        // push to the queue if the have elevation delta <= 1
        grid.0.neighbouring(cs)
            .for_each(|(ns, &elevation)| {
                if let Some((false, None)) = self.visited.square(ns) {
                    if elevation <= square + 1 {
                        // capture the square we arrived from
                        self.visited.square_mut(ns).unwrap().1 = Some(cs);
                        self.queue.push_back(ns)
                    }
                }
            });
        None
    }
    fn extract_path(&self, start:Coord) -> PathIter {
        PathIter { ps: self, cur: start }
    }
    fn draw(&self,ctx: &mut BTerm) {
        self.queue.iter()
            .for_each(|&cs| {
                ctx.set(cs.x,cs.y,RED,BLACK,to_cp437('\u{2588}'));
                self.extract_path(cs)
                    .for_each(|Coord{x,y}|
                        ctx.set(x,y,ORANGE, BLACK,to_cp437('\u{2588}'))
                    )
            })
    }
}
struct PathIter<'a> {
    ps: &'a PathSearch,
    cur: Coord
}
impl Iterator for PathIter<'_> {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        match self.ps.visited.square(self.cur).unwrap().1 {
            Some(par) => {
                self.cur = par;
                Some(par)
            }
            _ => None
        }
    }
}

struct ElevationGrid(Grid<u8>);

impl ElevationGrid {
    fn width(&self) -> usize { self.0.width }
    fn height(&self) -> usize { self.0.height }
    fn reverse_elevation(&mut self) {
        let &max = self.0.iter().max().unwrap();
        self.0.iter_mut()
            .map(|val|{
                *val = max - *val;
            })
            .all(|_| true);
    }
    fn visualise_path(&self, path:Vec<Coord>) {
        let mut gpath= ElevationGrid(Grid::new(self.width(), self.height()) );
        path.iter().for_each(|&a| *gpath.0.square_mut(a).unwrap() = *self.0.square(a).unwrap() );
        println!("Path length: {}\n{:?}",path.len(),gpath);
    }
    fn shortest_path<F>(&self, start: Coord, goal:F ) -> Vec<Coord> where F: Fn(Coord)->bool {

        let mut ps = PathSearch::init(self);
        // push start in the queue
        ps.queue.push_back(start);

        // pop from top & while still nodes in the queue
        while let Some(cs) = ps.queue.pop_front() {

            // position matches target
            if goal(cs) {
                // extract parent position from target
                let mut cur = cs;
                while let Some(par) = ps.visited.square(cur).unwrap().1 {
                    ps.path.push(par);
                    cur = par;
                }
                // remove start position from path
                ps.path.pop();
                break
            }

            // mark square as visited
            ps.visited.square_mut(cs).unwrap().0 = true;

            let &square = self.0.square(cs).unwrap();

            // evaluate neighbour squares and
            // push to the queue if the have elevation delta <= 1
            self.0.neighbouring(cs)
                .for_each(|(ns, &elevation)| {
                    if let Some((false, None)) = ps.visited.square(ns) {
                        if elevation <= square + 1 {
                            // capture the square we arrived from
                            ps.visited.square_mut(ns).unwrap().1 = Some(cs);
                            ps.queue.push_back(ns)
                        }
                    }
                })
        }
        ps.path
    }
    fn draw(&self, ctx: &mut BTerm) {
        let rgb: Vec<_> = RgbLerp::new(CADETBLUE.into(), WHITESMOKE.into(), 27).collect();
        (0..self.height()).for_each(|y|{
            (0..self.width()).for_each(|x|
                ctx.set_bg(x, y, self.0.square((x, y).into()).map(|&cell| rgb[cell as usize]).unwrap_or(BLACK.into()))
            );
        });
    }
}

impl Debug for ElevationGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (0..self.height()).for_each(|y|{
            (0..self.width()).for_each(|x|
                write!(f, "{:^2}",
                       self.0.square((x, y).into())
                           .map(|&cell| match cell { 0 => '.', _=> 'x'})
                           .expect("TODO: panic message")
                ).expect("failed in x")
            );
            writeln!(f).expect("failed in y");
        });
        Ok(())
    }
}
```

## Code Walkthrough

### Core Data Structures

```advent2022/src/bin/day12.rs#L223-224
struct ElevationGrid(Grid<u8>);
```

The solution uses an `ElevationGrid` wrapper around a generic `Grid<u8>` to represent the heightmap. Elevation values are stored as bytes, with special values for the start and end positions.

```advent2022/src/bin/day12.rs#L179-183
struct PathSearch {
    queue: VecDeque<Coord>,
    visited: Grid<(bool,Option<Coord>)>,
    path: Vec<Coord>
}
```

The `PathSearch` struct manages the breadth-first search algorithm, tracking:
- A queue of coordinates to explore
- A grid marking visited positions and their parent positions
- A vector to store the final path

### Input Parsing

```advent2022/src/bin/day12.rs#L150-178
fn parse_elevation(data: &str) -> (ElevationGrid, Coord, Coord) {
    let width = data.lines().next().unwrap().len();
    let height = data.lines().count();
    let mut grid = Grid::new(width,height);
    let (mut start, mut finish) = ((0,0).into(),(0,0).into());

    for (y,line) in data.lines().enumerate() {
        for (x, val) in line.bytes().enumerate() {
            match val {
                b'S' => {
                    start = (x, y).into();
                    *grid.square_mut(start).unwrap() = 0;
                },
                b'E' => {
                    finish = (x, y).into();
                    *grid.square_mut(finish).unwrap() = b'z'-b'a'+2;
                }
                _ => *grid.square_mut((x, y).into()).unwrap() = val - b'a' + 1
            }
        }
    }
    (ElevationGrid(grid), start, finish)
}
```

The parsing function:
1. Creates a grid of the appropriate size
2. Processes each character in the input:
   - 'S' (start) is mapped to elevation 0 and its position is stored
   - 'E' (end) is mapped to elevation 27 and its position is stored
   - Letters 'a' to 'z' are mapped to values 1 to 26
3. Returns the grid and the start and end positions

### Breadth-First Search Implementation

```advent2022/src/bin/day12.rs#L254-295
fn shortest_path<F>(&self, start: Coord, goal:F ) -> Vec<Coord> where F: Fn(Coord)->bool {
    let mut ps = PathSearch::init(self);
    // push start in the queue
    ps.queue.push_back(start);

    // pop from top & while still nodes in the queue
    while let Some(cs) = ps.queue.pop_front() {
        // position matches target
        if goal(cs) {
            // extract parent position from target
            let mut cur = cs;
            while let Some(par) = ps.visited.square(cur).unwrap().1 {
                ps.path.push(par);
                cur = par;
            }
            // remove start position from path
            ps.path.pop();
            break
        }

        // mark square as visited
        ps.visited.square_mut(cs).unwrap().0 = true;

        let &square = self.0.square(cs).unwrap();

        // evaluate neighbour squares and
        // push to the queue if the have elevation delta <= 1
        self.0.neighbouring(cs)
            .for_each(|(ns, &elevation)| {
                if let Some((false, None)) = ps.visited.square(ns) {
                    if elevation <= square + 1 {
                        // capture the square we arrived from
                        ps.visited.square_mut(ns).unwrap().1 = Some(cs);
                        ps.queue.push_back(ns)
                    }
                }
            })
    }
    ps.path
}
```

The BFS algorithm:
1. Initializes a `PathSearch` instance with the grid dimensions
2. Adds the start position to the queue
3. Processes positions from the queue until finding one that satisfies the goal condition
4. For each position, visits neighboring positions that satisfy the elevation constraint
5. When the goal is reached, reconstructs the path by following parent pointers

### Elevation Reversal for Part 2

```advent2022/src/bin/day12.rs#L226-235
fn reverse_elevation(&mut self) {
    let &max = self.0.iter().max().unwrap();
    self.0.iter_mut()
        .map(|val|{
            *val = max - *val;
        })
        .all(|_| true);
}
```

This method reverses the elevation values, which allows solving Part 2 by starting from the end position and searching for the closest square with elevation 'a'.

### Path Visualization

```advent2022/src/bin/day12.rs#L236-241
fn visualise_path(&self, path:Vec<Coord>) {
    let mut gpath= ElevationGrid(Grid::new(self.width(), self.height()) );
    path.iter().for_each(|&a| *gpath.0.square_mut(a).unwrap() = *self.0.square(a).unwrap() );
    println!("Path length: {}\n{:?}",path.len(),gpath);
}
```

This method creates a new grid highlighting only the cells in the path, then prints it to the console.

### Interactive Visualization

The solution includes a sophisticated interactive visualization using the bracket-lib library. This allows exploring the map and watching the path-finding algorithm in action.

```advent2022/src/bin/day12.rs#L28-44
let mut ctx = BTermBuilder::simple(160,120)?
    .with_simple_console(grid.width(),grid.height(), "terminal8x8.png")
    .with_simple_console_no_bg(grid.width(),grid.height(), "terminal8x8.png")
    .with_simple_console_no_bg(grid.width(),grid.height(), "terminal8x8.png")
    .with_fps_cap(640f32)
    .with_title("Day12: Path Search")
    .build()?;

let ps = PathSearch::init(&grid);
let mut app = App::init(GStore { grid, target, start, ps } , Level::MENU);

app.register_level(Level::MENU, Menu);
app.register_level(Level::LEVEL1, ExerciseOne);
app.register_level(Level::LEVEL2, ExerciseTwo);

ctx.set_active_console(1);
app.store().grid.draw(&mut ctx);
main_loop(ctx, app)
```

This setup creates a visualization window with multiple layers and implements an interactive application with different levels.

## Main Solution Flow

```advent2022/src/bin/day12.rs#L11-26
let input = std::fs::read_to_string("src/bin/day12_input.txt").expect("ops!");

// parse elevations onto a grid
let (mut grid,start, target) = parse_elevation(input.as_str());

// find path with closure fn() goal set at reaching the target coordinate
let path = grid.shortest_path(start,|cs| cs.eq(&target));

// visualise path produced
grid.visualise_path(path);

// reverse the elevation so E(0) and S(27)
grid.reverse_elevation();

// find path with closure fn() goal set as reaching elevation(26) = a
let path = grid.shortest_path(target, |cs| 26.eq(grid.0.square(cs).unwrap()));

// visualise path produced
grid.visualise_path(path);
grid.reverse_elevation();
```

The main solution:

1. Parses the input into a grid and identifies start and end positions
2. For Part 1:
   - Finds the shortest path from start to end
   - Visualizes the path
3. For Part 2:
   - Reverses elevation values
   - Finds the shortest path from end to any position with elevation 'a'
   - Visualizes the path

## Implementation Notes

- **Goal Function**: The solution uses a closure as a goal function, making it flexible for both parts
- **Path Reconstruction**: The algorithm reconstructs the path by storing parent pointers in the `visited` grid
- **Interactive Visualization**: The solution includes a sophisticated visualization using bracket-lib
- **Functional Programming Style**: The code makes extensive use of iterators and functional programming patterns