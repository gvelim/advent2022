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

        volcano.flow.get_mut(&valve).unwrap().1 = true;

        let mut options = volcano.flow.iter()
            .filter(|&(_,&(bar,open))| bar > 0  && !open )
            .filter_map(|(target,data)|
                Some((BFS::find_path(&volcano, &valve, target),data))
            )
            .inspect(|(path,_)| print!("Path: {:?}",path) )
            .fold(vec![],|mut out, (path,&(flow,_))|{
                let value = flow / (path.len()+1);
                println!(" = Pressure:{}, Cost:{} Value: {:?}",flow,path.len()+1,value);
                out.push((path[0].clone(),value));
                out
            });
        options.sort_by_key(|a| a.1 );
        if let Some(option) = options.pop() {
            println!("====> Options {:?} ==> Option: {:?}",options,option);
            queue.push_back(option.0.clone());
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

struct Volcano {
    graph: HashMap<String,Vec<String>>,
    flow: HashMap<String, (usize,bool)>
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
                    (usize::from_str(flow).expect("Cannot convert flow"),false)
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