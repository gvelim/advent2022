#[derive(Debug,Copy,Clone)]
enum RPS { Rock=1, Paper, Scissor }
enum Outcome { Win, Loss, Draw }
impl From<RPS> for u64 {
    fn from(rps: RPS) -> Self {
        match rps {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissor => 3
        }
    }
}
impl From<u8> for RPS {
    fn from(c: u8) -> Self {
        match c {
            b'A' | b'X' => RPS::Rock,
            b'B' | b'Y' => RPS::Paper,
            b'C' | b'Z' => RPS::Scissor,
            _ => unreachable!()
        }
    }
}
impl RPS {
    fn read_positions(round:&str) -> (RPS,RPS) {
        if let &[a,_,b] = round.as_bytes() { (RPS::from(a),RPS::from(b)) } else { unreachable!() }
    }
    fn derive_positions(round:&str) -> (RPS, RPS) {
        let (a,outcome) = RPS::read_positions(round);
        (a, match (a,outcome) {
            (RPS::Rock, RPS::Rock) => RPS::Scissor,
            (RPS::Paper, RPS::Rock) => RPS::Rock,
            (RPS::Scissor, RPS::Rock) => RPS::Paper,
            (RPS::Rock, RPS::Paper) => RPS::Rock,
            (RPS::Paper, RPS::Paper) => RPS::Paper,
            (RPS::Scissor, RPS::Paper) => RPS::Scissor,
            (RPS::Rock, RPS::Scissor) => RPS::Paper,
            (RPS::Paper, RPS::Scissor) => RPS::Scissor,
            (RPS::Scissor, RPS::Scissor) => RPS::Rock,
        }
        )
    }
    fn strategy_1(round:&str) -> Outcome {
        match RPS::read_positions(round) {
            (RPS::Rock, RPS::Rock) |
            (RPS::Paper, RPS::Paper) |
            (RPS::Scissor, RPS::Scissor) => Outcome::Draw,
            (RPS::Rock, RPS::Paper) |
            (RPS::Paper, RPS::Scissor) |
            (RPS::Scissor, RPS::Rock) =>Outcome::Win,
            (RPS::Rock, RPS::Scissor) |
            (RPS::Paper, RPS::Rock) |
            (RPS::Scissor, RPS::Paper) => Outcome::Loss,
        }
    }
    fn strategy_2(round:&str) -> Outcome {
        let (_,res) = RPS::read_positions(round);
        match res {
            RPS::Rock => Outcome::Loss,
            RPS::Paper => Outcome::Draw,
            RPS::Scissor => Outcome::Win,
        }
    }
    fn score(round:&str, positions:fn(&str)->(RPS, RPS), strategy:fn(&str)->Outcome) -> u64 {
        let (_,b) = positions(round);
        match strategy(round) {
            Outcome::Win => b as u64 + 6,
            Outcome::Loss => b as u64 + 0,
            Outcome::Draw => b as u64 + 3
        }
    }
}

fn main() {
    let (score1, score2) = std::fs::read_to_string("./src/bin/day2_input.txt")
        .unwrap()
        .lines()
        .map(|round| (
            RPS::score(round, RPS::read_positions, RPS::strategy_1),
            RPS::score(round, RPS::derive_positions, RPS::strategy_2)
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
