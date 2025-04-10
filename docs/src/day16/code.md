# Day 16: Code

Below is an explanation of the code for Day 16's solution, which finds the optimal valve opening sequence to maximize pressure release.

## Code Structure

The solution for Day 16 is quite complex and uses several key components:

1. `ValveNet`: Represents the network of valves and tunnels
2. `Valve`: Represents a single valve with its flow rate
3. `ValveBacktrack`: Implements the backtracking algorithm to find optimal paths
4. `Cache`: Provides efficient caching of distances between valves

## Key Components

### Valve and ValveNet Structures

```advent2022/src/bin/day16.rs#L233-242
struct Valve {
    pressure: usize,
    open: bool
}

struct ValveNet<'a> {
    graph: HashMap<&'a str,Vec<&'a str>>,
    flow: HashMap<&'a str, Valve>,
    cache: Cache<(&'a str, &'a str)>
}
```

The `Valve` struct represents a single valve with its flow rate and status. The `ValveNet` struct represents the entire network, using hashmaps to store the graph structure and valve information, along with a cache for distances.

### Valve Network Methods

The `ValveNet` implementation includes several key methods:

```advent2022/src/bin/day16.rs#L244-275
impl<'a> ValveNet<'a> {
    fn backtrack(&'a self) -> ValveBacktrack {
        ValveBacktrack {
            net: self,
            path: Vec::with_capacity(self.flow.len()),
            solution: Vec::with_capacity(self.flow.len()),
            pressure: 0,
            max: 0,
            time: Cell::new(std::time::SystemTime::now())
        }
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
    fn nonzero_valves(&self) -> Vec<&str> {
        self.flow.iter()
            .filter(|(_, v)| v.pressure > 0 )
            .fold( vec![],|mut out, (name, _)| {
                out.push(name);
                out
            })
    }
}
```

These methods set up the backtracking algorithm, build a cache of distances between valves, and identify the valves with non-zero flow rates.

### Backtracking Implementation

The core of the solution is the backtracking algorithm implemented in `ValveBacktrack`. For Part 2 (with an elephant), the implementation explores combinations of valve assignments:

```advent2022/src/bin/day16.rs#L54-90
fn combinations_elf_elephant(&mut self, time_left: &[usize], start: &[&'a str], valves: &[&'a str]) {

    // have we run out of valve destinations ?
    if valves.is_empty() {
        // we have a candidate solution; valve combination within 30"
        if self.max < self.pressure {
            self.max = self.pressure;
            self.solution = self.path.clone();
            self.solution.extend(start);

            let time = self.time.replace(std::time::SystemTime::now());
            print!("Found (EoV): {:?},{:?}", self.pressure, &self.path);
            println!(" - {:.2?},", std::time::SystemTime::now().duration_since(time).unwrap());
        }
        // END OF RECURSION HERE
        return;
    }

    // Entering a valves
    self.path.extend(start);

    // Run combinations of valves
    // valves visited by Elf
    (0..valves.len())
        .for_each( |elf| {
            // valves visited by Elephant
            (0..valves.len())
                .for_each(|elephant| {
                    // Are they both on the same valve ?
                    if elf == elephant {return;}

                    // pick the target valves to walk towards
                    let (elf_target,eleph_target) = ( valves[elf], valves[elephant] );

                    // Calculate costs to move to each valve
                    // Only proceed if there's enough time
                    // Calculate pressure released
                    // Recursive call with remaining valves
                    // Backtrack
                });
        });
}
```

This method recursively explores different combinations of valve assignments between the player and elephant, calculating the total pressure released for each combination.

### Distance Calculation

The solution calculates distances between valves using breadth-first search and caches the results for efficiency:

```advent2022/src/bin/day16.rs#L276-300
fn travel_distance(&self, start:&'a str, end:&'a str) -> Option<usize> {

    if let Some(cost) = self.cache.pull((start,end)) {
        return Some(cost)
    }

    let mut queue = VecDeque::new();
    let mut state: HashMap<&str,(bool,Option<&str>)> =
        self.flow.iter()
            .map(|(&key,_)| (key, (false, None)))
            .collect::<HashMap<_,_>>();
    let mut path_cost = 0;

    queue.push_back(start);
    while let Some(valve) = queue.pop_front() {

        if valve.eq(end) {
            let mut cur = valve;
            while let Some(par) = state[&cur].1 {
                path_cost += 1;
                cur = par;
            }
            path_cost += 1;
            self.cache.push((start, end), path_cost);
            // Return the found distance
        }
        
        // Process next valves in BFS
    }
}
```

This function performs a breadth-first search to find the shortest path between valves, then caches the result to avoid redundant calculations.

### Main Function

The main function sets up and runs the solution:

```advent2022/src/bin/day16.rs#L20-41
fn main() {

    // Found 2059,["AA", "II", "JI", "VC", "TE", "XF", "WT", "DM", "ZK", "KI", "VF", "DU", "BD", "XS", "IY"]
    let input = std::fs::read_to_string("src/bin/day16_input.txt").expect("ops!");
    let net = ValveNet::parse(input.as_str());

    let start = "AA";
    let mut valves = net.nonzero_valves();
    println!("Valves: {:?}",valves);

    valves.push(start);
    net.build_cache(&valves);
    valves.pop();

    let time = std::time::SystemTime::now();

    // create all valve visit order combinations
    let mut btrack = net.backtrack();
    btrack.combinations_elf_elephant(&[TIME-4,TIME-4], &[start,start], &valves);
    println!("Lapse time: {:?}",std::time::SystemTime::now().duration_since(time));
    println!("Max flow {:?}\nSolution: {:?}\n", btrack.max, (&btrack.solution,btrack.path));
}
```

The main function:
1. Parses the input to create the valve network
2. Identifies valves with non-zero flow rates
3. Builds a cache of distances between valves
4. Runs the backtracking algorithm for Part 2 (with an elephant)
5. Prints the maximum pressure that can be released and the optimal path

## Implementation Notes

- **Caching Strategy**: The solution uses extensive caching to avoid redundant calculations
- **Pruning**: The algorithm prunes paths that can't possibly lead to better solutions
- **Two-Actor Coordination**: The solution handles coordination between two actors (player and elephant) to avoid conflicting actions
- **Backtracking Approach**: The core algorithm uses a recursive backtracking approach to explore the solution space

The solution efficiently handles the complex optimization problem by focusing on the most relevant valves and using appropriate data structures and algorithms.