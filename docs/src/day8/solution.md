# Day 8: Solution Explanation

## Approach

Day 8 involves analyzing a grid of trees to determine visibility and scenic scores. The solution breaks down into two main parts:

1. **Visibility Analysis**: Determine which trees are visible from outside the grid
2. **Scenic Score Calculation**: Calculate the scenic score for each tree and find the maximum

The key to solving both parts efficiently is to create appropriate data structures and algorithms for scanning the grid in different directions.

## Implementation Details

### Core Data Structures

#### Coordinates

First, the solution defines a `Coord` struct to represent positions in the grid:

```rust
#[derive(Debug,Copy, Clone)]
struct Coord {
    x: usize,
    y: usize
}
impl From<(usize,usize)> for Coord {
    fn from(p: (usize, usize)) -> Self {
        Coord { x:p.0, y:p.1 }
    }
}
```

This provides a clean way to handle grid positions and includes a convenient conversion from tuples.

#### Grid Structure

The core of the solution is a generic `Grid<T>` structure that can store any type of data in a 2D grid:

```rust
#[derive(Debug)]
struct Grid<T> {
    width: usize,
    height: usize,
    grid: Vec<T>,
}
```

The grid is stored as a flat vector for efficiency, with methods to access elements by coordinates:

```rust
fn tree(&self, p: Coord) -> Option<&T> {
    if !self.in_bounds(p) {
        return None
    }
    Some(&self.grid[p.y * self.width + p.x])
}
```

### Visibility Analysis

The visibility analysis is handled by the `Visibility` struct, which keeps track of which trees are visible:

```rust
#[derive(Debug)]
struct Visibility<'a> {
    forest: &'a Grid<i32>,       // Reference to the forest grid
    visible: Grid<bool>,         // Grid tracking visible trees
}
```

The key method is `scan_visibility`, which processes a sequence of coordinates in a given direction:

```rust
fn scan_visibility(&mut self, direction: ScanSequence) -> &mut Self {
    direction.into_iter()
        .for_each(|pos| {
            let mut tallest = -1;
            pos.into_iter().for_each(|e| {
                let tree = self.visible.tree_mut(e).unwrap();
                let t= self.forest.tree(e).unwrap();
                if tallest.lt(t) {
                    tallest = *t;
                    *tree = true;
                }
            });
        });
    self
}
```

This method:
1. Takes a sequence of coordinate sequences (representing scan lines)
2. For each scan line, tracks the tallest tree seen so far
3. Marks trees as visible if they're taller than all previous trees in the scan line

By calling this method with scan sequences from all four directions (left-to-right, right-to-left, top-to-bottom, bottom-to-top), we can determine all visible trees.

### Scenic Score Calculation

The scenic score calculation is handled by the `Scenic` struct:

```rust
#[derive(Debug)]
struct Scenic<'a> {
    forest: &'a Grid<i32>,
}
```

The main methods are:

```rust
fn scenic_score_dir(&mut self, p:Coord, (dx,dy):(isize,isize)) -> usize {
    let line = (1..).map_while(|i| {
        let coord = Coord {
            x: p.x.checked_add_signed(dx * i)?,
            y: p.y.checked_add_signed(dy * i)?,
        };
        self.forest.tree(coord)
    });

    let mut total = 0;
    let our_height = self.forest.tree(p).unwrap();
    for height in line {
        total += 1;
        if height >= our_height {
            break;
        }
    }
    total
}

fn scenic_score(&mut self, p: Coord) -> usize {
    let dirs =  [(-1, 0), (1, 0), (0, -1), (0, 1)];
    dirs.into_iter()
        .map(|dir| self.scenic_score_dir(p,dir) )
        .product()
}
```

These methods:
1. Calculate the viewing distance in a specific direction using `scenic_score_dir`
2. Combine the viewing distances in all four directions using `scenic_score`

The viewing distance calculation uses an infinite iterator with `map_while` to look in a specific direction until it reaches the edge or a blocking tree.

### Generating Scan Sequences

To scan the grid in different directions, the solution defines helper functions that generate sequences of coordinates:

