use std::cell::Cell;
use std::collections::VecDeque;
use std::ops::{Add, Mul};
use std::str::FromStr;

fn main() {

    let input = std::fs::read_to_string("src/bin/day11_input.txt").expect("Ops!");

    let mut monkeys = Monkey::parse_text(input.as_str());
    let div_product: WorryType = monkeys.iter().map(|m| m.test).product();

    // Queue for passing items around the monkeys
    let mut queue = vec![VecDeque::<WorryType>::new(); monkeys.len()];

    (0..10000).all(|_| {
        monkeys.iter_mut()
            .map(|monkey| {

                // pull from queue anything thrown at him
                while let Some(item) = queue[monkey.name].pop_front() {
                    monkey.catch(item)
                };

                // observe and throw back at
                monkey.observe_all(div_product)
                    .into_iter()
                    // .filter_map(|throw| throw)
                    .all(|throw|
                        throw.map(
                            |(monkey,item)| queue[monkey].push_back(item)
                        ).is_some()
                    )
            })
            .all(|run| run)
    });

    monkeys.sort_by(|a,b| b.inspect.cmp(&a.inspect));
    println!("level of monkey business after 10000 rounds : {:?}",
             monkeys[0].inspections() * monkeys[1].inspections()
    );
}


type WorryType = u64;
const WORRY_DEF: WorryType = 0;

#[derive(Debug)]
enum Operation {
    Add(WorryType),
    Mul(WorryType),
}
#[derive(Debug)]
struct Monkey {
    name: usize,
    items: VecDeque<WorryType>,
    op: Operation,
    test: WorryType,
    send: (usize,usize),
    inspect: usize
}
impl Monkey {
    fn parse_text(input: &str) -> Vec<Monkey> {
        input.split("\n\n")
            .map(|monkey| Monkey::from_str(monkey).unwrap() )
            .fold(Vec::new(), |mut out, monkey|{
                out.push(monkey);
                out
            })
    }
    fn catch(&mut self, item: WorryType) {
        self.items.push_back(item)
    }
    fn throw(&self, worry: WorryType) -> (usize, WorryType) {
        if (worry % self.test) == 0 as WorryType {
            // Current worry level is divisible by 23.
            // Sent to Monkey
            (self.send.0, worry)
        } else {
            // Current worry level is not divisible by 23.
            // Sent to Monkey
            (self.send.1, worry)
        }
    }
    fn observe(&mut self, div: WorryType) -> Option<(usize, WorryType)> {
        self.inspect += 1;
        //   Monkey inspects an item with a worry level of 79.
        match self.items.pop_front() {
            Some(mut worry) => {
                //     Worry level is multiplied by 19 to 1501.
                //     Monkey gets bored with item. Worry level is divided by 3 to 500.
                worry %= div;
                Some( self.throw(
                    match self.op {
                        Operation::Add(WORRY_DEF) => worry.add(worry),
                        Operation::Mul(WORRY_DEF) => worry.mul(worry),
                        Operation::Add(n) => worry + n,
                        Operation::Mul(n) => worry * n,
                    }
                ))
            }
            None => None
        }
    }
    fn observe_all(&mut self, div: WorryType) -> Vec<Option<(usize, WorryType)>> {
        (0..self.items.len())
            .fold(vec![], |mut out, _|{
                out.push( self.observe(div));
                out
            })
    }
    fn inspections(&self) -> usize {
        self.inspect
    }
}
impl Default for Monkey {
    fn default() -> Self {
        Monkey {
            name: 0,
            items: VecDeque::new(),
            op: Operation::Add(WORRY_DEF),
            test: WORRY_DEF,
            send: (0,0),
            inspect: 0
        }
    }
}
impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut monkey = Cell::new(Monkey::default());
        s.lines()
            .map(|line| line.trim().split(':').collect::<Vec<_>>())
            .map(|parts|{
                let m = monkey.get_mut();
                match parts[0] {
                    "Starting items" => {
                        parts[1].split(',')
                            .map(|n| WorryType::from_str(n.trim()).unwrap() )
                            .all(|a| { m.items.push_back(a); true });
                    }
                    "Operation" => {
                        let [op,act] = parts[1]
                            .split("new = old ")
                            .last()
                            .unwrap()
                            .split(' ')
                            .collect::<Vec<_>>()[..] else { panic!("Operation: cannot be extracted") };
                        let a = WorryType::from_str(act);
                        match (op,a) {
                            ("*",Ok(n)) => m.op = Operation::Mul(n),
                            ("+",Ok(n)) => m.op = Operation::Add(n),
                            ("*",_) => m.op = Operation::Mul(WORRY_DEF),
                            ("+",_) => m.op = Operation::Add(WORRY_DEF),
                            _ => {}
                        }
                    }
                    "Test" => {
                        let s = parts[1].trim().split("divisible by").last().unwrap().trim();
                        m.test = WorryType::from_str(s).unwrap();
                    }
                    "If true" => {
                        let s = parts[1].trim().split("throw to monkey").last().unwrap().trim();
                        m.send.0 = usize::from_str(s).unwrap();
                    }
                    "If false" => {
                        let s = parts[1].trim().split("throw to monkey").last().unwrap().trim();
                        m.send.1 = usize::from_str(s).unwrap();
                    }
                    name => {
                        m.name = usize::from_str(name.split(' ').last().unwrap().trim()).unwrap();
                    }
                }
                true
            })
            .all(|run| run);

        Ok(monkey.take())
    }
}