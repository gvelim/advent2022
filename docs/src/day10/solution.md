# Day 10: Solution Explanation

## Approach

Day 10 involves simulating a simple CPU with a basic instruction set and a CRT display. The solution requires us to:

1. **Parse the instructions**: Read the input and convert it to a series of CPU instructions
2. **Simulate the CPU**: Execute instructions while tracking the X register value
3. **Monitor signal strength**: Calculate signal strength at specific cycles
4. **Render the CRT**: Draw pixels based on the X register value

The solution models the CPU, its execution cycle, and the CRT display as separate components that interact with each other.

## Implementation Details

### Instruction Set

We start by defining the instruction set and what each instruction does:

```rust
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
```

The `InstructionSet` enum represents the two possible instructions:
- `Noop`: Does nothing
- `AddX(isize)`: Adds the specified value to the X register

The `Instruction` struct combines an operation with the number of cycles it takes to execute. The `result` method returns the value that should be added to the X register after execution.

### CPU Simulation

The CPU is modeled as a state machine with several components:

```rust
#[derive(Debug)]
struct Register(isize);

struct CPU {
    x: Register,             // X register
    buffer: Option<Instruction>, // Currently executing instruction
    exec_cycles: Cycles,      // Remaining cycles for current instruction
    ip: Option<IntoIter<Instruction>> // Instruction pointer
}
```

The CPU implementation includes methods for loading instructions, fetching the next instruction, executing instructions, and advancing the clock:

```rust
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
                if self.exec_cycles == 0 {           // exec cycles reached?
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

This implementation models the CPU's behavior:
- `execute` processes one cycle of the current instruction and returns whether execution is ongoing
- `tick` advances the CPU by one cycle, either continuing execution of the current instruction or fetching a new one
- `reg_x` provides access to the current value of the X register

### CRT Display

The CRT display is modeled as a separate component:

```rust
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

The CRT:
- Tracks its own clock cycle
- Draws a pixel based on the current cycle and the X register value
- Automatically handles line breaks when reaching the end of a row

### Parsing Instructions

The input is parsed into a sequence of instructions:

```rust
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

This function:
1. Converts each line into an `Instruction`
2. Sets the appropriate number of cycles for each instruction type (1 for `noop`, 2 for `addx`)
3. Returns both the instructions and the total number of cycles they'll take to execute

### Main Simulation

The main simulation brings everything together:

```rust
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

    println!("{sum} is the sum of signal strengths at {:?}", sample_intervals);
}
```

The main function:
1. Sets up the sample intervals for signal strength measurement
2. Creates the CPU and CRT
3. Parses the instructions and loads them into the CPU
4. Runs the simulation for the specified number of cycles, ticking both CPU and CRT each cycle
5. Filters for the specific cycles we need to sample
6. Calculates the signal strength at those cycles
7. Sums the signal strengths for Part 1

Part 2's output is handled automatically by the CRT's `draw` method, which prints the characters directly to the console.

## Algorithm Analysis

### Time Complexity

- Parsing the input: O(n) where n is the number of instructions
- Simulating the CPU: O(c) where c is the total number of cycles
- Overall: O(n + c), which is effectively O(c) since the number of cycles is proportional to the number of instructions

### Space Complexity

- Storing instructions: O(n) where n is the number of instructions
- CPU state: O(1)
- CRT state: O(1)
- Overall: O(n)

## Alternative Approaches

### Simplified CPU Model

Instead of modeling the CPU with an instruction buffer and execution cycles, we could use a simpler approach that just keeps track of the current instruction and cycles:

```rust
struct SimplifiedCPU {
    x: isize,
    cycle: usize,
    instructions: Vec<(String, isize)>
}

impl SimplifiedCPU {
    fn run(&mut self) -> Vec<(usize, isize)> {
        let mut history = Vec::new();
        let mut pc = 0;
        
        while pc < self.instructions.len() {
            let (instr, val) = &self.instructions[pc];
            
            match instr.as_str() {
                "noop" => {
                    self.cycle += 1;
                    history.push((self.cycle, self.x));
                }
                "addx" => {
                    self.cycle += 1;
                    history.push((self.cycle, self.x));
                    self.cycle += 1;
                    history.push((self.cycle, self.x));
                    self.x += val;
                }
                _ => panic!("Unknown instruction")
            }
            
            pc += 1;
        }
        
        history
    }
}
```

This approach is more straightforward but less flexible if we wanted to add more instructions or change the behavior.

### CRT as a String Buffer

Instead of printing directly, the CRT could build a string buffer:

```rust
struct BufferedCRT {
    width: usize,
    height: usize,
    buffer: Vec<char>,
    position: usize
}

impl BufferedCRT {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec!['.'; width * height],
            position: 0
        }
    }
    
    fn draw(&mut self, sprite_pos: isize) {
        let col = self.position % self.width;
        if (sprite_pos-1..=sprite_pos+1).contains(&(col as isize)) {
            self.buffer[self.position] = '#';
        }
        self.position += 1;
    }
    
    fn display(&self) -> String {
        self.buffer.chunks(self.width)
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
```

This would allow us to build up the entire display and then render it all at once, which might be preferable for some applications.

## Conclusion

This solution demonstrates how to simulate a simple CPU and CRT display. The modular approach with separate CPU and CRT components makes the code clean and maintainable. The use of Rust's pattern matching and option handling helps elegantly manage the CPU's state and instruction execution.