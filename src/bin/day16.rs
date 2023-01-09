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


fn main() {

    struct Combinations<'a> {
        path: Vec<&'a str>,
        solution: Vec<&'a str>,
        max: usize
    }
    impl<'a> Combinations<'a> {
        fn combinations(&mut self, net: &ValveNet, valves: &[&'a str]) {

            self.path.push(valves[0]);

            if valves.len() == 1 {
                // ok we got potential solution, store it
                let pressure = path_pressure(&net, &self.path);
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
    let max = greedy_search(&net, "AA");
    println!("Pressure (Greedy): {}", max );

    // create all valve visit order combinations
    let mut comb = Combinations{ path: vec![], solution: vec![], max };
    comb.combinations(&net,&valves);

    println!("Solutions: {:?}\nMax flow {:?}", comb.solution, comb.max)
}

fn path_pressure(volcano:&ValveNet, combinations: &[&str]) -> usize {

    let mut time_left = 30;

    combinations
        .windows(2)
        .map_while(|valves| {
            let target = valves[1];
            let path = BFS::find_path(&volcano, valves[0], &target);
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

fn greedy_search(net:&ValveNet, start: &str) -> usize {
    let mut queue = VecDeque::new();
    // let mut combination = vec!["CC", "EE", "HH", "JJ", "BB", "DD", "AA"];
    let mut flow = net.flow.iter()
        .map(|(key,valve)| (key.clone(), valve.clone()))
        .collect::<HashMap<_,_>>();

    queue.push_back(start);

    let mut budget = 30;
    let mut pressure = 0;

    while let Some(valve) = queue.pop_front() {

        flow.get_mut(&valve).unwrap().open = true;

        let mut options = flow.iter()
            .filter(|&(_,valve)| valve.pressure > 0  && !valve.open )
            .map(|(target,_)|
                BFS::find_path(&net, &valve, &target)
            )
            // .inspect(|path| print!("\t {:?}",path))
            .map(|path|
                (path[0].clone(), path.len(), net.flow[&path[0]].pressure/(path.len()))
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
            if budget < cost { return pressure }
            budget -= cost;
            pressure += net.flow[&valve].pressure * budget;
            println!("====> Time: {budget} got for Option {:?} out of Options: {:?}",(&valve,cost,value,budget,pressure),options);
            queue.push_back(valve);
        }
    }
    pressure
}

#[derive(Debug)]
struct BFS();
impl BFS {
    fn find_path<'a>(vol:&'a ValveNet, start:&'a str, end:&'a str) -> Vec<&'a str> {
        let mut queue = VecDeque::new();
        let mut state: HashMap<&str,(bool,Option<&str>)> =
            vol.flow.iter()
                .map(|(&key,_)| (key, (false, None)))
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

struct ValveNet<'a> {
    graph: HashMap<&'a str,Vec<&'a str>>,
    flow: HashMap<&'a str, Valve>
}

impl ValveNet<'_> {
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

        ValveNet { graph, flow }
    }
}