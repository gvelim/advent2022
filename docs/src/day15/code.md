# Day 15: Code

Below is the complete code for Day 15's solution, which analyzes sensor coverage to find positions where beacons cannot be present.

## Full Solution

```advent2022/src/bin/day15.rs#L1-167
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
                .collect::<Vec<_>>()
        }
    }
    fn beacons_at(&self, line:isize) -> HashSet<Coord> {
        self.sensors.iter().filter_map(|s| if s.beacon.y == line { Some(s.beacon)} else {None}).collect::<HashSet<_>>()
    }
    fn sensor_coverage_at(&self, line: isize) -> Vec<RangeInclusive<isize>> {

        let mut result = vec![];

        let mut ranges = self.sensors.iter()
                .filter_map(|sensor| sensor.coverage_at(line))
                .collect::<Vec<_>>();

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
        if diff <= self.dist {
            Some(RangeInclusive::new(
                x.saturating_sub_unsigned(self.dist - diff),
                x.saturating_add_unsigned(self.dist - diff))
            )
        } else {
            None
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
```

## Code Walkthrough

### Core Data Structures

```advent2022/src/bin/day15.rs#L147-155
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
```

The `Coord` struct represents a 2D coordinate with x and y values. It implements several traits to make it comparable, hashable, and printable.

```advent2022/src/bin/day15.rs#L109-111
#[derive(Eq, PartialEq, Hash)]
struct Sensor {
    pos: Coord,
    beacon: Coord,
    dist: usize
}
```

The `Sensor` struct contains information about a sensor's position, its closest beacon's position, and the Manhattan distance between them.

```advent2022/src/bin/day15.rs#L44-46
struct Area {
    sensors: Vec<Sensor>
}
```

The `Area` struct is a container for all sensors in the input.

### Sensor Coverage Calculation

```advent2022/src/bin/day15.rs#L112-123
fn coverage_at(&self, d: isize) -> Option<RangeInclusive<isize>> {
    let Coord{x,y} = self.pos;
    let diff = y.abs_diff(d);
    if diff <= self.dist {
        Some(RangeInclusive::new(
            x.saturating_sub_unsigned(self.dist - diff),
            x.saturating_add_unsigned(self.dist - diff))
        )
    } else {
        None
    }
}
```

This method calculates the x-coordinate range that a sensor can cover at a specific y-coordinate. It:
1. Calculates the vertical distance to the target line
2. If this distance is within the sensor's range, calculates the horizontal range
3. Returns the range, or `None` if the line is out of range

### Analyzing Sensor Coverage on a Row

```advent2022/src/bin/day15.rs#L67-99
fn sensor_coverage_at(&self, line: isize) -> Vec<RangeInclusive<isize>> {

    let mut result = vec![];

    let mut ranges = self.sensors.iter()
            .filter_map(|sensor| sensor.coverage_at(line))
            .collect::<Vec<_>>();

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
```

This method aggregates coverage from all sensors on a specific row:
1. Collects ranges from all sensors that cover the specified row
2. Sorts the ranges by their start position
3. Merges overlapping ranges using a `reduce` operation
4. Returns a vector of non-overlapping ranges representing total coverage

### Finding Beacons on a Row

```advent2022/src/bin/day15.rs#L64-66
fn beacons_at(&self, line:isize) -> HashSet<Coord> {
    self.sensors.iter().filter_map(|s| if s.beacon.y == line { Some(s.beacon)} else {None}).collect::<HashSet<_>>()
}
```

This method identifies all beacons located on a specific row.

### Parsing Input

```advent2022/src/bin/day15.rs#L47-63
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
            .collect::<Vec<_>>()
    }
}
```

This method parses the input text into `Sensor` objects by:
1. Splitting each line into parts
2. Filtering out non-numeric parts
3. Converting numeric strings to integers
4. Constructing sensors with their positions, beacon positions, and distances

### Main Function

```advent2022/src/bin/day15.rs#L17-42
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
```

The main function:

1. Reads and parses the input file
2. For Part 1:
   - Gets the sensor coverage on row 2000000
   - Identifies beacons already on that row
   - Calculates the number of positions that cannot contain a beacon
3. For Part 2:
   - Checks each row in the search area (0 to 4000000)
   - Finds a row where the coverage is split with a gap of exactly one position
   - Calculates the tuning frequency of the distress beacon

The key insight for Part 2 is that the distress beacon must be in a position that is just outside the range of multiple sensors, which appears as a gap in the coverage.

## Implementation Notes

- **Range Representation**: The solution uses `RangeInclusive<isize>` to represent coverage ranges efficiently
- **Merge Algorithm**: Overlapping ranges are merged, significantly reducing the number of ranges needed to represent coverage
- **Efficient Searching**: The solution for Part 2 efficiently finds the gap by examining rows with split coverage rather than checking every position