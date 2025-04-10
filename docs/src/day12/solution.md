# Day 12: Solution Explanation

## Approach

Day 12 involves finding the shortest path through a grid with elevation constraints. The key to solving this problem is to use a breadth-first search (BFS) algorithm, which is optimal for finding the shortest path in an unweighted graph.

The solution breaks down into several key components:

1. **Representing the heightmap**: We need to parse the input into a grid of elevation values
2. **Implementing BFS**: We need to find the shortest path from start to end, respecting elevation constraints
3. **Reversing the problem for Part 2**: We can efficiently solve Part 2 by starting from the end point and finding the closest square with elevation 'a'
4. **Visualizing the path**: As a bonus, the solution includes visualization using the bracket-lib library

## Implementation Details

### Grid Representation

The solution uses a custom `Grid` structure to represent the heightmap:

```rust
struct ElevationGrid(Grid<u8>);
```

This wraps a generic `Grid<u8>` from a shared library, with elevation values represented as unsigned bytes. During parsing, letters 'a' to 'z' are converted to values 1 to 26, with 'S' (start) mapped to 0 and 'E' (end) mapped to 27.

### Parsing the Input

The input is parsed into an `ElevationGrid`, with special handling for the start ('S') and end ('E') positions:

```rust
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

This function returns the grid, start coordinate, and end coordinate.

### Path Finding with BFS

The core of the solution is the `shortest_path` method on `ElevationGrid`, which implements BFS to find the shortest path satisfying a given goal condition:

```rust
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

Key aspects of this implementation:

1. It uses a queue for BFS traversal, starting from the specified position
2. It checks each position against a goal function passed as a parameter
3. It respects the elevation constraint (can only move to positions with elevation at most 1 higher)
4. It reconstructs the path from end to start using parent pointers

### Path Search Data Structure

The BFS algorithm is supported by a `PathSearch` struct that manages the search state:

```rust
struct PathSearch {
    queue: VecDeque<Coord>,
    visited: Grid<(bool,Option<Coord>)>,
    path: Vec<Coord>
}
```

This structure maintains:
- A queue of coordinates to explore
- A grid tracking visited positions and their parent positions (for path reconstruction)
- A vector to store the final path

### Solving Part 1

For Part 1, we find the shortest path from the start position to the end position:

```rust
// find path with closure fn() goal set at reaching the target coordinate
let path = grid.shortest_path(start, |cs| cs.eq(&target));
```

We use a closure that checks if the current position matches the target position.

### Solving Part 2

For Part 2, we need to find the shortest path from any position with elevation 'a' to the end position. Instead of running BFS from each possible starting position, we reverse the problem:

```rust
// reverse the elevation so E(0) and S(27)
grid.reverse_elevation();

// find path with closure fn() goal set as reaching elevation(26) = a
let path = grid.shortest_path(target, |cs| 26.eq(grid.0.square(cs).unwrap()));
```

This elegant approach:
1. Reverses the elevation values (making 'a' the highest and 'z' the lowest)
2. Starts BFS from the end position
3. Looks for the first position with elevation value 26 (which corresponds to 'a' after reversal)

The elevation reversal is implemented as:

```rust
fn reverse_elevation(&mut self) {
    let &max = self.0.iter().max().unwrap();
    self.0.iter_mut()
        .map(|val|{
            *val = max - *val;
        })
        .all(|_| true);
}
```

This effectively flips the elevation constraint, allowing us to find the shortest path from the end position to any 'a' position.

### Visualization

The solution includes visualization using the bracket-lib library, which renders the grid and path in a graphical window. This is not essential for solving the problem but provides a nice way to see the results.

## Algorithmic Analysis

### Time Complexity

- **BFS**: O(V + E) where V is the number of vertices (grid cells) and E is the number of edges (adjacent cell pairs). In a grid, this simplifies to O(n) where n is the number of cells.
- **Path Reconstruction**: O(p) where p is the length of the path.
- **Overall**: O(n) for each part, where n is the number of grid cells.

### Space Complexity

- **Grid Storage**: O(n) to store the grid
- **BFS Data Structures**: O(n) for the queue and visited tracking
- **Path Storage**: O(p) where p is the path length
- **Overall**: O(n)

## Alternative Approaches

### Dijkstra's Algorithm or A*

While BFS is optimal for unweighted graphs, we could also use Dijkstra's algorithm or A* if we wanted to add more complex cost calculations. For example:

```rust
fn shortest_path_astar(&self, start: Coord, end: Coord) -> Vec<Coord> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();
    
    g_score.insert(start, 0);
    f_score.insert(start, manhattan_distance(start, end));
    open_set.push(Node { pos: start, f_score: *f_score.get(&start).unwrap() });
    
    // A* algorithm implementation...
}

fn manhattan_distance(a: Coord, b: Coord) -> u32 {
    ((a.x as i32 - b.x as i32).abs() + (a.y as i32 - b.y as i32).abs()) as u32
}
```

However, for this problem, BFS is sufficient and more efficient.

### Dynamic Programming

Another approach could be to use dynamic programming to calculate the shortest distance to each cell from the starting point:

```rust
fn shortest_distance_dp(&self, start: Coord) -> Grid<Option<usize>> {
    let mut distances = Grid::new(self.width(), self.height());
    distances.square_mut(start).unwrap() = Some(0);
    
    let mut changed = true;
    while changed {
        changed = false;
        // For each cell, update distances based on neighbors
        // ...
    }
    
    distances
}
```

This would be more complex and less efficient than BFS for this problem.

## Conclusion

This solution demonstrates an efficient approach to pathfinding in a grid with elevation constraints. By using BFS and cleverly reversing the problem for Part 2, we achieve a clean and performant solution. The visualization component adds an interesting way to see the results of the algorithm in action.