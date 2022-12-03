fn main() {
    fn strategy_1(round:&str) -> u64 {
        match round {
            // Question 1: ABC, XYZ denotes player choices
            "A X" => 3+1,
            "A Y" => 6+2,
            "A Z" => 0+3,
            "B X" => 0+1,
            "B Y" => 3+2,
            "B Z" => 6+3,
            "C X" => 6+1,
            "C Y" => 0+2,
            "C Z" => 3+3,
            _ => panic!("unknown input")
        }
    }
    fn strategy_2(round:&str) -> u64 {
        match round {
            // Question 2: XYZ denotes your choice results to loose, draw, win
            "A X" => 0+3,
            "A Y" => 3+1,
            "A Z" => 6+2,
            "B X" => 0+1,
            "B Y" => 3+2,
            "B Z" => 6+3,
            "C X" => 0+2,
            "C Y" => 3+3,
            "C Z" => 6+1,
            _ => panic!("unknown input")
        }
    }

    let (score1, score2) = std::fs::read_to_string("./src/bin/day2_input.txt")
        .unwrap()
        .lines()
        .map(|round| (strategy_1(round), strategy_2(round)) )
        .reduce(|mut acc, round| {
            acc.0 += round.0;
            acc.1 += round.1;
            acc
        })
        .unwrap_or_else(|| panic!("Empty iterator ?"));
    println!("Strategy 1 : {:?}",score1);
    println!("Strategy 2 : {:?}",score2);
}