use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::os::raw::c_void;
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
    println!("Valves: {:?}", valves );

    let (max_seed, solution_seed) = net.greedy_search(TIME, start);
    println!("Pressure (Greedy): {}\nPath: {:?}", max_seed, solution_seed);

    // create all valve visit order combinations
    let mut btrack = net.backtrack();
    btrack.combinations_v2(TIME, &solution_seed);
    println!("Solutions: {:?}\nMax flow {:?}", (btrack.solution, btrack.pressure), btrack.max);
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

    fn combinations_v2(&mut self, mut time_left: usize, valves: &[&'a str]) {

        self.path.push(valves[0]);

        if valves.len() == 1 {
            let time = self.time.replace(std::time::SystemTime::now());
            print!("\nIn time found: {:?},{:?}",self.pressure.last(), self.path);
            print!("- {:.2?},", std::time::SystemTime::now().duration_since(time).unwrap());

            let &total = self.pressure.last().unwrap();
            if total > self.max {
                self.max = total;
                self.solution = self.path.clone();
                print!("<<< MAX\n");
            }
            self.path.pop();
            return;
        }

        let mut tmp = valves[1..].to_vec();
        (0..tmp.len())
            .for_each(|i|{
                tmp.swap(0,i);

                let target = tmp[0];
                let cost = self.net.find_path_cost(valves[0], target).unwrap();
                if time_left >= cost {

                    let time = time_left - cost;
                    let a = self.pressure.last().unwrap();
                    let b = self.net.flow[&target].pressure * time;
                    self.pressure.push(a+b);
                    self.combinations_v2(time,&tmp);
                    self.pressure.pop();

                } else {
                    let &total = self.pressure.last().unwrap();
                    if total > self.max {
                        self.max = total;
                        self.solution = self.path.clone();

                        let time = self.time.replace(std::time::SystemTime::now());
                        print!("\nRun out of time: {:?},{:?}",self.pressure.last(), self.path);
                        println!("- {:.2?},", std::time::SystemTime::now().duration_since(time).unwrap());
                    }
                }
            });
        self.path.pop();
    }

    fn combinations(&mut self, valves: &[&'a str]) {
        self.path.push(valves[0]);

        if valves.len() == 1 {
            // ok we got potential solution, store it
            let pressure = self.path_pressure(TIME, &self.path);
            if pressure > self.max {
                println!("Found {},{:?}",pressure, self.path);
                self.max = pressure;
                self.solution = self.path.clone();
            }
            self.path.pop();
            return;
        }

        let mut tmp = valves[1..].to_vec();
        (0..valves[1..].len())
            .for_each(|i|{
                tmp.swap(0,i);
                self.combinations(&tmp);
            });

        self.path.pop();
    }
    fn path_pressure(&self, mut time_left: usize, combinations: &[&'a str]) -> usize {
        combinations
            .windows(2)
            .map_while(|valves| {
                let target = valves[1];
                let cost = self.net.find_path_cost(valves[0], target).unwrap();
                if time_left <  cost {
                    None
                } else {
                    time_left -= cost; // = len-1 steps + open valve
                    Some( self.net.flow[&target].pressure * time_left )
                }
            })
            .sum::<usize>()
    }
}

struct Cache<'a> {
    cache: Cell<HashMap<(&'a str,&'a str),usize>>
}
impl<'a> Cache<'a> {
    fn pull(&self, start: &str, end: &str) -> Option<usize> {
        let cache = self.cache.take();
        let out = cache.get(&(start, end)).and_then(|cost| Some(*cost));
        self.cache.set(cache);
        out
    }
    fn push(&'a self, start: &'a str, end: &'a str, cost:usize) {
        let mut cache = self.cache.take();
        cache.insert((start, end),cost);
        cache.insert((end, start),cost);
        self.cache.set(cache);
    }
}

#[derive(Debug, Copy, Clone)]
struct Valve {
    pressure: usize,
    open: bool
}

struct ValveNet<'a> {
    graph: HashMap<&'a str,Vec<&'a str>>,
    flow: HashMap<&'a str, Valve>,
    cache: Cache<'a>,
    time: Cell<std::time::SystemTime>
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
    fn nonzero_valves(&self, start:&'a str) -> Vec<&str> {
        self.flow.iter()
            .filter(|(_, v)| v.pressure > 0 )
            .fold( vec![start],|mut out, (name, _)| {
                out.push(name);
                out
            })
    }
    fn find_path_cost(&'a self, start:&'a str, end:&'a str) -> Option<usize> {

        if let Some(cost) = self.cache.pull(start,end) {
            return Some(cost)
        }

        let time = self.time.replace(std::time::SystemTime::now());
        println!("missed cache {:?} {:.2?},", (start, end), std::time::SystemTime::now().duration_since(time).unwrap());

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
                self.cache.push(start, end, path_cost);
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
                    (target, self.find_path_cost(&valve, &target).unwrap())
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

        ValveNet { graph, flow, cache: Cache { cache: Cell::new(HashMap::new()) }, time: Cell::new(std::time::SystemTime::now()) }
    }
}

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
        net.build_cache(&valves);
        // let (max_seed, solution_seed) = net.greedy_search(TIME, start);
        // println!("Pressure (Greedy): {}\nPath: {:?}", max_seed, solution_seed);

        let backtrack = net.backtrack();
        let pressure = backtrack.path_pressure(TIME, &["AA", "DD", "BB", "JJ", "HH", "EE", "CC"]);
        println!("Pressure (Backtrack): {}", pressure);
    }
}