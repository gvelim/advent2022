use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
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
    let mut volcano = Volcano::parse(INPUT);
    println!("Graph: {:?}",volcano.graph);
    println!("FlowMap: {:?}",volcano.flow);


    let mut queue = VecDeque::new();
    queue.push_back("AA".to_string());

    while let Some(valve) = queue.pop_front() {

        volcano.flow.get_mut(&valve).unwrap().open = true;

        let mut options = volcano.flow.iter()
            .filter(|&(_,valve)| valve.pressure > 0  && !valve.open )
            .map(|(target,v)|
                (BFS::find_path(&volcano, &valve, target),v)
            )
            .map(|(path, valve)|
                (path[0].clone(), path.len(), valve.pressure/path.len())
            )
            .inspect(|(target, time, value)|
                println!("\tPressure:{}, Cost:{} Value: {:?} = {:?}", volcano.flow[target].pressure, time, value, target)
            )
            .collect::<Vec<_>>();

        options.sort_by(|a,b|
            match a.2.cmp(&b.2) {
                res@
                (Ordering::Less | Ordering::Greater) => res,
                Ordering::Equal => b.1.cmp(&a.1)
            }
        );

        if let Some(option) = options.pop() {
            println!("====> Option {:?} out of Options: {:?}",option,options);
            queue.push_back(option.0);
        }
    }
}

#[derive(Debug)]
struct BFS();
impl BFS {
    fn find_path(vol:&Volcano, start:&String, end:&String) -> Vec<String> {
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

#[derive(Debug)]
struct Valve {
    name: String,
    pressure: usize,
    open: bool
}
struct Volcano {
    graph: HashMap<String,Vec<String>>,
    flow: HashMap<String, Valve>
}

impl Volcano {
    fn parse(input: &str) -> Volcano {
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
                        name: key.to_string(),
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

        Volcano { graph, flow }
    }
}