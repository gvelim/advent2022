// #![feature(let_chains)]
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
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

const BUDGET:usize = 30;

fn main() {

    struct ValveBacktrack<'a> {
        path: Vec<&'a str>,
        solution: Vec<&'a str>,
        max: usize

    }
    impl<'a> ValveBacktrack<'a> {
        fn combinations(&mut self, net: &ValveNet, valves: &[&'a str]) {

            self.path.push(valves[0]);

            if valves.len() == 1 {
                // ok we got potential solution, store it
                let pressure = path_pressure(&net, BUDGET, &self.path);
                if pressure > self.max {
                    println!("Found {},{:?}",pressure,self.path);
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
                    self.combinations(net, tmp.as_slice());
                });

            self.path.pop();
        }
    }

    let input = std::fs::read_to_string("src/bin/day16_input.txt").expect("ops!");
    let net = ValveNet::parse(INPUT);
    let start = "AA";
    let valves = net.flow.iter()
        .filter(|(_, v)| v.pressure > 0 )
        .fold( vec![start],|mut out, (name, _)| {
            out.push(name);
            out
        });
    println!("Valves: {:?}", valves );
    let (max,solution) = greedy_search(&net, BUDGET, start);
    println!("Pressure (Greedy): {}", max );

    // create all valve visit order combinations
    let mut comb = ValveBacktrack { path: vec![], solution: vec![], max };
    comb.combinations(&net,&solution);

    println!("Solutions: {:?}\nMax flow {:?}", comb.solution, comb.max)
}

fn path_pressure(net:&ValveNet, mut time_left: usize, combinations: &[&str]) -> usize {

    combinations
        .windows(2)
        .map_while(|valves| {
            let target = valves[1];
            let path = net.find_path(valves[0], &target);
            if time_left < path.len() {
                None
            } else {
                time_left -= path.len(); // = len-1 steps + open valve
                let total_pressure_released = net.flow[&target].pressure * time_left;
                Some(total_pressure_released)
            }
        })
        .sum::<usize>()
}

fn greedy_search<'a>(net:&'a ValveNet, mut time_left:usize, start: &'a str) -> (usize,Vec<&'a str>) {
    let mut queue = VecDeque::new();
    // let mut combination = vec!["CC", "EE", "HH", "JJ", "BB", "DD", "AA"];
    let mut flow = net.flow.iter()
        .map(|(key,valve)| (key, valve.clone()))
        .collect::<HashMap<_,_>>();
    let mut path = vec![start];

    queue.push_back(start);

    let mut pressure = 0;

    while let Some(valve) = queue.pop_front() {

        flow.get_mut(&valve).unwrap().open = true;

        let mut options = flow.iter()
            .filter(|&(_,valve)| valve.pressure > 0  && !valve.open )
            .map(|(target,_)|
                net.find_path(&valve, &target)
            )
            .map(|path|
                (path[0], path.len(), net.flow[&path[0]].pressure/(path.len()))
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
                path.extend(options.iter().map(|&(v,..)| v));
                return (pressure,path)
            }
            time_left -= cost;
            pressure += net.flow[&valve].pressure * time_left;
            println!("====> Time: {time_left} got for Option {:?} out of Options: {:?}", (&valve, cost, value, time_left, pressure), options);
            queue.push_back(valve);
        }
    }
    (pressure,path)
}

#[derive(Debug)]
struct BFS();
impl BFS {
}

#[derive(Debug, Copy, Clone)]
struct Valve {
    pressure: usize,
    open: bool
}

struct ValveNet<'a> {
    graph: HashMap<&'a str,Vec<&'a str>>,
    flow: HashMap<&'a str, Valve>,
    cache: HashMap<(&'a str,&'a str),usize>
}

impl ValveNet<'_> {
    fn find_path<'a>(&'a self, start:&'a str, end:&'a str) -> Vec<&'a str> {
        let mut queue = VecDeque::new();
        let mut state: HashMap<&str,(bool,Option<&str>)> =
            self.flow.iter()
                .map(|(&key,_)| (key, (false, None)))
                .collect::<HashMap<_,_>>();
        let mut path = vec![];

        queue.push_back(start);
        while let Some(valve) = queue.pop_front() {

            if valve.eq(end) {
                path.push(valve);
                let mut cur = valve;
                while let Some(par) = state[&cur].1 {
                    path.push(par);
                    cur = par;
                }
                break
            }
            state.get_mut(valve).unwrap().0 = true;
            for &v in &self.graph[valve] {
                if !state[v].0 {
                    state.get_mut(v).unwrap().1 = Some(valve);
                    queue.push_back(v)
                }
            }
        }
        path
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

        ValveNet { graph, flow, cache: HashMap::new() }
    }
}