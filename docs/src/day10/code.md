# Day 10: Code

Below is the complete code for Day 10's solution, which simulates a CPU and CRT display.

## Full Solution

```advent2022/src/bin/day10.rs#L1-121
use std::str::FromStr;
use std::vec::IntoIter;

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
    exec_cycles: Cycles,
    ip: Option<IntoIter<Instruction>>
}
impl CPU {
    fn new() -> CPU {
        CPU { x: Register(1), buffer: None, exec_cycles: 0, ip: None }
    }
    fn load(&mut self, ops: Vec<Instruction>) {
        self.ip = Some(ops.into_iter());
    }
    fn fetch(&mut self, op: Instruction) {
        self.exec_cycles = op.ticks;
        self.buffer = Some(op);
    }
    fn execute(&mut self) -> bool {
        match self.buffer {                         // Check instruction buffer
            None => false,                          // empty, not exec, go and load
            Some(op) => {                 // Instruction loaded
                self.exec_cycles -= 1;               // execution cycle #
                if self.exec_cycles == 0 {           // exec cycles reached ?
                    self.x.0 += op.result();            // move Val to Reg X
                    self.buffer = None;                 // flush instruction buffer
                    false                           // not exec, go and load
                } else { true }                     // Busy executing
            }
        }
    }
    fn tick(&mut self) {
        if !self.execute() {
            let mut ip = self.ip.take().unwrap();
            self.fetch(ip.next().unwrap());
            self.ip.replace(ip);
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
        .map(|line| {
            let mut iter = line.split(' ');
            match iter.next() {
                Some("noop") => Instruction { op: InstructionSet::Noop, ticks: 1 },
                Some("addx") => {
                    let val = isize::from_str(
                        iter.next().expect("parse_instructions: addx is missing its value!")
                    ).expect("parse_instructions: addx not followed by numeric value!");
                    Instruction { op: InstructionSet::AddX(val), ticks: 2 }
                },
                _ => panic!("parse_instructions: unknown instruction caught!")
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

    let sample_intervals = vec![20usize, 60, 100, 140, 180, 220];
    let mut sampling_interval = sample_intervals.iter().peekable();

    let mut crt = CRT::new(40);
    let mut cpu = CPU::new();

    let (opcode, clock) = parse_instructions(input.as_str() );
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
```

## Code Walkthrough

### Data Types and Instruction Set

```advent2022/src/bin/day10.rs#L4-21
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
```

The code defines the core types for the CPU simulation:

- `Cycles` is a type alias for `usize` to represent clock cycles
- `InstructionSet` is an enum of the possible instructions (`Noop` and `AddX`)
- `Instruction` combines an operation with the number of cycles it takes
- `Register` is a simple wrapper around an `isize` value

The `result` method on `Instruction` returns the value that should be added to the X register after execution.

### CPU Implementation

```advent2022/src/bin/day10.rs#L24-64
struct CPU {
    x: Register,
    buffer: Option<Instruction>,
    exec_cycles: Cycles,
    ip: Option<IntoIter<Instruction>>
}
impl CPU {
    fn new() -> CPU {
        CPU { x: Register(1), buffer: None, exec_cycles: 0, ip: None }
    }
    fn load(&mut self, ops: Vec<Instruction>) {
        self.ip = Some(ops.into_iter());
    }
    fn fetch(&mut self, op: Instruction) {
        self.exec_cycles = op.ticks;
        self.buffer = Some(op);
    }
    fn execute(&mut self) -> bool {
        match self.buffer {                         // Check instruction buffer
            None => false,                          // empty, not exec, go and load
            Some(op) => {                 // Instruction loaded
                self.exec_cycles -= 1;               // execution cycle #
                if self.exec_cycles == 0 {           // exec cycles reached ?
                    self.x.0 += op.result();            // move Val to Reg X
                    self.buffer = None;                 // flush instruction buffer
                    false                           // not exec, go and load
                } else { true }                     // Busy executing
            }
        }
    }
    fn tick(&mut self) {
        if !self.execute() {
            let mut ip = self.ip.take().unwrap();
            self.fetch(ip.next().unwrap());
            self.ip.replace(ip);
        }
    }
    fn reg_x(&self) -> isize {
        self.x.0
    }
}
```

The `CPU` struct models a simple processor with:

- An X register storing a single value
- An instruction buffer for the currently executing instruction
- A counter for the remaining execution cycles
- An instruction pointer to iterate through the program

The key methods are:

- `execute()` - Processes one cycle of the current instruction, decrements the cycle counter, and returns whether execution is still in progress
- `tick()` - Advances the CPU by one cycle, either continuing execution or fetching a new instruction
- `reg_x()` - Returns the current value of the X register

### CRT Implementation

```advent2022/src/bin/day10.rs#L66-82
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
```

The `CRT` struct implements a simple display:

- `width` defines how many pixels are in each row
- `clock` tracks the current pixel position
- `draw()` prints a pixel based on whether the sprite (positioned at `pos`) overlaps with the current pixel
- `tick()` advances the CRT clock after drawing a pixel

### Instruction Parsing

```advent2022/src/bin/day10.rs#L84-106
fn parse_instructions(inp: &str) -> (Vec<Instruction>, usize) {
    inp.lines()
        .map(|line| {
            let mut iter = line.split(' ');
            match iter.next() {
                Some("noop") => Instruction { op: InstructionSet::Noop, ticks: 1 },
                Some("addx") => {
                    let val = isize::from_str(
                        iter.next().expect("parse_instructions: addx is missing its value!")
                    ).expect("parse_instructions: addx not followed by numeric value!");
                    Instruction { op: InstructionSet::AddX(val), ticks: 2 }
                },
                _ => panic!("parse_instructions: unknown instruction caught!")
            }
        })
        .fold((vec![],0), |(mut out,mut total), op| {
            total += op.ticks;
            out.push(op);
            (out,total)
        })
}
```

The `parse_instructions` function converts the input text to a list of instructions:

1. It splits each line and matches the instruction type
2. For `noop`, it creates an instruction with 1 execution cycle
3. For `addx`, it parses the value and creates an instruction with 2 execution cycles
4. It uses `fold` to build a vector of instructions while also calculating the total number of cycles

### Main Function

```advent2022/src/bin/day10.rs#L108-124
fn main() {
    let input = std::fs::read_to_string("src/bin/day10_input.txt").expect("Ops!");

    let sample_intervals = vec![20usize, 60, 100, 140, 180, 220];
    let mut sampling_interval = sample_intervals.iter().peekable();

    let mut crt = CRT::new(40);
    let mut cpu = CPU::new();

    let (opcode, clock) = parse_instructions(input.as_str() );
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
```

The main function ties everything together:

1. It defines the specific cycles at which to sample the signal (20, 60, 100, etc.)
2. It initializes the CRT and CPU
3. It parses the instructions and loads them into the CPU
4. It creates a range for all cycles and maps each cycle to:
   - Advance the CPU
   - Update the CRT
   - Return the cycle number and register value
5. It filters for the specific cycles we want to sample
6. It calculates the signal strength (cycle number × register value) for each sampled cycle
7. It sums all signal strengths and prints the result

The Part 2 output (the eight capital letters) is printed directly by the CRT during simulation.

## Implementation Notes

- **State Machine Design**: The CPU is implemented as a state machine that processes instructions cycle-by-cycle
- **Separation of Concerns**: The CPU and CRT are separate components with their own state and behavior
- **Pipeline Simulation**: The instruction execution follows a simple pipeline pattern with fetch and execute stages
- **Functional Programming**: The code uses functional programming patterns like `map`, `filter`, and `fold` for concise data processing