```rust
fn left_to_right(f: &Grid<i32>) -> ScanSequence {
    (0..f.height)
        .map(|y| (0..f.width).map(move |x| (x, y).into()).collect::<Vec<Coord>>() )
        .collect::<Vec<_>>()
}

fn right_to_left(f: &Grid<i32>) -> ScanSequence {
    (0..f.height)
        .map(|y| (0..f.width).rev().map(move |x| (x, y).into()).collect::<Vec<Coord>>() )
        .collect::<Vec<_>>()
}

// Similar functions for top_to_bottom and bottom_to_up
```

Each function generates a sequence of scan lines, where each scan line is a sequence of coordinates.

### Parsing the Input

The input is parsed into a grid of tree heights:

```rust
fn parse_forest(data: &str) -> Grid<i32>  {
    let width = data.lines().next().unwrap().len();
    let height = data.lines().count();
    let mut grid = Grid::new(width,height);

    for (y,line) in data.lines().enumerate() {
        for (x, val) in line.bytes().enumerate() {
            *grid.tree_mut((x,y).into()).unwrap() = (val - b'0') as i32;
        }
    }
    grid
}
```

This converts each digit character to an integer height value.

### Main Solution

The main solution flow is:

```rust
fn main() {
    let data = std::fs::read_to_string("src/bin/day8_input.txt").expect("Ops!");
    let grid = parse_forest(data.as_str());

    // Part 1: Count visible trees
    let count = Visibility::new(&grid)
        .scan_visibility(left_to_right(&grid))
        .scan_visibility(top_to_bottom(&grid))
        .scan_visibility(right_to_left(&grid))
        .scan_visibility(bottom_to_up(&grid))
        .count_visible();
    println!("Total Visible = {:?}", count);

    // Part 2: Find maximum scenic score
    let mut scenic = Scenic::new(&grid);
    let max = left_to_right(&grid).into_iter()
        .flat_map(|x| x)
        .map(|p| scenic.scenic_score(p))
        .max().unwrap();
    println!("Max scenic = {:?}", max);
}
```

For Part 1, it scans the grid from all four directions and counts the visible trees. For Part 2, it calculates the scenic score for every tree and finds the maximum.

## Algorithm Analysis

### Time Complexity

- **Visibility Analysis**: O(n²) where n is the grid dimension (width or height), as we scan each cell in each direction
- **Scenic Score Calculation**: O(n³) in the worst case, as for each of the n² cells we might need to look n steps in each direction

### Space Complexity

- **Grid Storage**: O(n²) to store the forest grid and visibility grid
- **Scan Sequences**: O(n²) to store the coordinate sequences

## Alternative Approaches

### Single-Pass Visibility Check

For the visibility check, an alternative approach would be to use dynamic programming to precompute the maximum height seen from each direction:

```rust
// Precompute maximum heights from left
let mut max_left = vec![vec![-1; width]; height];
for y in 0..height {
    for x in 0..width {
        if x > 0 {
            max_left[y][x] = max(max_left[y][x-1], grid[y][x-1]);
        }
    }
}
// Similar for other directions

// Check visibility
for y in 0..height {
    for x in 0..width {
        if grid[y][x] > max_left[y][x] || grid[y][x] > max_right[y][x] || 
           grid[y][x] > max_top[y][x] || grid[y][x] > max_bottom[y][x] {
            visible[y][x] = true;
        }
    }
}
```

This would have the same asymptotic complexity but might be faster in practice due to better cache locality.

### Optimized Scenic Score Calculation

For the scenic score calculation, we could optimize by precomputing the viewing distance in each direction:

```rust
let mut view_distance = vec![vec![(0, 0, 0, 0); width]; height];

// Compute left viewing distances
for y in 0..height {
    let mut last_height = vec![0; 10];
    for x in 0..width {
        let h = grid[y][x] as usize;
        view_distance[y][x].0 = x - *last_height[..=h].iter().max().unwrap_or(&0);
        last_height[h] = x;
    }
}
// Similar for other directions
```

This would reduce the time complexity to O(n²), but would be more complex to implement.

## Conclusion

This solution demonstrates how to efficiently work with 2D grids and perform directional scanning operations. The use of custom data structures for coordinates and grids makes the code clean and maintainable, while the separation of visibility analysis and scenic score calculation into different structs keeps the code organized.