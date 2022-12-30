use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::ops::RangeInclusive;
use std::str::FromStr;

// const INPUT : &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
// Sensor at x=9, y=16: closest beacon is at x=10, y=16
// Sensor at x=13, y=2: closest beacon is at x=15, y=3
// Sensor at x=12, y=14: closest beacon is at x=10, y=16
// Sensor at x=10, y=20: closest beacon is at x=10, y=16
// Sensor at x=14, y=17: closest beacon is at x=10, y=16
// Sensor at x=8, y=7: closest beacon is at x=2, y=10
// Sensor at x=2, y=0: closest beacon is at x=2, y=10
// Sensor at x=0, y=11: closest beacon is at x=2, y=10
// Sensor at x=20, y=14: closest beacon is at x=25, y=17
// Sensor at x=17, y=20: closest beacon is at x=21, y=22
// Sensor at x=16, y=7: closest beacon is at x=15, y=3
// Sensor at x=14, y=3: closest beacon is at x=15, y=3
// Sensor at x=20, y=1: closest beacon is at x=15, y=3";

fn main() {
    let input = std::fs::read_to_string("src/bin/day15_input.txt").expect("Ops!");

    let area = Area::deploy_sensors(input.as_str());

    // Component 1
    let res = area.sensor_coverage_at(2000000);
    println!("Signal Coverage @2000000 = {:?}",res);
    let beacons = area.beacons_at(2000000);
    println!("Beacons = {:?}",beacons);

    let positions = res.into_iter()
        .map(|r| r.count())
        .sum::<usize>();
    println!("{}-{}={} (4793062)", positions,beacons.len(),positions-beacons.len());

    // Component 2
    let (line, v) = (0..=4000000)
        .map(|line| (line,area.sensor_coverage_at(line)))
        .filter(|(_,v)| v.len() > 1 )
        .filter(|(_,v)| v[1].start() - v[0].end() > 1 )
        .next().unwrap();

    let total = (v[0].end() + 1) * 4000000 + line;
    println!("Signal Coverage @{line} = {:?} \nFreq of distress beacon: {total}", v);
}

struct Area {
    sensors: Vec<Sensor>
}
impl Area {
    fn deploy_sensors(sensors:&str ) -> Area {
        Area {
            sensors: sensors.lines()
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
                .fold( vec![], |mut out, sensor| {
                    out.push(sensor);
                    out
                })
        }
    }
    fn beacons_at(&self, line:isize) -> HashSet<Coord> {
        self.sensors.iter().filter_map(|s| if s.beacon.y == line { Some(s.beacon)} else {None}).collect::<HashSet<_>>()
    }
    fn sensor_coverage_at(&self, line: isize) -> Vec<RangeInclusive<isize>> {
        Area::merge_ranges(
            self.sensors.iter()
                .filter_map(|sensor| sensor.coverage_at(line))
                .fold(vec![], |mut out, r| {
                    out.push(r);
                    out
                })
        )
    }
    fn merge_ranges(mut ranges: Vec<RangeInclusive<isize>>) -> Vec<RangeInclusive<isize>> {
        let mut result = vec![];

        ranges.sort_by_key(|a| *a.start());

        if let Some(last) = ranges.into_iter()
            .reduce(|a, b|
                if a.end() >= &(b.start()-1) {
                    if a.end() < b.end() {
                        *a.start()..=*b.end()
                    } else { a }
                } else {
                    // We got a range gap here hence we must save range A
                    // while we pass on Range B to the next iteration
                    result.push(a);
                    b
                }
            ) {
            result.push(last);
        }
        result
    }
}

#[derive(Eq, PartialEq, Hash)]
struct Sensor {
    pos: Coord,
    beacon: Coord,
    dist: usize
}
impl Sensor {
    fn coverage_at(&self, d: isize) -> Option<RangeInclusive<isize>> {
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
}

impl Debug for Sensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "S{:?} {:?} B{:?}",self.pos, self.dist, self.beacon)
    }
}

/// Generics
///

#[derive(Ord, PartialOrd,Copy, Clone, Eq, PartialEq,Hash)]
struct Coord {
    x: isize,
    y: isize
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
