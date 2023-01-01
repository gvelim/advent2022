use std::collections::{HashMap};
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
    let volcano = Volcano::parse(INPUT);
    println!("Graph: {:?}",volcano.graph);
    println!("FlowMap: {:?}",volcano.flow);

    let mut dfs = DFS::new(&volcano);
    println!("DFS: {:?}",dfs);
    let path = dfs.find_path(&volcano,"AA");
    println!("DFS: {:?}",path);
    let sum = dfs.path.into_iter()
        .reduce(|a,b| (b.0, a.1+b.1)  ).unwrap();
    println!("DFS: {:?}",sum);
}

#[derive(Debug)]
struct DFS {
    visited: HashMap<String,bool>,
    path: Vec<(String,usize)>
}
impl DFS {
    fn new(volcano: &Volcano) -> DFS {
        DFS {
            visited: volcano.flow.iter().map(|(key,_)| (key.clone(),false)).collect(),
            path: vec![]
        }
    }
    fn find_path(&mut self, vol:&Volcano, start:&str) -> &Vec<(String,usize)> {
        let s = start.to_string();
        *self.visited.get_mut(&s).unwrap() = true;

        if let Some(pipes) = vol.graph.get(s.as_str()) {
            for pipe in pipes {
                if !self.visited[pipe] {
                    self.find_path(vol,pipe);
                }
            }
        }
        let flow = vol.flow[&s];
        self.path.push((s,flow));
        &self.path
    }
}

struct Volcano {
    graph: HashMap<String,Vec<String>>,
    flow: HashMap<String, usize>
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
                f.entry(key.to_string()).or_insert(usize::from_str(flow).expect("Cannot convert flow"));
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