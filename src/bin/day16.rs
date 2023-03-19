use std::cell::Cell;
use std::cmp::Ordering;
use std::hash::Hash;
use std::collections::{HashMap,vec_deque::VecDeque};
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

struct ValveBacktrack<'a> {
    net: &'a ValveNet<'a>,
    path: Vec<&'a str>,
    solution: Vec<&'a str>,
    max: usize,
    pressure: usize,
    time: Cell<std::time::SystemTime>
}

impl<'a> ValveBacktrack<'a> {

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

                        let (elf_cost, eleph_cost) = (
                            self.net.travel_distance(start[0], elf_target).unwrap(),
                            self.net.travel_distance(start[1], eleph_target).unwrap()
                        );

                        // do we have time to move to target valves ?
                        if elf_cost <= time_left[0] && eleph_cost <= time_left[1] {

                            let (elf_time, eleph_time) = ( time_left[0] - elf_cost, time_left[1] - eleph_cost );

                            // calculate the total pressure resulting from this move
                            let pressure=
                                self.net.flow[&elf_target].pressure * elf_time
                                    + self.net.flow[&eleph_target].pressure * eleph_time;

                            // Store the total pressure released
                            self.pressure += pressure;

                            // remove the elf & elephant targets from the valves to visit
                            let valves_remain= valves.iter()
                                .enumerate()
                                .filter_map(|(i,&v)| if i != elf && i != elephant {Some(v)} else { None } )
                                .collect::<Vec<&str>>();

                            // println!("\tElf:{:?}, Eleph:{:?} - {:?},[{:?},{:?}]",
                            //          (start[0], elf_target, elf_cost, time_left[0]),
                            //          (start[1], eleph_target, eleph_cost, time_left[1]),
                            //          (self.max,self.pressure+self.path_pressure(elf_time, &valves_remain)), (elf_target, eleph_target), &valves_remain
                            // );
                            self.combinations_elf_elephant(
                                &[elf_time, eleph_time],
                                &[elf_target, eleph_target],
                                &valves_remain
                            );
                            // we've finished with this combination hence remove from total pressure
                            self.pressure -= pressure;
                        } else {
                            // We've run out of time so we've finished and store the total pressure for this combination
                            if self.pressure > self.max {
                                self.max = self.pressure;
                                self.solution = self.path.clone();

                                let time = self.time.replace(std::time::SystemTime::now());
                                print!("Found (OoT): {:?},{:?}", self.pressure, self.path);
                                println!(" - {:.2?},", std::time::SystemTime::now().duration_since(time).unwrap());
                            }
                        }
                    });
            });
        // Leaving the valve we entered; finished testing combinations
        self.path.pop();
        self.path.pop();
    }
    fn combinations_elf(&mut self, time_left: usize, start: &'a str, valves: &[&'a str]) {

        // Is this the last valve to enter for current combination ?
        if valves.is_empty() {
            // we have a candidate solution; valve combination within 30"
            if self.max < self.pressure {
                self.max = self.pressure;
                self.solution = self.path.clone();
                self.solution.push(start);

                let time = self.time.replace(std::time::SystemTime::now());
                print!("Found (EoV): {:?},{:?}", self.pressure, self.path);
                println!("- {:.2?},", std::time::SystemTime::now().duration_since(time).unwrap());
            }
            // END OF RECURSION HERE
            return;
        }
        // Entering a valve
        self.path.push(start);

        // Run combinations of starting valve[0] against target valves, that is, valves[1..n]
        (0..valves.len())
            .for_each( |elf| {

                let cost = self.net.travel_distance(start, valves[elf]).unwrap();
                // do we have time to move to valve ?
                if time_left >= cost {
                    let time = time_left - cost;
                    let pressure = self.net.flow[ &valves[elf] ].pressure * time;
                    // Store the total pressure released up to this point / combination
                    self.pressure += pressure;

                    let valves_remain= valves
                        .iter()
                        .enumerate()
                        .filter_map(|(i,&v)| if i != elf { Some(v)} else { None } )
                        .collect::<Vec<&str>>();

                    // move to the next position with start:target[0], end:targets[1]
                    self.combinations_elf(time, valves[elf], &valves_remain);
                    // we've finished with this combination hence remove from total pressure
                    self.pressure -= pressure;

                } else {
                    // We've run out of time so we've finished and store the total pressure for this combination
                    if self.pressure > self.max {
                        self.max = self.pressure;
                        self.solution = self.path.clone();

                        let time = self.time.replace(std::time::SystemTime::now());
                        print!("Found (OoT): {:?},{:?}", self.pressure, self.path);
                        println!("- {:.2?},", std::time::SystemTime::now().duration_since(time).unwrap());
                    }
                }
            });
        // Leaving the valve we entered; finished testing combinations
        self.path.pop();
    }
    fn path_pressure(&self, mut time_left: usize, combinations: &[&'a str]) -> usize {
        combinations
            .windows(2)
            .map_while(|valves| {
                let cost = self.net.travel_distance(valves[0], valves[1]).unwrap();
                if time_left <  cost {
                    None
                } else {
                    time_left -= cost; // = len-1 steps + open valve
                    Some( self.net.flow[&valves[1]].pressure * time_left )
                }
            })
            .sum::<usize>()
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

#[derive(Copy, Clone)]
struct Valve {
    pressure: usize,
    open: bool
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
    fn greedy_search(&'a self, mut time_left:usize, start: &'a str) -> (usize,Vec<&'a str>) {

        let mut queue = VecDeque::new();
        let mut flow = self.flow.iter()
            .map(|(key,valve)| (key, valve.clone()))
            .collect::<HashMap<_,_>>();
        let mut path = vec![start];

        queue.push_back(start);

        let mut pressure = 0;

        while let Some(valve) = queue.pop_front() {

            flow.get_mut(&valve).unwrap().open = true;

            let mut options = flow.iter()
                .filter(|&(_,valve)| valve.pressure > 0  && !valve.open )
                .map(|(&target,_)|
                    (target, self.travel_distance(&valve, &target).unwrap())
                )
                .map(|(&target,cost)|
                    (target, cost, self.flow[target].pressure/cost)
                )
                .collect::<Vec<_>>();

            options.sort_by(|a,b|
                match a.2.cmp(&b.2) {
                    res@
                    (Ordering::Less | Ordering::Greater) => res,
                    Ordering::Equal => b.1.cmp(&a.1)
                }
            );

            if let Some((valve,cost,value)) = options.pop() {
                path.push(valve);
                if time_left < cost {
                    path.extend(options.iter().map(|&(v,..)| v).rev());
                    return (pressure,path)
                }
                time_left -= cost;
                pressure += self.flow[&valve].pressure * time_left;
                println!("====> Time: {time_left} got for Option {:?} out of Options: {:?}", (&valve, cost, value, time_left, pressure), options);
                queue.push_back(valve);
            }
        }
        (pressure,path)
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
                        open: false
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_greedy_vs_backtrack() {
        let net = ValveNet::parse(INPUT);

        let start = "AA";
        let valves = net.flow.iter()
            .filter(|(_, v)| v.pressure > 0)
            .fold(vec![start], |mut out, (name, _)| {
                out.push(name);
                out
            });
        println!("Valves: {:?}", valves);

        let mut time = std::time::SystemTime::now();
        let (max_seed, solution_seed) = net.greedy_search(TIME, start);
        println!(" - {:.2?},", std::time::SystemTime::now().duration_since(time).unwrap());
        println!("Pressure (Greedy): {}\nPath: {:?}", max_seed, solution_seed);

        time = std::time::SystemTime::now();
        let backtrack = net.backtrack();
        let pressure = backtrack.path_pressure(TIME, &["AA", "DD", "BB", "JJ", "HH", "EE", "CC"]);
        println!(" - {:.2?},", std::time::SystemTime::now().duration_since(time).unwrap());
        println!("Pressure (Backtrack): {}", pressure);
        assert_eq!(pressure,1651);
    }
    #[test]
    fn test_sample_set_elf() {
        // Found 1651, ["AA", "DD", "BB", "JJ", "HH", "EE", "CC"]
        assert_eq!( test_backtrack_elf(INPUT), 1651)
    }
    #[test]
    fn test_large_set_elf() {
        // Found 2059,["AA", "II", "JI", "VC", "TE", "XF", "WT", "DM", "ZK", "KI", "VF", "DU", "BD", "XS", "IY"]
        let input = std::fs::read_to_string("src/bin/day16_input.txt").expect("ops!");
        assert_eq!( test_backtrack_elf(input.as_str()), 2059)
    }

    fn test_backtrack_elf(input: &str) -> usize {

        let net = ValveNet::parse(input);
        let valves = net.nonzero_valves();
        net.build_cache(&valves);

        let time = std::time::SystemTime::now();
        // create all valve visit order combinations
        let mut btrack = net.backtrack();
        btrack.combinations_elf(TIME, "AA", &valves);

        println!("Valves: {:?}",valves);
        println!("Lapse time: {:?}",std::time::SystemTime::now().duration_since(time));
        println!("Max flow {:?}\nSolution: {:?}\n", &btrack.max, &btrack.solution);

        btrack.max
    }

    #[test]
    fn test_sample_set_elf_elephant() {
        // Found 1651, ["AA", "DD", "BB", "JJ", "HH", "EE", "CC"]
        assert_eq!( test_backtrack_elf_elephant(INPUT), 1707)
    }
    #[test]
    fn test_large_set_elf_elephant() {
        // Found 2059,["AA", "II", "JI", "VC", "TE", "XF", "WT", "DM", "ZK", "KI", "VF", "DU", "BD", "XS", "IY"]
        let input = std::fs::read_to_string("src/bin/day16_input.txt").expect("ops!");
        assert_eq!( test_backtrack_elf_elephant(input.as_str()), 2790)
    }

    fn test_backtrack_elf_elephant(input:&str) -> usize {

        let net = ValveNet::parse(input);
        let valves = net.nonzero_valves();

        net.build_cache(&valves);

        let time = std::time::SystemTime::now();

        // create all valve visit order combinations
        let mut btrack = net.backtrack();
        btrack.combinations_elf_elephant(&[TIME-4,TIME-4], &["AA","AA"], &valves);

        println!("Valves: {:?}",valves);
        println!("Lapse time: {:?}",std::time::SystemTime::now().duration_since(time));
        println!("Max flow {:?}\nSolution: {:?}\n", btrack.max, (&btrack.solution,btrack.path));

        btrack.max
    }
}