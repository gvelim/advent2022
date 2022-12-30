use std::fmt::{Debug, Formatter, write};
use std::ops::RangeInclusive;
use std::str::FromStr;

const input : &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

fn main() {
    let sensors = Sensor::parse(input);

    sensors.iter()
        .filter_map(|sensor| sensor.signal_at(10))
        .inspect(|e| println!("{:?}",e))
        .all(|l| true);
}

struct Sensor {
    pos: Coord,
    beacon: Coord,
    dist: usize
}
impl Sensor {
    fn signal_at(&self, d: isize) -> Option<RangeInclusive<isize>> {
        let Coord{x,y} = self.pos;
        let diff = y.abs_diff(d);
        match diff {
            n if n <= self.dist =>
                Some(RangeInclusive::new(
                    x.saturating_sub_unsigned(self.dist - diff),
                    x.saturating_add_unsigned(self.dist - diff))
                ),
            _ => None
        }
    }
    fn parse(inp: &str) -> Vec<Sensor> {
        inp.lines()
            .map(|line|
                line.split(&[' ','=',',',':'])
                    .filter(|item| !item.trim().is_empty() )
                    .filter(|item| item.chars().all(|d| d.is_numeric() || d == '-'))
                    .filter_map(|n| isize::from_str(n).ok())
                    .collect::<Vec<_>>()
            )
            .map(|comb|
                Sensor {
                    pos: (comb[0],comb[1]).into(),
                    beacon: (comb[2],comb[3]).into(),
                    dist: comb[0].abs_diff(comb[2]) + comb[1].abs_diff(comb[3])
                }
            )
            // .inspect(|e| print!("{:?},",e))
            .fold( vec![], |mut out, mut sensor| {
                out.push(sensor);
                out
            })
    }
}

impl Debug for Sensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "S{:?} {:?} B{:?}",self.pos, self.dist, self.beacon)
    }
}

/// Generics
///

#[derive(Ord, PartialOrd,Copy, Clone, Eq, PartialEq)]
struct Coord {
    x: isize,
    y: isize
}
impl Coord {
    fn dist_manhattan(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}
impl Debug for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})",self.x,self.y)
    }
}
impl From<(isize,isize)> for Coord {
    fn from(p: (isize, isize)) -> Self {
        Coord { x:p.0, y:p.1 }
    }
}
