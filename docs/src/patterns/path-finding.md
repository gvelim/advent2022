# Path Finding

Path finding algorithms are crucial for solving many Advent of Code challenges, especially those involving navigating through mazes, graphs, or finding optimal routes.

## Breadth-First Search (BFS)

BFS is ideal for finding the shortest path in an unweighted graph:

```rust
use std::collections::{HashSet, VecDeque};

fn bfs<N, FN, FS>(
    start: N,
    get_neighbors: FN,
    is_goal: FS,
) -> Option<usize>
where
    N: Clone + Eq + std::hash::Hash,
    FN: Fn(&N) -> Vec<N>,
    FS: Fn(&N) -> bool,
{
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    queue.push_back((start.clone(), 0)); // (node, distance)
    visited.insert(start);
    
    while let Some((node, distance)) = queue.pop_front() {
        if is_goal(&node) {
            return Some(distance);
        }
        
        for neighbor in get_neighbors(&node) {
            if visited.insert(neighbor.clone()) {
                queue.push_back((neighbor, distance + 1));
            }
        }
    }
    
    None // No path found
}

// Example usage for a 2D grid
fn find_shortest_path(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let height = grid.len();
    let width = grid[0].len();
    
    bfs(
        start,
        |&(x, y)| {
            let mut neighbors = Vec::new();
            // Check four adjacent cells
            if x > 0 { neighbors.push((x - 1, y)); }
            if y > 0 { neighbors.push((x, y - 1)); }
            if x + 1 < width { neighbors.push((x + 1, y)); }
            if y + 1 < height { neighbors.push((x, y + 1)); }
            
            // Filter only passable cells
            neighbors.into_iter()
                .filter(|&(nx, ny)| grid[ny][nx] != '#')
                .collect()
        },
        |&pos| pos == end
    )
}
```

## Depth-First Search (DFS)

DFS is useful for exploring all possible paths or maze solving:

```rust
fn dfs<N, FN, FS>(
    node: N,
    get_neighbors: &FN,
    is_goal: &FS,
    visited: &mut HashSet<N>,
) -> bool
where
    N: Clone + Eq + std::hash::Hash,
    FN: Fn(&N) -> Vec<N>,
    FS: Fn(&N) -> bool,
{
    if is_goal(&node) {
        return true;
    }
    
    visited.insert(node.clone());
    
    for neighbor in get_neighbors(&node) {
        if !visited.contains(&neighbor) {
            if dfs(neighbor, get_neighbors, is_goal, visited) {
                return true;
            }
        }
    }
    
    false
}
```

## Dijkstra's Algorithm

Dijkstra's algorithm finds the shortest path in a weighted graph:

```rust
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<T> {
    cost: usize,
    node: T,
}

impl<T: Eq> Ord for State<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<T: Eq> PartialOrd for State<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra<N, FN>(
    start: N,
    goal: N,
    get_neighbors: FN,
) -> Option<usize>
where
    N: Clone + Eq + std::hash::Hash,
    FN: Fn(&N) -> Vec<(N, usize)>, // Returns (neighbor, cost)
{
    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();
    
    distances.insert(start.clone(), 0);
    heap.push(State { cost: 0, node: start });
    
    while let Some(State { cost, node }) = heap.pop() {
        if node == goal {
            return Some(cost);
        }
        
        if let Some(&best_cost) = distances.get(&node) {
            if cost > best_cost {
                continue; // We've found a better path to this node
            }
        }
        
        for (neighbor, edge_cost) in get_neighbors(&node) {
            let new_cost = cost + edge_cost;
            let is_better = distances
                .get(&neighbor)
                .map_or(true, |&current| new_cost < current);
                
            if is_better {
                distances.insert(neighbor.clone(), new_cost);
                heap.push(State { cost: new_cost, node: neighbor });
            }
        }
    }
    
    None // No path found
}
```

## A* Algorithm

A* improves on Dijkstra by using a heuristic to guide the search:

```rust
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<T> {
    f_score: usize, // f = g + h (total cost + heuristic)
    g_score: usize, // g = cost from start to current
    node: T,
}

impl<T: Eq> Ord for State<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl<T: Eq> PartialOrd for State<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn a_star<N, FN, H>(
    start: N,
    goal: N,
    get_neighbors: FN,
    heuristic: H,
) -> Option<usize>
where
    N: Clone + Eq + std::hash::Hash,
    FN: Fn(&N) -> Vec<(N, usize)>, // Returns (neighbor, cost)
    H: Fn(&N, &N) -> usize, // Heuristic function
{
    let mut g_scores = HashMap::new();
    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();
    
    g_scores.insert(start.clone(), 0);
    open_set.push(State { 
        f_score: heuristic(&start, &goal),
        g_score: 0,
        node: start.clone() 
    });
    
    while let Some(State { g_score, node, .. }) = open_set.pop() {
        if node == goal {
            return Some(g_score);
        }
        
        if !closed_set.insert(node.clone()) {
            continue; // Already evaluated
        }
        
        for (neighbor, edge_cost) in get_neighbors(&node) {
            if closed_set.contains(&neighbor) {
                continue;
            }
            
            let tentative_g = g_score + edge_cost;
            
            let is_better = g_scores
                .get(&neighbor)
                .map_or(true, |&current| tentative_g < current);
                
            if is_better {
                g_scores.insert(neighbor.clone(), tentative_g);
                let f_score = tentative_g + heuristic(&neighbor, &goal);
                open_set.push(State { 
                    f_score, 
                    g_score: tentative_g, 
                    node: neighbor 
                });
            }
        }
    }
    
    None // No path found
}

// Example heuristic for 2D grid (Manhattan distance)
fn manhattan_distance(a: &(i32, i32), b: &(i32, i32)) -> usize {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as usize
}
```

## Choosing the Right Algorithm

- **BFS**: For unweighted graphs or when all edges have equal weight
- **DFS**: When you need to explore all possible paths or find any valid path
- **Dijkstra**: For weighted graphs when you need the shortest path
- **A***: For weighted graphs when you have a good heuristic to guide the search