# Day 2: Solution Explanation

## Approach

Day 2's problem requires implementing a Rock Paper Scissors game with two different interpretations of a strategy guide. We need to:

1. Parse the input into rounds of play
2. Calculate scores for each round according to both interpretations
3. Sum the scores to get the total

### Strategy 1 vs Strategy 2

The key difference between the two strategies is the interpretation of the second column:

- **Strategy 1**: The second column (X, Y, Z) represents your move (Rock, Paper, Scissors)
- **Strategy 2**: The second column represents the desired outcome (Lose, Draw, Win)

### Game Logic

To implement the game, we need to model:

1. The possible moves (Rock, Paper, Scissors)
2. The possible outcomes (Win, Loss, Draw)
3. The scoring rules for moves and outcomes
4. The winning relationships between moves
5. How to derive a move given an opponent's move and a desired outcome

## Implementation Details

### The Move Enum

We define a `Move` enum with values for Rock, Paper, and Scissors, each with its corresponding score value:

```rust
#[derive(Debug,Copy,Clone,PartialEq)]
enum Move { Rock=1, Paper, Scissors }
```

The numeric values (1, 2, 3) are automatically assigned based on the enum declaration order.

### Parsing Input

We implement the `From<u8>` trait to convert characters from the input into `Move` values:

```rust
impl From<u8> for Move {
    fn from(c: u8) -> Self {
        match c {
            b'A' | b'X' => Move::Rock,
            b'B' | b'Y' => Move::Paper,
            b'C' | b'Z' => Move::Scissors,
            _ => unreachable!()
        }
    }
}
```

### Determining Outcomes

We implement a method to determine if one move wins against another:

```rust
fn is_winning(&self, other:&Self) -> bool {
    matches!(
        (other,self),
        (Move::Rock, Move::Paper) |
        (Move::Paper, Move::Scissors) |
        (Move::Scissors, Move::Rock)
    )
}
```

And a method to determine the outcome of a round:

```rust
fn outcome(&self, other:&Self) -> Outcome {
    if self.is_winning(other) {
        Outcome::Win
    } else if other.is_winning(self) {
        Outcome::Loss
    } else {
        Outcome::Draw
    }
}
```

### Strategy 2: Deriving Moves

For Strategy 2, we need to determine what move to make given an opponent's move and a desired outcome:

```rust
fn derive(&self, out:Outcome) -> Move {
    let iter = once(Move::Rock).chain(once(Move::Paper)).chain(once(Move::Scissors)).cycle();
    iter.skip_while(|e| self != e).skip(out as usize).next().unwrap()
}
```

This is a clever solution that creates a circular iterator of moves and skips to the move that produces the desired outcome.

### Scoring

We define an `Outcome` enum and implement scoring for outcomes:

```rust
enum Outcome { Draw, Win, Loss }

impl Outcome {
    fn score_value(&self) -> u64 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6
        }
    }
}
```

### Combining Everything

We create a `Round` struct to represent a round of Rock Paper Scissors:

```rust
struct Round(Move,Move);

impl Round {
    fn score(&self) -> u64 {
        let Round(other, me) = self;
        me.outcome(other).score_value() + *me as u64
    }
}
```

Each round is scored by adding the outcome value to the value of the move chosen.

### Processing the Input

Finally, we process the input file, calculating scores for both strategies:

```rust
fn main() {
    let (score1, score2) = std::fs::read_to_string("./src/bin/day2_input.txt")
        .unwrap()
        .lines()
        .map(|round| (
            Round::new(round).score(),      // Strategy 1
            Round::derived(round).score()   // Strategy 2
        ))
        .reduce(|sum, round| {
            (sum.0 + round.0, sum.1 + round.1)
        })
        .unwrap_or_else(|| panic!("Empty iterator ?"));
    
    println!("Strategy 1 : {:?}",score1);
    println!("Strategy 2 : {:?}",score2);
}
```

We map each line to a tuple of scores for both strategies, then reduce the results to get the total scores.

## Alternative Approaches

### Pattern Matching

A simpler approach could use direct pattern matching for each input combination:

```rust
fn strategy_1(round:&str) -> u64 {
    match round {
        "A X" => 3+1, // Rock vs Rock = Draw (3) + Rock (1)
        "A Y" => 6+2, // Rock vs Paper = Win (6) + Paper (2)
        "A Z" => 0+3, // Rock vs Scissors = Loss (0) + Scissors (3)
        // ... other combinations
        _ => panic!("unknown input")
    }
}
```

While this approach is more direct, it's less flexible and doesn't model the game's logic as cleanly.

## Optimization Considerations

- The current solution uses enums to represent both moves and outcomes, which makes the code clear and easy to understand.
- The `derive` method is particularly elegant, using Rust's iterator functionality to find the right move.
- For very large inputs, we could consider using a lookup table for move derivation instead of the iterator approach.