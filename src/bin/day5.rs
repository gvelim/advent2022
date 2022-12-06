use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug,Copy,Clone)]
struct Move {
    count: usize,
    from: usize,
    to: usize
}
impl FromStr for Move {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [_,count,_,from,_,to] = s.split(' ').collect::<Vec<_>>()[..] {
            Ok(
                Move {
                    count: usize::from_str(count)?,
                    from: usize::from_str(from)?,
                    to: usize::from_str(to)?,
                }
            )
        } else {
            unreachable!()
        }
    }
}
impl Move {
    fn parse_moves(moves:&str) -> Vec<Move> {
        moves.lines()
            .map(|line| Move::from_str(line).unwrap_or_else(|e| panic!("{e}")) )
            .collect()
    }
}
#[derive(Debug)]
struct Buckets {
    buckets: HashMap<usize,Vec<char>>,
    keys: Vec<usize>
}
impl Buckets {
    fn new(start: &str) -> Buckets {
        let buckets = start.lines()
            .rev()
            .map(|line| line.split("").filter_map(|e| e.chars().next()).collect::<Vec<_>>())
            .fold(HashMap::new(), |map, e| {
                e.into_iter()
                    .enumerate()
                    .filter(|(_, c)| c.is_alphanumeric())
                    .fold(map, |mut out, (key, val)| {
                        out.entry(key)
                            .or_insert(Vec::new())
                            .push(val);
                        out
                    })
            });
        let mut keys = buckets.keys().copied().collect::<Vec<_>>();
        keys.sort();
        Buckets {
            buckets,
            keys
        }
    }
    fn crate_mover9000(&mut self, m: Move) {
        let (from, to) = self.get_keys(m);
        (0..m.count)
            .for_each(|_|{
                if let Some(c) = self.buckets.get_mut(&from).expect("").pop() {
                    self.buckets.get_mut(&to).expect("").push(c)
                }
        });
    }
    fn crate_mover9001(&mut self, m: Move) {
        let (from, to) = self.get_keys(m);
        let v = (0..m.count)
            .fold(vec![],|mut out,_|{
                if let Some(c) = self.buckets.get_mut(&from).expect("").pop() { out.push(c) }
                out
            });
        self.buckets.get_mut(&to).expect("").extend(v.iter().rev());
    }
    fn scoop_top(&self) -> String {
        self.keys.iter()
            .filter_map(|key| self.buckets.get(key))
            .filter_map(|arr| arr.last().copied() )
            .fold(String::new(),|mut out,s| { out.push(s); out })
    }
    fn get_keys(&self, m:Move) -> (usize,usize) {
        (self.keys[m.from-1],self.keys[m.to-1])
    }
}

fn main() {

    let data = std::fs::read_to_string("src/bin/day5_input.txt").expect("Ops!");

    let [start,moves] = data.split("\n\n").collect::<Vec<_>>()[..] else { panic!("") };

    let mut buckets = Buckets::new(start);
    let moves = Move::parse_moves(moves);

    moves.iter().for_each(|&m| buckets.crate_mover9000(m) );
    println!("{:?}",buckets.scoop_top());

    moves.iter().for_each(|&m| buckets.crate_mover9001(m) );
    println!("{:?}",buckets.scoop_top());

}
