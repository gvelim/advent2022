use std::cell::Cell;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::str::FromStr;

const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

const TIME:usize = 30;

fn main() {

    // Found 2059,["AA", "II", "JI", "VC", "TE", "XF", "WT", "DM", "ZK", "KI", "VF", "DU", "BD", "XS", "IY"]
    let input = std::fs::read_to_string("src/bin/day16_input.txt").expect("ops!");
    let net = ValveNet::parse(input.as_str());

    let start = "AA";
    let valves = net.nonzero_valves(start);
    println!("Valves: {:?}",valves);

    net.build_cache(&valves);

    let time = std::time::SystemTime::now();

    // create all valve visit order combinations
    let mut btrack = net.backtrack();
    btrack.combinations_dfs(TIME, &valves);
    println!("Lapse time: {:?}",std::time::SystemTime::now().duration_since(time));
    println!("Max flow {:?}\nSolution: {:?}\n", btrack.max, btrack.solution);

}

struct ValveBacktrack<'a> {
    net: &'a ValveNet<'a>,
    path: Vec<&'a str>,
    solution: Vec<&'a str>,
    pressure: Vec<usize>,
    max: usize,
    time: Cell<std::time::SystemTime>
}

impl<'a> ValveBacktrack<'a> {

    fn combinations_dfs(&mut self, time_left: usize, valves: &[&'a str]) {

        // Entering a valve
        self.path.push(valves[0]);

        let &total_pressure_now = self.pressure.last().unwrap();

        // Is this the last valve to enter for current combination ?
        if valves.len() == 1 {
            // we have a candidate solution; valve combination within 30"
            if self.max < total_pressure_now {
                self.max = total_pressure_now;
                self.solution = self.path.clone();
            }
            // Leaving the valve we've entered
            self.path.pop();
            // END OF RECURSION HERE
            return;
        }

        // Run combinations of starting valve[0] against target valves, that is, valves[1..n]
        let mut targets = valves[1..].to_vec();
        (0..targets.len())
            .for_each(|i|{
                // put i'th target always first by swapping
                targets.swap(0, i);

                let cost = self.net.travel_distance(valves[0], targets[0]).unwrap();
                // do we have time to move to valve ?
                if time_left >= cost {
                    // Store the total pressure released up to this point / combination
                    self.pressure.push(
                        total_pressure_now +
                            self.net.flow[&targets[0]].pressure * (time_left - cost)
                    );
                    // move to the next position with start:target[0], end:targets[1]
                    self.combinations_dfs(time_left - cost, &targets);
                    // we've finished with this combination hence remove from total pressure
                    self.pressure.pop();

                } else {
                    // We've run out of time so we've finished and store the total pressure for this combination
                    if total_pressure_now > self.max {
                        self.max = total_pressure_now;
                        self.solution = self.path.clone();

                        let time = self.time.replace(std::time::SystemTime::now());
                        print!("Found (OoT): {:?},{:?}", total_pressure_now, self.path);
                        println!("- {:.2?},", std::time::SystemTime::now().duration_since(time).unwrap());
                    }
                }
            });
        // Leaving the valve we entered; finished testing combinations
        self.path.pop();
    }
}

struct Cache<T> where T: Eq + Hash {
    cache: Cell<HashMap<T,usize>>
}
impl<T> Cache<T> where T: Eq + Hash {
    fn pull(&self, key: T) -> Option<usize> {
        let cache = self.cache.take();
        let out = cache.get(&key).copied();
        self.cache.set(cache);
        out
    }
    fn push(&self, key: T, cost: usize) {
        let mut cache = self.cache.take();
        cache.insert(key,cost);
        self.cache.set(cache);
    }
}

struct Valve {
    pressure: usize,
    _open: bool
}

struct ValveNet<'a> {
    graph: HashMap<&'a str,Vec<&'a str>>,
    flow: HashMap<&'a str, Valve>,
    cache: Cache<(&'a str, &'a str)>
}

impl<'a> ValveNet<'a> {
    fn backtrack(&'a self) -> ValveBacktrack {
        ValveBacktrack {
            net: self,
            path: vec![],
            solution: vec![],
            pressure:vec![0],
            max: 0,
            time: Cell::new(std::time::SystemTime::now()) }
    }
    fn build_cache(&self, valves: &[&'a str]) {
        for &a in valves {
            for &b in valves {
                self.cache.push(
                    (a,b),
                    self.travel_distance(a, b).unwrap()
                );
            }
        }

    }
    fn nonzero_valves(&self, start:&'a str) -> Vec<&str> {
        self.flow.iter()
            .filter(|(_, v)| v.pressure > 0 )
            .fold( vec![start],|mut out, (name, _)| {
                out.push(name);
                out
            })
    }
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
                return Some(path_cost);
            }
            state.get_mut(valve).unwrap().0 = true;
            for &v in &self.graph[valve] {
                if !state[v].0 {
                    state.get_mut(v).unwrap().1 = Some(valve);
                    queue.push_back(v)
                }
            }
        }
        None
    }

    fn parse(input: &str) -> ValveNet {
        let (graph, flow) = input.lines()
            .map(|line| {
                line.split(&[' ','=',';',','])
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
            })
            .map(|s| (s[1],s[5],s[10..].to_vec()))
            .fold( (HashMap::new(),HashMap::new()),|(mut g, mut f),(key, flow, edges)| {
                f.entry(key).or_insert(
                    Valve {
                        pressure: usize::from_str(flow).expect("Cannot convert flow"),
                        _open: false
                    }
                );
                edges.into_iter()
                    .for_each(|edge|
                        g.entry(key)
                            .or_insert(Vec::new())
                            .push(edge)
                    );
                (g,f)
            });

        ValveNet { graph, flow, cache: Cache { cache: Cell::new(HashMap::new()) } }
    }
}
