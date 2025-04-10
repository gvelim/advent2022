# Day 11: Solution Explanation

## Approach

Day 11 involves simulating monkeys playing keep-away with items, applying operations to worry levels, and passing items between monkeys based on tests. The key challenges are:

1. **Parsing the monkey specifications** from the input text
2. **Modeling monkeys and their behavior** with appropriate data structures
3. **Simulating the rounds** of monkey inspections and item throwing
4. **Managing worry levels** efficiently, especially for Part 2

The solution uses a combination of custom data types and simulation logic to model the monkey behavior accurately.

## Implementation Details

### Data Structures

First, we define a type for representing worry levels and the operation that monkeys can perform:

```rust
type WorryType = u64;
const WORRY_DEF: WorryType = 0;

#[derive(Debug)]
enum Operation {
    Add(WorryType),
    Mul(WorryType),
}
```

`WorryType` is set to `u64` to handle the large numbers that can occur during the simulation. The `Operation` enum represents the two possible operations a monkey can perform: addition or multiplication.

The `Monkey` struct represents all the properties of a monkey:

```rust
#[derive(Debug)]
struct Monkey {
    name: usize,
    items: VecDeque<WorryType>,
    op: Operation,
    test: WorryType,
    send: (usize,usize),
    inspect: usize
}
```

Each monkey has:
- A name (index)
- A queue of items (worry levels)
- An operation to apply when inspecting items
- A divisibility test value
- Two target monkeys to throw to based on the test result
- A counter for the number of inspections performed

### Parsing Input

The solution uses Rust's `FromStr` trait to parse monkey specifications from the input text:

```rust
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
```

This implementation parses each line of the monkey specification and sets the corresponding fields in the `Monkey` struct.

### Monkey Behavior

The `Monkey` struct implements several methods to model its behavior:

```rust
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
            // Current worry level is divisible by the test value
            (self.send.0, worry)
        } else {
            // Current worry level is not divisible by the test value
            (self.send.1, worry)
        }
    }
    
    fn observe(&mut self, div: WorryType) -> Option<(usize, WorryType)> {
        self.inspect += 1;
        // Monkey inspects an item with a worry level
        match self.items.pop_front() {
            Some(mut worry) => {
                // Apply the modulo to keep worry levels manageable
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
```

These methods handle:
- Parsing all monkeys from the input text
- Catching items thrown by other monkeys
- Throwing items to other monkeys based on the test result
- Observing (inspecting) an item and updating its worry level
- Observing all items in a monkey's possession
- Tracking the number of inspections

### Managing Worry Levels

In Part 2, the challenge is managing the worry levels since they're no longer divided by 3 and can grow extremely large. The key insight is that we don't need the exact worry levels, only whether they're divisible by the monkeys' test values.

Using the Chinese Remainder Theorem, we can apply modular arithmetic with the product of all monkeys' test values as the modulus. This keeps the worry levels manageable while preserving divisibility properties:

```rust
let div_product: WorryType = monkeys.iter().map(|m| m.test).product();
```

This technique is applied in the `observe` method where we calculate `worry %= div`.

### Simulation Logic

The main simulation logic runs for the specified number of rounds and tracks the items as they're thrown between monkeys:

```rust
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
                .all(|throw|
                    throw.map(
                        |(monkey,item)| queue[monkey].push_back(item)
                    ).is_some()
                )
        })
        .all(|run| run)
});
```

The simulation:
1. Iterates through each round
2. For each monkey, processes all items in its possession
3. Calculates new worry levels and determines target monkeys
4. Uses queues to handle the items being thrown between monkeys

### Calculating Monkey Business

Finally, the solution calculates the level of monkey business by multiplying the inspection counts of the two most active monkeys:

```rust
monkeys.sort_by(|a,b| b.inspect.cmp(&a.inspect));
println!("level of monkey business after 10000 rounds : {:?}",
         monkeys[0].inspections() * monkeys[1].inspections()
);
```

## Algorithmic Analysis

### Time Complexity

- Parsing input: O(n) where n is the length of the input text
- Simulation: O(r * m * i) where:
  - r is the number of rounds (10,000 for Part 2)
  - m is the number of monkeys
  - i is the average number of items per monkey

### Space Complexity

- O(m * i) for storing the monkeys and their items
- O(m) for the queues used to pass items between monkeys

## Key Insights

### Chinese Remainder Theorem Application

The key insight for Part 2 is using modular arithmetic to manage worry levels. Since we only care about divisibility by each monkey's test value, we can use the product of all test values as a modulus.

This works because if we have:
- Original worry level: W
- Modulus: M = product of all test divisors
- Remainder: R = W mod M

Then for any test divisor D that is a factor of M:
- W is divisible by D if and only if R is divisible by D

This allows us to keep the worry levels manageable while preserving the divisibility properties needed for the monkey's tests.

## Alternative Approaches

### Direct Divisibility Tracking

Instead of tracking the actual worry levels, we could track just the remainders when divided by each monkey's test value:

```rust
struct Item {
    remainders: HashMap<WorryType, WorryType>, // Map from test value to remainder
}
```

This would allow us to update the remainders directly without ever dealing with the full worry values. However, this is more complex to implement and likely not necessary given the effectiveness of the modulo approach.

### Simulation Optimization

For a large number of rounds, we could look for patterns in the monkey's behavior and potentially skip ahead in the simulation. However, this would add complexity and might not be necessary for the given constraints.

## Conclusion

This solution demonstrates how to model a complex system with multiple interacting entities. The key insights are:

1. Using appropriate data structures to model the monkeys and their behavior
2. Applying modular arithmetic to manage worry levels efficiently
3. Using queues to handle the passing of items between monkeys

These techniques allow us to simulate the monkey's behavior for a large number of rounds without running into numerical overflow issues.