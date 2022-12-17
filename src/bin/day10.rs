use std::str::FromStr;

type Cycles = usize;
#[derive(Debug,Copy, Clone)]
enum InstructionSet { noop, addx(isize) }

#[derive(Debug,Copy, Clone)]
struct Instruction {
    op: InstructionSet,
    ticks: Cycles
}

#[derive(Debug)]
struct Register(isize);

#[derive(Debug)]
struct CPU {
    x: Register,
    ir: Option<Instruction>,
    count: Cycles
}
impl CPU {
    fn new() -> CPU {
        CPU { x: Register(0), ir: None, count: 0 }
    }
    fn fetch(&mut self, op: Instruction) {
        self.count = 0;
        self.ir = Some(op);
    }
    fn execute(&mut self) -> bool {
        if let Some(op) = self.ir {
            self.count += 1;
            if op.ticks == self.count {
                match op.op {
                    InstructionSet::noop => {},
                    InstructionSet::addx(val) => self.x.0 += val
                }
                self.ir = None;
                false // no executing
            } else { true } // in execution
        } else {
            false // not executing
        }
    }
}

fn parse_instructions(inp: &str) -> Vec<Instruction> {
    inp.lines()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|mut item| {
            match item[0] {
                "noop" => Instruction{ op:InstructionSet::noop, ticks: 1 },
                "addx" => {
                    let val = isize::from_str(item[1]).unwrap();
                    Instruction{ op:InstructionSet::addx(val), ticks: 2 }
                },
                _ => panic!("Woohaa!")
            }
        })
        .fold(vec![], |mut out, op| {
            out.push(op);
            out
        })
}

fn main() {

    let input = std::fs::read_to_string("src/bin/day10_input.txt").expect("Ops!");
    // let sw = parse_instructions("noop\naddx 3\naddx -5" );
    let sw = parse_instructions(input.as_str() );

    let mut cpu = CPU::new();
    let clock = sw.iter().map(|e| e.ticks).sum();
    let mut ip = sw.into_iter();
    (1..=clock)
        .map(|cycle| {
            print!("{cycle} - ");
            if !cpu.execute() {
                print!("{:?}", cpu);
                cpu.fetch( ip.next().unwrap());
            }
            (cycle,cpu.x.0)
        })
        .inspect(|p| println!("{:?}",p))
        .all(|_| true);
}