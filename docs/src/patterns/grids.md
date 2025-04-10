# Grid-based Problems

Many Advent of Code challenges involve working with 2D grids. This page covers common techniques for handling grid-based problems.

## Grid Representation

There are several ways to represent a grid in Rust:

```rust
// 1. Vector of vectors
type Grid<T> = Vec<Vec<T>>;

// 2. Flat vector with computed indices
struct Grid {
    cells: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Option<&char> {
        if x < self.width && y < self.height {
            self.cells.get(y * self.width + x)
        } else {
            None
        }
    }
}

// 3. HashMap with coordinates as keys
use std::collections::HashMap;
type Grid<T> = HashMap<(i32, i32), T>;
```

## Grid Navigation

### Cardinal Directions

```rust
const DIRECTIONS: [(i32, i32); 4] = [
    (0, 1),   // South
    (1, 0),   // East
    (0, -1),  // North
    (-1, 0),  // West
];

fn get_neighbors(x: i32, y: i32) -> Vec<(i32, i32)> {
    DIRECTIONS.iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .collect()
}
```

### Including Diagonals

```rust
const DIRECTIONS_WITH_DIAGONALS: [(i32, i32); 8] = [
    (0, 1),   // South
    (1, 1),   // Southeast
    (1, 0),   // East
    (1, -1),  // Northeast
    (0, -1),  // North
    (-1, -1), // Northwest
    (-1, 0),  // West
    (-1, 1),  // Southwest
];
```

## Grid Traversal

### Simple Iteration

```rust
fn iterate_grid<F>(grid: &Vec<Vec<char>>, mut f: F)
where
    F: FnMut(usize, usize, char)
{
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            f(x, y, grid[y][x]);
        }
    }
}
```

### Flood Fill

```rust
use std::collections::VecDeque;

fn flood_fill(grid: &mut Vec<Vec<char>>, start_x: usize, start_y: usize, fill_char: char) {
    let target_char = grid[start_y][start_x];
    if target_char == fill_char {
        return;
    }
    
    let height = grid.len();
    let width = grid[0].len();
    let mut queue = VecDeque::new();
    queue.push_back((start_x, start_y));
    
    while let Some((x, y)) = queue.pop_front() {
        if grid[y][x] != target_char {
            continue;
        }
        
        grid[y][x] = fill_char;
        
        // Check four adjacent cells
        if x > 0 { queue.push_back((x - 1, y)); }
        if y > 0 { queue.push_back((x, y - 1)); }
        if x + 1 < width { queue.push_back((x + 1, y)); }
        if y + 1 < height { queue.push_back((x, y + 1)); }
    }
}
```

## Visibility and Line-of-Sight

```rust
// Check visibility in four directions
fn is_visible(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let height = grid.len();
    let width = grid[0].len();
    let value = grid[y][x];
    
    // Edge cells are always visible
    if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
        return true;
    }
    
    // Check from left
    let visible_from_left = (0..x).all(|i| grid[y][i] < value);
    if visible_from_left {
        return true;
    }
    
    // Check from right
    let visible_from_right = (x+1..width).all(|i| grid[y][i] < value);
    if visible_from_right {
        return true;
    }
    
    // Check from top
    let visible_from_top = (0..y).all(|i| grid[i][x] < value);
    if visible_from_top {
        return true;
    }
    
    // Check from bottom
    let visible_from_bottom = (y+1..height).all(|i| grid[i][x] < value);
    if visible_from_bottom {
        return true;
    }
    
    false
}
```

## Performance Considerations

1. **Pre-allocate memory** when building grids to avoid resizing
2. **Use flat arrays** for large, dense grids to improve cache locality
3. **Bound checking** can be expensive; consider wrapping grid access in checked functions
4. **Consider sparse representation** (HashMaps) for very large grids with few occupied cells