use std::ops::RangeInclusive;
use std::str::FromStr;

trait InclusiveRangeExt {
    fn is_subset(&self, other: &Self) -> bool;
    fn is_overlapping(&self, other: &Self) -> bool;
}

impl<T> InclusiveRangeExt for RangeInclusive<T>
    where T : PartialOrd {
    fn is_subset(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }
    fn is_overlapping(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

fn main() {

    let data = std::fs::read_to_string("src/bin/day4_input.txt").expect("Ops! Cannot read file");
    let pairs = data.lines()
        .map(|line|
            line.split(|c:char| c.is_ascii_punctuation())
                .map(|c| u32::from_str(c).unwrap_or_else(|e| panic!("{e}")) )
                .collect::<Vec<_>>()
        )
        .map(|pair| {
            let [a, b, c, d] = pair[..] else { panic!("") };
            ((a..=b), (c..=d))
        })
        .collect::<Vec<_>>();

    let out = pairs.iter()
        .filter(|(a,b)|
            a.is_subset(b) || b.is_subset(a)
        )
        .count();
    println!("Component 1 = {out}");

    let out = pairs.iter()
        .filter(|(a,b)|
            a.is_overlapping(b) || b.is_overlapping(a)
        )
        .count();
    println!("Component 2 = {out}");
}