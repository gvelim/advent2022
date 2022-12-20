use std::cell::Cell;
use std::collections::VecDeque;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

fn main() {
    let input =
    "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3

Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
        If true: throw to monkey 2
        If false: throw to monkey 0

Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
        If true: throw to monkey 1
        If false: throw to monkey 3

Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
        If true: throw to monkey 0
        If false: throw to monkey 1";

    // let input = std::fs::read_to_string("src/bin/day11_input.txt").expect("Ops!");

    let mut monkeys = Monkey::parse_text(input);
    println!("{:?}",monkeys);
    let mut queue = vec![ VecDeque::<usize>::new(); monkeys.len()];
    (0..1).all(|_| {
        monkeys.iter_mut()
            // .inspect(|e| println!("From queue: {:?}",e))
            .map(|monkey|{
                // pull from queue
                while let Some(item) = queue[monkey.name].pop_front() {
                    println!("From queue: {:?}",item);
                    monkey.catch(item)
                }
                println!("Monkey: {:?}",monkey);
                // observe and throw
                let r = monkey.observe_all();
                r.into_iter()
                    .filter_map(|throw| throw)
                    .inspect(|e| println!("To queue: {:?}",e))
                    .all(|(monkey, item)| {
                        queue[monkey].push_back(item);
                        true
                    })
            })
            .all(|t| t)
    });
    println!("{:?}",monkeys);
}


#[derive(Debug)]
enum Operation {
    Add(usize),
    Sub(usize),
    Div(usize),
    Mul(usize),
}
#[derive(Debug)]
struct Monkey {
    name: usize,
    items: VecDeque<usize>,
    op: Operation,
    test: usize,
    send: (usize,usize)
}
impl Monkey {
    fn parse_text(input: &str) -> Vec<Monkey> {
        input.split("\n\n")
            .into_iter()
            .map(|monkey| Monkey::from_str(monkey).unwrap() )
            .fold(Vec::new(), |mut out, monkey|{
                out.push(monkey);
                out
            })
    }
    fn catch(&mut self, item: usize) {
        self.items.push_back(item)
    }
    fn throw(&self, worry:usize) -> Option<(usize,usize)> {
        if (worry % self.test) == 0 {
            // Current worry level is divisible by 23.
            // Sent to Monkey
            Some((self.send.0, worry))
        } else {
            // Current worry level is not divisible by 23.
            // Sent to Monkey
            Some((self.send.1, worry))
        }
    }
    fn observe(&mut self) -> Option<(usize, usize)> {
        //   Monkey inspects an item with a worry level of 79.
        match self.items.pop_front() {
            Some(worry) => {
                //     Worry level is multiplied by 19 to 1501.
                //     Monkey gets bored with item. Worry level is divided by 3 to 500.
                self.throw(
                    match self.op {
                        Operation::Add(0) => worry.add(worry),
                        Operation::Sub(0) => worry.sub(worry),
                        Operation::Div(0) => worry.div(worry),
                        Operation::Mul(0) => worry.mul(worry),
                        Operation::Add(n) => worry + n,
                        Operation::Sub(n) => worry - n,
                        Operation::Div(n) => worry / n,
                        Operation::Mul(n) => worry * n,
                    } / 3
                )
            }
            None => None
        }
    }
    fn observe_all(&mut self) -> Vec<Option<(usize,usize)>> {
        (0..self.items.len())
            .fold(vec![], |mut out, _|{
                out.push( self.observe());
                out
            })
    }
}
impl Default for Monkey {
    fn default() -> Self {
        Monkey {
            name: 0,
            items: VecDeque::new(),
            op: Operation::Add(0),
            test: 0,
            send: (0,0)
        }
    }
}
impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut monkey = Cell::new(Monkey::default());
        s.lines()
            .into_iter()
            .map(|line| line.trim().split(':').collect::<Vec<_>>())
            // .inspect(|e| println!("{:?}",e))
            .map(|parts|{
                let m = monkey.get_mut();
                match parts[0] {
                    "Starting items" => {
                        parts[1].split(',')
                            .into_iter()
                            .map(|n| usize::from_str(n.trim()).unwrap() )
                            .all(|a| { m.items.push_back(a); true });
                    }
                    "Operation" => {
                        let [op,act] = parts[1]
                            .split("new = old ")
                            .last()
                            .unwrap()
                            .split(' ')
                            .collect::<Vec<_>>()[..] else { panic!("Operation: cannot be extracted") };
                        let a = usize::from_str(act);
                        match (op,a) {
                            ("*",Ok(n)) => m.op = Operation::Mul(n),
                            ("+",Ok(n)) => m.op = Operation::Add(n),
                            ("/",Ok(n)) => m.op = Operation::Div(n),
                            ("-",Ok(n)) => m.op = Operation::Sub(n),
                            ("*",_) => m.op = Operation::Mul(0),
                            ("+",_) => m.op = Operation::Add(0),
                            ("/",_) => m.op = Operation::Div(0),
                            ("-",_) => m.op = Operation::Sub(0),
                            _ => {}
                        }
                    }
                    "Test" => {
                        let s = parts[1].trim().split("divisible by").last().unwrap().trim();
                        m.test = usize::from_str(s).unwrap();
                    }
                    "If true" => {
                        let s = parts[1].trim().split("throw to monkey").last().unwrap().trim();
                        m.send.0 = usize::from_str(s).unwrap();
                    }
                    "If false" => {
                        let s = parts[1].trim().split("throw to monkey").last().unwrap().trim();
                        m.send.1 = usize::from_str(s).unwrap();
                    }
                    monk => {
                        m.name = usize::from_str(monk.split(' ').last().unwrap().trim()).unwrap();
                    }
                }
            })
            .all(|_| true);

        Ok(monkey.take())
    }
}