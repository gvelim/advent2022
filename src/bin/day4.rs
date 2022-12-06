use std::collections::HashSet;
use std::str::FromStr;

fn main() {

    let data = std::fs::read_to_string("src/bin/day4_input.txt").expect("Ops! Cannot read file");
    let pairs = data.lines()
        .map(|line|
            line.split(|c:char| c.is_ascii_punctuation())
                .map(|c| u32::from_str(c).unwrap_or_else(|e| panic!("{e}")) )
                .collect::<Vec<_>>()
        ).map(|pair| {
            let [a, b, c, d] = pair[..] else { panic!("") };
            (
                (a..=b).collect::<HashSet<_>>(),
                (c..=d).collect::<HashSet<_>>()
            )
        })
        .collect::<Vec<_>>();

    let out = pairs.iter()
        .filter(|(a,b)|
            match  a.len() < b.len() {
                true => a.is_subset(&b),
                false => b.is_subset(&a)
            }
        )
        .count();
    println!("Component 1 = {out}");

    let len = pairs.len();
    let out = len - pairs.iter()
        .filter(|(a,b)| a.is_disjoint(&b) )
        .count();
    println!("Component 2 = {out}");
}