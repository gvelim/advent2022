use std::cell::RefCell;
use std::rc::Rc;
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


struct CPU {
    x: Register,
    buffer: Option<Instruction>,
    exec_cycle: Cycles,
    ip: Option<Rc<RefCell<dyn Iterator<Item=Instruction>>>>
}
impl CPU {
    fn new() -> CPU {
        CPU { x: Register(1), buffer: None, exec_cycle: 0, ip: None }
    }
    fn load(&mut self, ops: Vec<Instruction>) {
        self.ip = Some(Rc::new(RefCell::new(ops.into_iter())));
    }
    fn fetch(&mut self, op: Instruction) {
        self.exec_cycle = 0;
        self.buffer = Some(op);
    }
    fn execute(&mut self) -> bool {
        match self.buffer {                         // Check instruction buffer
            None => false,                          // empty, not exec, go and load
            Some(op) => {                 // Instruction loaded
                self.exec_cycle += 1;               // execution cycle #
                if op.ticks == self.exec_cycle {    // exec cycles reached ?
                    self.x.0 += op.result();            // move Val to Reg X
                    self.buffer = None;                 // flush instruction buffer
                    false                           // not exec, go and load
                } else { true }                     // Busy executing
            }
        }
    }
    fn tick(&mut self) {
        let Some(tmp) = self.ip.clone() else { panic!("")};
        let mut iter = tmp.borrow_mut();
        if !self.execute() {
            self.fetch(iter.next().unwrap());
        }
    }
    fn reg_x(&self) -> isize {
        self.x.0
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
        print!("{}",
            if (pos-1..=pos+1).contains(&(col as isize)) { '#' } else { '.' }
        );
        if col == self.width-1 { println!() }
    }
    fn tick(&mut self, pos:isize) {
        self.draw(pos);
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
    // let mut ip = opcode.into_iter();

    let sample_intervals = vec![20usize, 60, 100, 140, 180, 220];
    let mut sampling_interval = sample_intervals.iter().peekable();

    let mut crt = CRT::new(40);
    let mut cpu = CPU::new();
    cpu.load(opcode);


    let sum = (1..=clock)
        .map(|cycle| {
            cpu.tick();
            crt.tick(cpu.reg_x());
            ( cycle, cpu.reg_x() )
        })
        .filter(|(cycle,_)|
            match sampling_interval.peek() {
                Some(&to_sample) if to_sample.eq(cycle) => { sampling_interval.next(); true }
                _ => false
            }
        )
        .map(|(clock, x)| x * clock as isize)
        .sum::<isize>();

    println!("{sum} is the sum of  signal strengths at {:?}", sample_intervals);
}