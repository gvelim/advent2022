# Day 15: Solution Explanation

## Approach

Day 15 involves analyzing sensor coverage to find positions where beacons cannot be present. The key challenge is efficiently handling the potentially large search space.

The solution breaks down into several components:

1. **Parsing the input data**: Extract sensor and beacon positions from the input
2. **Calculating sensor coverage**: Determine the area each sensor can cover based on Manhattan distance
3. **Analyzing coverage on specific rows**: Find ranges of positions that cannot contain a beacon
4. **Finding the distress beacon**: Identify the one position where the distress beacon must be located

The key insight is to work with ranges rather than individual positions, which allows for much more efficient processing.

## Implementation Details

### Data Structures

The solution uses several key data structures:

```rust
#[derive(Ord, PartialOrd, Copy, Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: isize,
    y: isize
}

#[derive(Eq, PartialEq, Hash)]
struct Sensor {
    pos: Coord,
    beacon: Coord,
    dist: usize
}

struct Area {
    sensors: Vec<Sensor>
}
```

These structures represent coordinates, sensors, and the overall area being analyzed.

### Parsing the Input

The input is parsed into a collection of sensors:

```rust
fn deploy_sensors(sensors: &str) -> Area {
    Area {
        sensors: sensors.lines()
            .map(|line|
                line.split(&[' ','=',',',':'])
                    .filter(|item| !item.trim().is_empty())
                    .filter(|item| item.chars().all(|d| d.is_numeric() || d == '-'))
                    .filter_map(|n| isize::from_str(n).ok())
                    .collect::<Vec<_>>()
            )
            .map(|comb|
                Sensor {
                    pos: (comb[0], comb[1]).into(),
                    beacon: (comb[2], comb[3]).into(),
                    dist: comb[0].abs_diff(comb[2]) + comb[1].abs_diff(comb[3])
                }
            )
            .collect::<Vec<_>>()
    }
}
```

This function extracts the coordinates from each line and calculates the Manhattan distance between each sensor and its nearest beacon.

### Calculating Sensor Coverage

For each sensor, we need to determine its coverage at a specific y-coordinate. This is done by calculating a range of x-coordinates that the sensor can cover:

```rust
fn coverage_at(&self, d: isize) -> Option<RangeInclusive<isize>> {
    let Coord{x, y} = self.pos;
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

This method:
1. Calculates the vertical distance from the sensor to the specified y-coordinate
2. If this distance is within the sensor's range, calculates the horizontal range the sensor can cover at that y-coordinate
3. Returns the range as a `RangeInclusive<isize>`, or `None` if the y-coordinate is out of range

### Analyzing Coverage on a Row

To determine the coverage on a specific row, we need to combine the ranges from all sensors:

```rust
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

This method:
1. Collects the coverage ranges from all sensors for the specified row
2. Sorts the ranges by their start position
3. Merges overlapping or adjacent ranges
4. Returns a list of non-overlapping ranges representing the total coverage

The merging step is crucial for efficiency, as it allows us to represent large areas of coverage with just a few ranges.

### Finding Beacons on a Row

We also need to identify beacons that are already on the specified row:

```rust
fn beacons_at(&self, line: isize) -> HashSet<Coord> {
    self.sensors.iter()
        .filter_map(|s| if s.beacon.y == line { Some(s.beacon) } else { None })
        .collect::<HashSet<_>>()
}
```

This is used to exclude beacon positions from the count of positions where a beacon cannot be present.

### Finding the Distress Beacon (Part 2)

For Part 2, we need to find the one position in a large area where the distress beacon must be located. The key insight is that this position must be just outside the range of multiple sensors:

```rust
let (line, v) = (0..=4000000)
    .map(|line| (line, area.sensor_coverage_at(line)))
    .filter(|(_, v)| v.len() > 1)
    .filter(|(_, v)| v[1].start() - v[0].end() > 1)
    .next().unwrap();

let total = (v[0].end() + 1) * 4000000 + line;
```

This code:
1. Checks each row in the search area
2. Identifies rows where the coverage is split into multiple ranges
3. Finds the first row where there's a gap of exactly one position between ranges
4. Calculates the tuning frequency based on the position in the gap

This approach is much more efficient than checking every possible position, as it only needs to examine rows where the coverage is not continuous.

## Algorithmic Analysis

### Time Complexity

- **Parsing**: O(n) where n is the number of sensors
- **Coverage Calculation**: O(n) for each row analyzed
- **Range Merging**: O(n log n) due to the sorting step
- **Part 1**: O(n log n)
- **Part 2**: O(y * n log n) where y is the number of rows in the search area

### Space Complexity

- **Storage**: O(n) for storing the sensors and their information
- **Range Processing**: O(n) for storing the ranges during processing

## Alternative Approaches

### Grid-Based Approach

A naive approach would be to use a grid to track each position:

```rust
fn count_positions_without_beacon(sensors: &[Sensor], y: isize, x_range: RangeInclusive<isize>) -> usize {
    let mut count = 0;
    for x in x_range {
        let pos = Coord { x, y };
        if sensors.iter().any(|s| s.covers(pos)) && !sensors.iter().any(|s| s.beacon == pos) {
            count += 1;
        }
    }
    count
}
```

This would be much less efficient for large search areas, with a time complexity of O(x * n) where x is the width of the search area.

### Binary Search for Part 2

Another approach for Part 2 would be to use binary search to find the gap more efficiently:

```rust
fn find_gap(ranges: &[RangeInclusive<isize>], min: isize, max: isize) -> Option<isize> {
    // Binary search for a gap in the ranges
    // ...
}
```

This could potentially reduce the time complexity for finding the gap, but would be more complex to implement correctly.

### Geometric Approach

A more sophisticated approach would be to use computational geometry techniques:

```rust
fn find_distress_beacon(sensors: &[Sensor], bounds: (isize, isize)) -> Coord {
    // Find intersection points of sensor boundaries
    // Check positions just outside the boundary of each sensor
    // ...
}
```

This would be more efficient for very large search areas but would require more complex geometric calculations.

## Conclusion

This solution demonstrates an efficient approach to a problem that involves analyzing large ranges of positions. By working with ranges rather than individual positions, we can efficiently solve both parts of the problem. The range merging technique is particularly effective for Part 1, while the gap-finding approach allows us to solve Part 2 without exhaustively checking every position.