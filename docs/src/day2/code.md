# Day 2: Code

Below is the complete code for Day 2's solution, which implements Rock Paper Scissors with two different interpretations of the strategy guide.

## Full Solution

```advent2022/src/bin/day2.rs#L1-89
use std::iter::once;

#[derive(Debug,Copy,Clone,PartialEq)]
enum Move { Rock=1, Paper, Scissors }
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
impl Move {
    fn is_winning(&self, other:&Self) -> bool {
        matches!(
            (other,self),
            (Move::Rock, Move::Paper) |
            (Move::Paper, Move::Scissors) |
            (Move::Scissors, Move::Rock)
        )
    }
    fn outcome(&self, other:&Self) -> Outcome {
        if self.is_winning(other) {
            Outcome::Win
        } else if other.is_winning(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }
    fn derive(&self, out:Outcome) -> Move {
        let iter = once(Move::Rock).chain(once(Move::Paper)).chain(once(Move::Scissors)).cycle();
        // match out {
        //     Outcome::Draw => iter.skip_while(|e| self != e).skip(0).next(),
        //     Outcome::Win => iter.skip_while(|e| self != e).skip(1).next()
        //     Outcome::Loss => iter.skip_while(|e| self != e).skip(2).next(),
        // }.unwrap()
        iter.skip_while(|e| self != e).skip(out as usize).next().unwrap()
    }
}
#[derive(Debug,Copy,Clone)]
enum Outcome { Draw, Win, Loss }
impl From<Move> for Outcome {
    fn from(m: Move) -> Self {
        match m {
            Move::Rock => Outcome::Loss,
            Move::Paper => Outcome::Draw,
            Move::Scissors => Outcome::Win
        }
    }
}
impl Outcome {
    fn score_value(&self) -> u64 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6
        }
    }
}
#[derive(Debug,Copy,Clone)]
struct Round(Move,Move);
impl Round {
    fn new(round:&str) -> Round {
        if let &[a,_,b] = round.as_bytes() { Round(Move::from(a), Move::from(b)) } else { unreachable!() }
    }
    fn derived(round:&str) -> Round {
        let Round(a,b) = Round::new(round);
        Round(a, a.derive(Outcome::from(b)))
    }
    fn score(&self) -> u64 {
        let Round(other, me) = self;
        me.outcome(other).score_value() + *me as u64
    }
}

fn main() {
    let (score1, score2) = std::fs::read_to_string("./src/bin/day2_input.txt")
        .unwrap()
        .lines()
        .map(|round| (
            Round::new(round).score(),
            Round::derived(round).score()
        ))
        .reduce(|sum, round| {
            (sum.0 + round.0, sum.1 + round.1)
        })
        .unwrap_or_else(|| panic!("Empty iterator ?"));
    println!("Strategy 1 : 15632 {:?}",score1);
    println!("Strategy 2 : 14416 {:?}",score2);
}

// fn strategy_1(round:&str) -> u64 {
//     match round {
//         // Question 1: ABC, XYZ denotes player choices
//         "A X" => 3+1,
//         "A Y" => 6+2,
//         "A Z" => 0+3,
//         "B X" => 0+1,
//         "B Y" => 3+2,
//         "B Z" => 6+3,
//         "C X" => 6+1,
//         "C Y" => 0+2,
//         "C Z" => 3+3,
//         _ => panic!("unknown input")
//     }
// }
// fn strategy_2(round:&str) -> u64 {
//     match round {
//         // Question 2: XYZ denotes your choice results to loose, draw, win
//         "A X" => 0+3,
//         "A Y" => 3+1,
//         "A Z" => 6+2,
//         "B X" => 0+1,
//         "B Y" => 3+2,
//         "B Z" => 6+3,
//         "C X" => 0+2,
//         "C Y" => 3+3,
//         "C Z" => 6+1,
//         _ => panic!("unknown input")
//     }
// }
```

## Code Walkthrough

### Core Types

The solution uses three main types:

1. **Move Enum**: Represents Rock, Paper, or Scissors with their score values:

```advent2022/src/bin/day2.rs#L3-4
#[derive(Debug,Copy,Clone,PartialEq)]
enum Move { Rock=1, Paper, Scissors }
```

2. **Outcome Enum**: Represents the possible outcomes of a round:

```advent2022/src/bin/day2.rs#L44-45
#[derive(Debug,Copy,Clone)]
enum Outcome { Draw, Win, Loss }
```

3. **Round Struct**: Represents a round of Rock Paper Scissors:

```advent2022/src/bin/day2.rs#L63-64
#[derive(Debug,Copy,Clone)]
struct Round(Move,Move);
```

### Game Logic

The solution implements several key methods:

1. **Determining Win Conditions**:

```advent2022/src/bin/day2.rs#L16-23
fn is_winning(&self, other:&Self) -> bool {
    matches!(
        (other,self),
        (Move::Rock, Move::Paper) |
        (Move::Paper, Move::Scissors) |
        (Move::Scissors, Move::Rock)
    )
}
```

2. **Determining Game Outcomes**:

```advent2022/src/bin/day2.rs#L24-32
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

3. **Deriving Moves Based on Desired Outcome**:

```advent2022/src/bin/day2.rs#L33-40
fn derive(&self, out:Outcome) -> Move {
    let iter = once(Move::Rock).chain(once(Move::Paper)).chain(once(Move::Scissors)).cycle();
    // Commented code shows an alternative approach using match
    iter.skip_while(|e| self != e).skip(out as usize).next().unwrap()
}
```

### Processing Input

The solution processes the input file and calculates scores for both strategies in a single pass:

```advent2022/src/bin/day2.rs#L75-85
let (score1, score2) = std::fs::read_to_string("./src/bin/day2_input.txt")
    .unwrap()
    .lines()
    .map(|round| (
        Round::new(round).score(),
        Round::derived(round).score()
    ))
    .reduce(|sum, round| {
        (sum.0 + round.0, sum.1 + round.1)
    })
    .unwrap_or_else(|| panic!("Empty iterator ?"));
```

### Alternative Approach

The commented-out functions at the end show an alternative approach using direct pattern matching for each input combination:

```advent2022/src/bin/day2.rs#L87-115
// fn strategy_1(round:&str) -> u64 {
//     match round {
//         // Question 1: ABC, XYZ denotes player choices
//         "A X" => 3+1,
//         "A Y" => 6+2,
//         "A Z" => 0+3,
//         "B X" => 0+1,
//         "B Y" => 3+2,
//         "B Z" => 6+3,
//         "C X" => 6+1,
//         "C Y" => 0+2,
//         "C Z" => 3+3,
//         _ => panic!("unknown input")
//     }
// }
// fn strategy_2(round:&str) -> u64 {
//     match round {
//         // Question 2: XYZ denotes your choice results to loose, draw, win
//         "A X" => 0+3,
//         "A Y" => 3+1,
//         "A Z" => 6+2,
//         "B X" => 0+1,
//         "B Y" => 3+2,
//         "B Z" => 6+3,
//         "C X" => 0+2,
//         "C Y" => 3+3,
//         "C Z" => 6+1,
//         _ => panic!("unknown input")
//     }
// }
```

This approach is more direct but less flexible than modeling the game with proper types.