extern crate core;

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


fn main() {

    struct Combinations<'a> {
        path: Vec<&'a str>,
        combos: Vec<Vec<&'a str>>
    }
    impl<'a> Combinations<'a> {
        fn combinations(&mut self, valves: &[&'a str]) -> &Vec<&str> {

            self.path.push(valves[0]);

            if valves.len() == 1 {
                // ok we got potential solution, store it
                self.combos.push(self.path.clone());
                self.path.pop();
                return &self.path;
            }

            let mut tmp = valves[1..].to_vec();
            (0..valves[1..].len())
                .for_each(|_|{
                    self.combinations(tmp.as_slice());
                    tmp.rotate_right(1);
                });

            self.path.pop();
            &self.path
        }
    }

    let net = ValveNet::parse(INPUT);
    let valves = ["AA", "DD", "BB", "JJ", "HH", "EE", "CC"];
    println!("Pressure: {}", greedy_search(&net, "AA") );

    // create all valve visit order combinations
    let mut comb = Combinations{ path: vec![], combos: vec![] };
    comb.combinations(&valves);

    // per path calculate total pressure released and select the max of all paths
    let max = comb.combos.iter()
        .map(|path|
            (path_pressure(&net, path.as_slice()), path)
        )
        .max_by(|a,b| a.0.cmp(&b.0) );
    println!("Solutions: {}\nMax flow {:?}",comb.combos.len(),max)
}

fn path_pressure(volcano:&ValveNet, combinations: &[&str]) -> usize {

    let mut time_left = 30;

    combinations
        .windows(2)
        .map_while(|valves| {
            let target = valves[1].to_string();
            let path = BFS::find_path(&volcano, &valves[0].to_string(), &target);
            if time_left < path.len() {
                None
            } else {
                time_left -= path.len(); // = len-1 steps + open valve
                let total_pressure_released = volcano.flow[&target].pressure * time_left;
                // println!("====> Time {time_left}, {:?}", (&target, path.len(), total_pressure_released));
                Some(total_pressure_released)
            }
        })
        .sum::<usize>()
}

fn greedy_search(volcano:&ValveNet, start: &str) -> usize {
    let mut queue = VecDeque::new();
    // let mut combination = vec!["CC", "EE", "HH", "JJ", "BB", "DD", "AA"];
    let mut flow = volcano.flow.iter()
        .map(|(key,valve)| (key.clone(), valve.clone()))
        .collect::<HashMap<_,_>>();

    queue.push_back(start.to_string());

    let mut budget = 30;
    let mut pressure = 0;

    while let Some(valve) = queue.pop_front() {

        flow.get_mut(&valve).unwrap().open = true;

        let mut options = flow.iter()
            .filter(|&(_,valve)| valve.pressure > 0  && !valve.open )
            .map(|(target,_)|
                BFS::find_path(&volcano, &valve, &target)
            )
            // .inspect(|path| print!("\t {:?}",path))
            .map(|path|
                (path[0].clone(), path.len(), volcano.flow[&path[0]].pressure/(path.len()))
            )
            // .inspect(|(target, time, value)|
            //     println!("\tPressure:{}, Cost:{} Value: {:?} = {:?}", volcano.flow[target].pressure, time, value, target)
            // )
            .collect::<Vec<_>>();

        options.sort_by(|a,b|
            match a.2.cmp(&b.2) {
                res@
                (Ordering::Less | Ordering::Greater) => res,
                Ordering::Equal => b.1.cmp(&a.1)
            }
        );

        // let Some(next) = combination.pop() else { continue };
        // if let Some(&(ref valve,cost,value)) = options.iter().find(|(s,_,_)| s.eq(next) ) {

        if let Some((valve,cost,value)) = options.pop() {
            budget -= cost;
            pressure += volcano.flow[&valve].pressure * budget;
            println!("====> Time: {budget} got for Option {:?} out of Options: {:?}",(&valve,cost,value,budget,pressure),options);
            queue.push_back(valve);
        }
    }
    pressure
}

#[derive(Debug)]
struct BFS();
impl BFS {
    fn find_path(vol:&ValveNet, start:&String, end:&String) -> Vec<String> {
        let mut queue = VecDeque::new();
        let mut state: HashMap<String,(bool,Option<String>)> =
            vol.flow.iter()
                .map(|(key,_)| (key.clone(), (false, None)))
                .collect::<HashMap<_,_>>();
        let mut path = vec![];

        queue.push_back(start);
        while let Some(valve) = queue.pop_front() {

            if valve.eq(end) {
                path.push(valve.clone());
                let mut cur = valve.clone();
                while let Some(par) = state[&cur].1.clone() {
                    path.push(par.clone());
                    cur = par;
                }
                break
            }
            state.get_mut(valve).unwrap().0 = true;
            for v in &vol.graph[valve] {
                if !state[v].0 {
                    state.get_mut(v).unwrap().1 = Some(valve.clone());
                    queue.push_back(v)
                }
            }
        }
        path
    }
}

#[derive(Debug, Copy, Clone)]
struct Valve {
    pressure: usize,
    open: bool
}

struct ValveNet {
    graph: HashMap<String,Vec<String>>,
    flow: HashMap<String, Valve>
}

impl ValveNet {
    fn parse(input: &str) -> ValveNet {
        let (graph, flow) = input.lines()
            .map(|line| {
                line.split(&[' ','=',';',','])
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
            })
            .map(|s| (s[1],s[5],s[10..].to_vec()))
            .fold( (HashMap::new(),HashMap::new()),|(mut g, mut f),(key, flow, edges)| {
                f.entry(key.to_string()).or_insert(
                    Valve {
                        pressure: usize::from_str(flow).expect("Cannot convert flow"),
                        open: false
                    }
                );
                edges.into_iter()
                    .for_each(|edge|
                        g.entry(key.to_string())
                            .or_insert(Vec::new())
                            .push(edge.to_string())
                    );
                (g,f)
            });

        ValveNet { graph, flow }
    }
}