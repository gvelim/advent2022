use std::cell::Cell;
use std::collections::VecDeque;
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

    input.split("\n\n")
        .into_iter()
        .map(|monkey| Monkey::from_str(monkey).unwrap() )
        .inspect(|m| println!("{:?}",m))
        .all(|_| true);

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
            .map(|vline|{
                let m = monkey.get_mut();
                match vline[0] {
                    "Starting items" => {
                        vline[1].split(',')
                            .into_iter()
                            .map(|n| usize::from_str(n.trim()).unwrap() )
                            .all(|a| { m.items.push_back(a); true });
                    }
                    "Operation" => {
                        let [op,act] = vline[1]
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
                        let s = vline[1].trim().split("divisible by").last().unwrap().trim();
                        m.test = usize::from_str(s).unwrap();
                    }
                    "If true" => {
                        let s = vline[1].trim().split("throw to monkey").last().unwrap().trim();
                        m.send.0 = usize::from_str(s).unwrap();
                    }
                    "If false" => {
                        let s = vline[1].trim().split("throw to monkey").last().unwrap().trim();
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