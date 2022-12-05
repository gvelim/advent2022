use std::collections::HashSet;
use std::str::FromStr;

fn main() {

    // let data = "2-4,6-8\n\
    // 2-3,4-5\n\
    // 5-7,7-9\n\
    // 2-8,3-7\n\
    // 6-6,4-6\n\
    // 2-6,4-8";

    let data = std::fs::read_to_string("src/bin/day4_input.txt").expect("Ops! Cannot read file");
    let out = data.lines()
        .map(|line|
            line.split(|c:char| c.is_ascii_punctuation())
                .map(|c| u32::from_str(c).unwrap_or_else(|e| panic!("{e}")) )
                .collect::<Vec<_>>()
        )
        .filter(|pair| {
            let [a, b, c, d] = pair[..] else { panic!("") };
            let A = (a..=b).collect::<HashSet<_>>();
            let B = (c..=d).collect::<HashSet<_>>();
            A.is_subset(&B) || B.is_subset(&A)
        })
        .count();
    println!("{out}");
}