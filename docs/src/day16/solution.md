# Day 16: Solution Explanation

## Approach

Day 16 involves optimizing a sequence of valve openings to maximize the pressure released in a limited time. This is a complex optimization problem that can be approached in several ways. The solution uses a combination of techniques:

1. **Graph Representation**: Modeling the valve network as a graph where valves are nodes and tunnels are edges
2. **Distance Caching**: Pre-computing the distances between all relevant valves to avoid redundant calculations
3. **Recursive Backtracking**: Exploring different valve opening sequences to find the optimal solution
4. **Pruning**: Eliminating non-productive paths to reduce the search space

The key insight is recognizing that valves with zero flow rate never need to be opened, which significantly reduces the search space.

## Implementation Details

### Data Structures

The solution uses several key data structures:

#### ValveNet

This structure represents the network of valves and tunnels:

```rust
struct ValveNet<'a> {
    graph: HashMap<&'a str, Vec<&'a str>>,  // Adjacency list representation
    flow: HashMap<&'a str, Valve>,           // Flow rate for each valve
    cache: Cache<(&'a str, &'a str)>         // Distance cache
}
```

#### Valve

This structure represents a single valve:

```rust
struct Valve {
    pressure: usize,  // Flow rate
    open: bool        // Whether the valve is open
}
```

#### ValveBacktrack

This structure handles the backtracking algorithm to find the optimal solution:

```rust
struct ValveBacktrack {
    net: &'a ValveNet<'a>,        // Reference to the valve network
    path: Vec<&'a str>,           // Current path being explored
    solution: Vec<&'a str>,       // Best solution found so far
    max: usize,                   // Maximum pressure released
    pressure: usize,              // Current pressure in this path
    time: Cell<SystemTime>        // For timing the solution
}
```

### Preprocessing

Before running the main algorithm, the solution performs several preprocessing steps:

1. **Parsing the input**: Converting the text input into a graph representation
2. **Identifying relevant valves**: Finding all valves with non-zero flow rates
3. **Building a distance cache**: Pre-computing the distances between all relevant valves

```rust
fn nonzero_valves(&self) -> Vec<&str> {
    self.flow.iter()
        .filter(|(_, v)| v.pressure > 0)
        .fold(vec![], |mut out, (name, _)| {
            out.push(name);
            out
        })
}

fn build_cache(&self, valves: &[&'a str]) {
    for &a in valves {
        for &b in valves {
            if a != b {
                self.cache.push(
                    (a, b),
                    self.travel_distance(a, b).unwrap()
                );
            }
        }
    }
}
```

The distance cache is crucial for performance, as it allows the algorithm to quickly look up the time required to move between valves without recalculating paths.

### Distance Calculation

The distances between valves are calculated using breadth-first search (BFS):

```rust
fn travel_distance(&self, start: &'a str, end: &'a str) -> Option<usize> {
    // Check if distance is already cached
    if let Some(cost) = self.cache.pull((start, end)) {
        return Some(cost);
    }

    // Perform BFS to find shortest path
    let mut queue = VecDeque::new();
    let mut state: HashMap<&str, (bool, Option<&str>)> = /* initialize state */;

    queue.push_back(start);
    while let Some(valve) = queue.pop_front() {
        if valve.eq(end) {
            // Path found, calculate cost
            // ...
            return Some(path_cost);
        }

        // Process neighbors
        // ...
    }

    None // No path found
}
```

### Backtracking Algorithm for Part 1

The main algorithm for Part 1 (single player) uses backtracking to explore different valve opening sequences:

```rust
fn combinations_elf(&mut self, time_left: usize, start: &'a str, valves: &[&'a str]) {
    // Base case: no more valves to visit or no more time
    if valves.is_empty() || time_left == 0 {
        if self.max < self.pressure {
            self.max = self.pressure;
            self.solution = self.path.clone();
            // Update best solution
        }
        return;
    }

    // Try each remaining valve
    for (i, &valve) in valves.iter().enumerate() {
        // Calculate cost to move to valve and open it
        let cost = self.net.travel_distance(start, valve).unwrap() + 1;

        // Skip if not enough time
        if cost > time_left {
            continue;
        }

        // Calculate pressure released
        let new_time_left = time_left - cost;
        let pressure_released = self.net.flow[&valve].pressure * new_time_left;

        // Add to current path
        self.path.push(valve);
        self.pressure += pressure_released;

        // Recursive call with remaining valves
        let remaining_valves = valves.iter()
            .enumerate()
            .filter_map(|(j, &v)| if j != i { Some(v) } else { None })
            .collect::<Vec<&str>>();

        self.combinations_elf(new_time_left, valve, &remaining_valves);

        // Backtrack
        self.path.pop();
        self.pressure -= pressure_released;
    }
}
```

### Backtracking Algorithm for Part 2

For Part 2 (with an elephant), the algorithm is extended to handle two actors moving simultaneously:

