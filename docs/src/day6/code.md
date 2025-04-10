# Day 6: Code

Below is the complete code for Day 6's solution, which finds marker patterns in a datastream.

## Full Solution

```advent2022/src/bin/day6.rs#L1-28
use std::fmt::Debug;

trait Duplicate {
    fn has_duplicates(&self) -> bool;
}
impl<T> Duplicate for [T] where T: Debug + Copy + PartialEq + Ord {
    fn has_duplicates(&self) -> bool {
        let mut tmp = self.to_vec();
        tmp.sort();
        tmp.windows(2).any(|a| a[0]==a[1])
    }
}

trait Signaling {
    fn marker_position(&self, len:usize) -> usize;
}
impl<T> Signaling for [T] where T : Debug + Copy + PartialEq + Ord {
    fn marker_position(&self, len: usize) -> usize {
        self.windows(len)
            .enumerate()
            .skip_while(|&(_,stm)| stm.has_duplicates() )
            .next()
            .map(|(i,_)| i + len)
            .unwrap_or_else(|| panic!("marker_position(): Ops!"))
    }
}

fn main() {
    let data = std::fs::read_to_string("src/bin/day6_input.txt").expect("");

    let out = data.bytes().collect::<Vec<_>>();
    println!("Marker Length @4 = {}", out.marker_position(4));
    println!("Marker Length @14 = {}", out.marker_position(14));
}
```

## Code Walkthrough

### Duplicate Detection Trait

```advent2022/src/bin/day6.rs#L3-11
trait Duplicate {
    fn has_duplicates(&self) -> bool;
}
impl<T> Duplicate for [T] where T: Debug + Copy + PartialEq + Ord {
    fn has_duplicates(&self) -> bool {
        let mut tmp = self.to_vec();
        tmp.sort();
        tmp.windows(2).any(|a| a[0]==a[1])
    }
}
```

This trait provides a method to check if a slice contains duplicate elements:

1. `Duplicate` trait defines a single method `has_duplicates` that returns a boolean
2. The implementation for slices `[T]` works with any type that can be debugged, copied, compared for equality, and ordered
3. The implementation creates a temporary copy of the slice, sorts it (bringing identical elements adjacent to each other), and then checks if any adjacent elements are equal
4. The `windows(2)` method creates sliding windows of 2 elements, and `any` checks if the predicate is true for any window

### Marker Detection Trait

```advent2022/src/bin/day6.rs#L13-23
trait Signaling {
    fn marker_position(&self, len:usize) -> usize;
}
impl<T> Signaling for [T] where T : Debug + Copy + PartialEq + Ord {
    fn marker_position(&self, len: usize) -> usize {
        self.windows(len)
            .enumerate()
            .skip_while(|&(_,stm)| stm.has_duplicates() )
            .next()
            .map(|(i,_)| i + len)
            .unwrap_or_else(|| panic!("marker_position(): Ops!"))
    }
}
```

This trait provides a method to find the position of the first marker of a specified length:

1. `Signaling` trait defines a single method `marker_position` that takes a length parameter and returns a position
2. The implementation creates sliding windows of the specified length using `windows(len)`
3. Each window is paired with its position using `enumerate()`
4. Windows containing duplicates are skipped using `skip_while`
5. The first window without duplicates is selected with `next()`
6. The marker position is calculated as the window index plus the window length

### Main Function

```advent2022/src/bin/day6.rs#L25-28
fn main() {
    let data = std::fs::read_to_string("src/bin/day6_input.txt").expect("");

    let out = data.bytes().collect::<Vec<_>>();
    println!("Marker Length @4 = {}", out.marker_position(4));
    println!("Marker Length @14 = {}", out.marker_position(14));
}
```

The main function:

1. Reads the input file into a string
2. Converts the string to a vector of bytes using `bytes().collect()`
3. Calls `marker_position(4)` to solve Part 1 (finding a start-of-packet marker)
4. Calls `marker_position(14)` to solve Part 2 (finding a start-of-message marker)

## Implementation Notes

- **Traits for Reusability**: The solution uses Rust's trait system to create reusable behaviors
- **Generic Implementation**: Both traits work with any type that meets the trait bounds, not just bytes or characters
- **Functional Approach**: The code uses a functional programming style with method chaining for concise and expressive code
- **Algorithm Choice**: The solution uses sorting for duplicate detection, which is efficient for small windows (like the 4 and 14 character windows in this problem)

The implementation is elegant and leverages Rust's powerful type system to create a generic, reusable solution that can handle both parts of the problem with the same code.