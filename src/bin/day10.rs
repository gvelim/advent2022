use std::str::FromStr;

type Cycles = usize;
#[derive(Debug,Copy, Clone)]
enum InstructionSet { Noop, AddX(isize) }

#[derive(Debug,Copy, Clone)]
struct Instruction {
    op: InstructionSet,
    ticks: Cycles
}
impl Instruction {
    fn result(&self) -> isize {
        match self.op {
            InstructionSet::Noop => 0,
            InstructionSet::AddX(val) => val
        }
    }
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
        CPU { x: Register(1), ir: None, count: 0 }
    }
    fn fetch(&mut self, op: Instruction) {
        self.count = 0;
        self.ir = Some(op);
    }
    fn execute(&mut self) -> bool {
        match self.ir {                         // Check instruction buffer
            None => false,                          // empty, not exec, go and load
            Some(op) => {                 // Instruction loaded
                self.count += 1;                // execution cycle #
                if op.ticks == self.count {     // exec cycles reached ?
                    self.x.0 += op.result();        // move Val to Reg X
                    self.ir = None;                 // flush instruction buffer
                    false                           // not exec, go and load
                } else { true }                 // Busy executing
            }
        }
    }
}

struct CRT {
    width: usize,
    clock: Cycles
}
impl CRT {
    fn new(width: usize) -> CRT {
        CRT{ width, clock: 0 }
    }
    fn draw(&mut self, pos: isize) {
        let col = self.clock % self.width;
        if (pos-1..=pos+1).contains(&(col as isize)) {
            print!("#")
        } else {
            print!(".")
        }
        if col == self.width-1 { println!() }
        self.clock += 1;
    }
}

fn parse_instructions(inp: &str) -> (Vec<Instruction>, usize) {
    inp.lines()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|item| {
            match item[0] {
                "noop" => Instruction { op: InstructionSet::Noop, ticks: 1 },
                "addx" => {
                    let val = isize::from_str(item[1]).unwrap();
                    Instruction { op: InstructionSet::AddX(val), ticks: 2 }
                },
                _ => panic!("Woohaa!")
            }
        })
        .fold((vec![],0), |(mut out,mut total), op| {
            total += op.ticks;
            out.push(op);
            (out,total)
        })
}

fn main() {
    let input = std::fs::read_to_string("src/bin/day10_input.txt").expect("Ops!");

    let (opcode, clock) = parse_instructions(input.as_str() );
    let mut ip = opcode.into_iter();

    let samples = vec![20usize, 60, 100, 140, 180, 220];
    let mut sampling = samples.iter().peekable();

    let mut cpu = CPU::new();
    let mut crt = CRT::new(40);

    let sum = (1..=clock)
        .map(|cycle| {
            if !cpu.execute() {
                cpu.fetch( ip.next().unwrap());
            }
            crt.draw(cpu.x.0);
            (cycle,cpu.x.0)
        })
        .filter(|(cycle,_)| {
            if let Some(s) = sampling.peek() {
                if cycle.eq(s) {
                    sampling.next();
                    true
                } else { false }
            } else { false }
        })
        .map(|(clock, x)| x * clock as isize)
        .sum::<isize>();

    println!("{sum} is the sum of  signal strengths at {:?}", samples);
}