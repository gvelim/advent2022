# Day 6: Solution Explanation

## Approach

Day 6's problem involves finding the first occurrence of a sequence of unique characters in a datastream. The approach is to:

1. Process the input datastream as a sequence of bytes
2. Examine consecutive windows of characters (of length 4 for part 1, 14 for part 2)
3. Check each window for duplicate characters
4. Find the position of the first window that contains no duplicates

The solution uses Rust's trait system to create reusable functionality for checking duplicates and finding marker positions.

## Implementation Details

### Detecting Duplicates

The first key component is a trait for checking whether a slice contains any duplicate elements:

```rust
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

This implementation:
1. Creates a copy of the slice
2. Sorts the copy (bringing identical elements next to each other)
3. Checks adjacent pairs for equality using `windows(2)`

The trait is implemented generically for any slice type `[T]` where `T` supports debugging, copying, equality comparison, and ordering.

### Finding Marker Positions

The second key component is a trait for finding the position of a marker in a datastream:

```rust
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

This implementation:
1. Creates sliding windows of the specified length using `windows(len)`
2. Pairs each window with its index using `enumerate()`
3. Skips windows that contain duplicates using `skip_while`
4. Takes the first window that has no duplicates
5. Returns the position after this window (index + window length)

### Main Solution

With these traits defined, the main solution becomes remarkably simple:

```rust
fn main() {
    let data = std::fs::read_to_string("src/bin/day6_input.txt").expect("");

    let out = data.bytes().collect::<Vec<_>>();
    println!("Marker Length @4 = {}", out.marker_position(4));
    println!("Marker Length @14 = {}", out.marker_position(14));
}
```

The solution reads the input file, converts it to a vector of bytes, and then calls `marker_position` with the appropriate lengths for part 1 (4) and part 2 (14).

## Algorithm Analysis

### Time Complexity

The time complexity of this solution depends on the length of the input (`n`) and the marker length (`m`):

- Checking for duplicates in a window takes O(m log m) time due to the sorting operation
- In the worst case, we check every window in the input, giving us O(n) windows
- Overall time complexity: O(n * m log m)

For this problem, `m` is small (4 or 14), so the logarithmic factor isn't significant, making the effective complexity close to O(n).

### Space Complexity

The space complexity is O(n) to store the input as a vector of bytes, plus O(m) temporary storage for each duplicate check.

## Alternative Approaches

### Using a HashSet for Duplicate Detection

A common alternative approach would be to use a `HashSet` to check for duplicates:

```rust
fn has_unique_chars(window: &[u8]) -> bool {
    let mut set = HashSet::new();
    window.iter().all(|&c| set.insert(c))
}
```

This would have O(m) time complexity for checking duplicates instead of O(m log m), but at the cost of using `HashSet` which has more overhead than simple sorting for small datasets.

### Using Frequency Counting

Another approach would be to count the frequency of each character:

```rust
fn has_unique_chars(window: &[u8]) -> bool {
    let mut counts = [0; 256]; // For ASCII
    for &c in window {
        counts[c as usize] += 1;
        if counts[c as usize] > 1 {
            return false;
        }
    }
    true
}
```

This has O(m) time complexity and uses a fixed amount of space, but is limited to ASCII or other bounded character sets.

### Using a Bit Set

For even more efficiency, a bit set could be used for the specific case of lowercase ASCII characters:

```rust
fn has_unique_chars(window: &[u8]) -> bool {
    let mut bits = 0u32;
    for &c in window {
        let mask = 1 << (c - b'a');
        if (bits & mask) != 0 {
            return false;
        }
        bits |= mask;
    }
    true
}
```

This has O(m) time complexity and uses only a single integer for storage, but is limited to a single case of character set.

## Conclusion

The solution demonstrates the power of Rust's traits for creating reusable, generic functionality. By separating the concerns of duplicate detection and marker finding into traits, the code becomes more modular and expressive. The generic implementation allows the solution to work with any type of element, not just characters, making it more versatile than specialized approaches.