```rust
fn combinations_elf_elephant(&mut self, time_left: &[usize], start: &[&'a str], valves: &[&'a str]) {
    // Base case: no more valves to visit
    if valves.is_empty() {
        if self.max < self.pressure {
            self.max = self.pressure;
            self.solution = self.path.clone();
            // Update best solution
        }
        return;
    }

    // Add current positions to path
    self.path.extend(start);

    // Try all combinations of valves for elf and elephant
    for elf in 0..valves.len() {
        for elephant in 0..valves.len() {
            // Skip if both try to visit the same valve
            if elf == elephant {
                continue;
            }

            let elf_target = valves[elf];
            let elephant_target = valves[elephant];

            // Calculate costs
            let elf_cost = self.net.travel_distance(start[0], elf_target).unwrap();
            let elephant_cost = self.net.travel_distance(start[1], elephant_target).unwrap();

            // Skip if not enough time
            if elf_cost > time_left[0] || elephant_cost > time_left[1] {
                continue;
            }

            // Calculate new time and pressure
            let elf_time = time_left[0] - elf_cost;
            let elephant_time = time_left[1] - elephant_cost;

            let pressure =
                self.net.flow[&elf_target].pressure * elf_time +
                self.net.flow[&elephant_target].pressure * elephant_time;

            // Add pressure
            self.pressure += pressure;

            // Recursive call with remaining valves
            let remaining_valves = valves.iter()
                .enumerate()
                .filter_map(|(i, &v)| if i != elf && i != elephant { Some(v) } else { None })
                .collect::<Vec<&str>>();

            self.combinations_elf_elephant(
                &[elf_time, elephant_time],
                &[elf_target, elephant_target],
                &remaining_valves
            );

            // Backtrack
            self.pressure -= pressure;
        }
    }

    // Remove current positions from path
    for _ in 0..start.len() {
        self.path.pop();
    }
}
```

This approach explores all possible combinations of valve assignments between the player and elephant.

### Optimizations

Several optimizations make the solution feasible:

1. **Filtering Zero-Flow Valves**: Only valves with non-zero flow rates are considered for opening
2. **Distance Caching**: Distances between valves are cached to avoid redundant calculations
3. **Early Pruning**: Paths that can't possibly beat the current best solution are pruned early
4. **Time Checking**: Valves that can't be reached in the remaining time are skipped

These optimizations significantly reduce the search space, making an otherwise intractable problem solvable in a reasonable time.

## Algorithm Analysis

### Time Complexity

The time complexity is primarily determined by the backtracking algorithm:

- **Part 1**: O(N!) where N is the number of non-zero flow valves, due to exploring all permutations
- **Part 2**: O(N! × N!) in the worst case, due to exploring all combinations of assignments between the player and elephant

However, the pruning optimizations significantly reduce the actual runtime.

### Space Complexity

- **Graph Representation**: O(V + E) where V is the number of valves and E is the number of tunnels
- **Distance Cache**: O(V²) for storing distances between all pairs of valves
- **Backtracking State**: O(V) for storing the current path and solution

## Alternative Approaches

### Dynamic Programming

A dynamic programming approach could potentially solve this problem by using a state representation that includes the current position, time remaining, and valves opened:

```rust
type State = (String, usize, BitSet);

fn max_pressure(state: State, memo: &mut HashMap<State, usize>) -> usize {
    // Base case
    if state.1 == 0 {
        return 0;
    }

    // Check memo
    if let Some(&result) = memo.get(&state) {
        return result;
    }

    // Calculate maximum pressure
    let mut best = 0;

    // Try opening the current valve
    // Try moving to each adjacent valve

    // Store result
    memo.insert(state, best);
    return best;
}
```

This approach would have a more predictable runtime but requires careful state representation to avoid memory issues.

### Greedy Algorithm

A simpler but **less optimal** approach would be a greedy algorithm that always chooses the valve with the highest potential pressure release (flow rate × remaining time after reaching it):

```rust
fn greedy_solution(net: &ValveNet, start: &str, time: usize) -> usize {
    let mut current = start;
    let mut time_left = time;
    let mut total_pressure = 0;
    let mut opened = HashSet::new();

    while time_left > 0 {
        // Find best valve to open next
        let best_valve = net.valves()
            .filter(|v| !opened.contains(v) && net.flow[v].pressure > 0)
            .max_by_key(|v| {
                let cost = net.distance(current, v) + 1;
                if cost >= time_left {
                    0
                } else {
                    net.flow[v].pressure * (time_left - cost)
                }
            });

        // No more valves worth opening
        if let Some(valve) = best_valve {
            // Move to valve and open it
            // Update state
        } else {
            break;
        }
    }

    total_pressure
}
```

This would run much faster but would likely produce **suboptimal** results.

## Conclusion

This solution demonstrates an effective approach to a complex optimization problem. By combining graph algorithms, caching, and backtracking with pruning, it finds the optimal valve opening sequence in a reasonable time. The extension to Part 2 shows how the algorithm can be adapted to handle multiple actors working simultaneously.
