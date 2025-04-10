# Day 13: Code

Below is the complete code for Day 13's solution, which parses and compares nested lists according to specific rules.

## Full Solution

```advent2022/src/bin/day13.rs#L1-177
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::iter::Peekable;
use std::str::FromStr;
use crate::ListItem::{L, N};

fn packets_in_right_order(input: &str) -> usize {
    input.split("\n\n")
        .map(|x| x.lines().collect::<Vec<_>>() )
        .map(|d|
            (ListItem::from_str(d[0]), ListItem::from_str(d[1]))
        )
        .enumerate()
        .filter_map(|(i,(l,r))|
            if l.lt(&r) { Some(i+1) } else { None }
        )
        .sum()
}

fn get_decoder_key(input: &str) -> usize {

    let dividers = vec![
        L(vec![L(vec![N(2)])]),
        L(vec![L(vec![N(6)])])
    ];

    let mut order = input.split("\n\n")
        .flat_map(|x| x.lines() )
        .filter_map(|d|
            ListItem::from_str(d).ok()
        )
        .chain(vec![ L(vec![L(vec![N(2)])]), L(vec![L(vec![N(6)])]) ] )
        .fold(vec![], |mut out, item|{
            out.push(item);
            out
        });

    order.sort();
    order.iter().for_each(|d| println!("{:?}",d));

    dividers.iter()
        .map(|d| order.binary_search(d).unwrap() + 1 )
        .product()
}

fn main() {
    // let mut input = "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\
    // [7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string();

    let input = std::fs::read_to_string("src/bin/day13_input.txt").expect("Ops!");

    let res = packets_in_right_order(input.as_str());
    println!("Correctly ordered packets = {:?}",res);
    let res = get_decoder_key(input.as_str());
    println!("Decoder Key = {:?}",res);

}

enum ListItem {
    N(u8),
    L(Vec<ListItem>)
}
impl ListItem {
    fn insert(&mut self, item:ListItem) {
        match (self,item) {
            (L(list), item) => list.push(item),
            (N(old), N(new)) => *old = new,
            (_,_) => unreachable!()
        }
    }
}

impl FromStr for ListItem {
    type Err = ();

    fn from_str(inp: &str) -> Result<Self, Self::Err> {

        struct Scanner<I: Iterator<Item=char>> {
            i: Peekable<I>,
        }
        impl<I: Iterator<Item=char>> Scanner<I> {
            fn new(s: I) -> Self {
                Scanner { i: s.peekable() }
            }
            fn parse_list(&mut self) -> ListItem {
                let mut s = String::new();
                let mut v = L(vec![]);
                loop {
                    match &self.i.peek() {
                        Some('[') => {
                            self.i.next();
                            v.insert(self.parse_list());
                        },
                        Some(&c@ '0'..='9') => s.push(c),
                        &c@
                        (Some(',') | Some(']')) if !s.is_empty() => {
                            v.insert(N(u8::from_str(s.as_str()).expect("")));
                            s.clear();
                            if ']'.eq(c.unwrap()) {
                                break v
                            }
                        },
                        Some(',') => {},
                        Some(']') => break v,
                        None => break v,
                        _ => unreachable!()
                    }
                    self.i.next();
                }
            }
        }
        let mut i = inp.chars().peekable();
        i.next();
        Ok(Scanner::new(i).parse_list())
    }
}

impl PartialEq<Self> for ListItem {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl Eq for ListItem {}

impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self,other) {
            (L(l), L(r)) => {
                let mut liter = l.iter();
                let mut riter = r.iter();

                loop {
                    match (liter.next(),riter.next()) {
                        (Some(l), Some(r)) =>
                            match l.cmp(r) {
                                Ordering::Equal => {},
                                ord@
                                (Ordering::Less | Ordering::Greater) => break ord,
                            },
                        (Some(_), None) => break Ordering::Greater,
                        (None, Some(_)) => break Ordering::Less,
                        (None,None) => break Ordering::Equal,
                    };
                }
            }
            (L(_), N(r)) => {
                let right = L(vec![N(*r)]);
                self.cmp(&right)
            }
            (N(l), L(_)) => {
                let left = L(vec![N(*l)]);
                left.cmp(other)
            }
            (N(l), N(r)) => l.cmp(r),
        }

    }
}

impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Debug for ListItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            N(n) => write!(f,"{n}")?,
            L(v) => f.debug_list().entries(v.iter()).finish()?
        };
        Ok(())
    }
}
```

## Code Walkthrough

### Data Structure for Packets

```advent2022/src/bin/day13.rs#L59-63
enum ListItem {
    N(u8),
    L(Vec<ListItem>)
}
```

The solution uses an enum `ListItem` to represent the nested list structure of packets:
- `N(u8)` represents a number (limited to u8 for this problem)
- `L(Vec<ListItem>)` represents a list containing other items (which can be numbers or lists)

This recursive structure can represent any valid packet in the problem.

### Parsing Packets

```advent2022/src/bin/day13.rs#L73-117
impl FromStr for ListItem {
    type Err = ();

    fn from_str(inp: &str) -> Result<Self, Self::Err> {

        struct Scanner<I: Iterator<Item=char>> {
            i: Peekable<I>,
        }
        impl<I: Iterator<Item=char>> Scanner<I> {
            fn new(s: I) -> Self {
                Scanner { i: s.peekable() }
            }
            fn parse_list(&mut self) -> ListItem {
                let mut s = String::new();
                let mut v = L(vec![]);
                loop {
                    match &self.i.peek() {
                        Some('[') => {
                            self.i.next();
                            v.insert(self.parse_list());
                        },
                        Some(&c@ '0'..='9') => s.push(c),
                        &c@
                        (Some(',') | Some(']')) if !s.is_empty() => {
                            v.insert(N(u8::from_str(s.as_str()).expect("")));
                            s.clear();
                            if ']'.eq(c.unwrap()) {
                                break v
                            }
                        },
                        Some(',') => {},
                        Some(']') => break v,
                        None => break v,
                        _ => unreachable!()
                    }
                    self.i.next();
                }
            }
        }
        let mut i = inp.chars().peekable();
        i.next();
        Ok(Scanner::new(i).parse_list())
    }
}
```

The `FromStr` implementation uses a custom scanner to parse the input string into a `ListItem`:

1. It creates a `Scanner` with a peekable iterator over the input characters
2. The `parse_list` method recursively builds the list structure by:
   - Creating a new list when encountering `[`
   - Accumulating digits for numbers
   - Inserting numbers when reaching a comma or closing bracket
   - Breaking when reaching the end of the list
3. The method returns the parsed `ListItem`

### Item Insertion Helper

```advent2022/src/bin/day13.rs#L64-72
impl ListItem {
    fn insert(&mut self, item:ListItem) {
        match (self,item) {
            (L(list), item) => list.push(item),
            (N(old), N(new)) => *old = new,
            (_,_) => unreachable!()
        }
    }
}
```

This helper method adds an item to a list or updates a number.

### Comparison Logic

```advent2022/src/bin/day13.rs#L125-159
impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self,other) {
            (L(l), L(r)) => {
                let mut liter = l.iter();
                let mut riter = r.iter();

                loop {
                    match (liter.next(),riter.next()) {
                        (Some(l), Some(r)) =>
                            match l.cmp(r) {
                                Ordering::Equal => {},
                                ord@
                                (Ordering::Less | Ordering::Greater) => break ord,
                            },
                        (Some(_), None) => break Ordering::Greater,
                        (None, Some(_)) => break Ordering::Less,
                        (None,None) => break Ordering::Equal,
                    };
                }
            }
            (L(_), N(r)) => {
                let right = L(vec![N(*r)]);
                self.cmp(&right)
            }
            (N(l), L(_)) => {
                let left = L(vec![N(*l)]);
                left.cmp(other)
            }
            (N(l), N(r)) => l.cmp(r),
        }
    }
}
```

The `Ord` implementation defines how to compare two `ListItem` values:

1. **List vs. List**: Compare elements one by one until finding a difference or reaching the end of a list
2. **List vs. Number**: Convert the number to a single-item list and retry comparison
3. **Number vs. List**: Convert the number to a single-item list and retry comparison
4. **Number vs. Number**: Use the built-in number comparison

This implements the comparison rules specified in the problem.

### Additional Trait Implementations

```advent2022/src/bin/day13.rs#L119-123
impl PartialEq<Self> for ListItem {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl Eq for ListItem {}
```

```advent2022/src/bin/day13.rs#L161-165
impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```

These implementations ensure that `ListItem` supports all the comparison operators and can be used in sorting operations.

### Debug Display

```advent2022/src/bin/day13.rs#L167-175
impl Debug for ListItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            N(n) => write!(f,"{n}")?,
            L(v) => f.debug_list().entries(v.iter()).finish()?
        };
        Ok(())
    }
}
```

This implementation formats `ListItem` values for debugging, using Rust's `debug_list` for nice formatting of lists.

### Part 1: Finding Correctly Ordered Pairs

```advent2022/src/bin/day13.rs#L7-17
fn packets_in_right_order(input: &str) -> usize {
    input.split("\n\n")
        .map(|x| x.lines().collect::<Vec<_>>() )
        .map(|d|
            (ListItem::from_str(d[0]), ListItem::from_str(d[1]))
        )
        .enumerate()
        .filter_map(|(i,(l,r))|
            if l.lt(&r) { Some(i+1) } else { None }
        )
        .sum()
}
```

This function processes the input for Part 1:
1. Splits the input by double newlines to get pairs of packets
2. Parses each packet into a `ListItem`
3. Uses the `lt` comparison to check if pairs are in the right order
4. Keeps 1-based indices of correctly ordered pairs
5. Sums these indices

### Part 2: Sorting and Finding Divider Packets

```advent2022/src/bin/day13.rs#L19-41
fn get_decoder_key(input: &str) -> usize {

    let dividers = vec![
        L(vec![L(vec![N(2)])]),
        L(vec![L(vec![N(6)])])
    ];

    let mut order = input.split("\n\n")
        .flat_map(|x| x.lines() )
        .filter_map(|d|
            ListItem::from_str(d).ok()
        )
        .chain(vec![ L(vec![L(vec![N(2)])]), L(vec![L(vec![N(6)])]) ] )
        .fold(vec![], |mut out, item|{
            out.push(item);
            out
        });

    order.sort();
    order.iter().for_each(|d| println!("{:?}",d));

    dividers.iter()
        .map(|d| order.binary_search(d).unwrap() + 1 )
        .product()
}
```

This function processes the input for Part 2:
1. Defines the two divider packets (`[[2]]` and `[[6]]`)
2. Parses all packets from the input and adds the divider packets
3. Sorts all packets using the comparison logic
4. Finds the 1-based indices of the divider packets
5. Multiplies these indices to get the decoder key

### Main Function

```advent2022/src/bin/day13.rs#L43-55
fn main() {
    // Commented-out test input

    let input = std::fs::read_to_string("src/bin/day13_input.txt").expect("Ops!");

    let res = packets_in_right_order(input.as_str());
    println!("Correctly ordered packets = {:?}",res);
    let res = get_decoder_key(input.as_str());
    println!("Decoder Key = {:?}",res);
}
```

The main function reads the input file and runs both parts of the problem.

## Implementation Notes

- **Recursive Data Structure**: The solution uses a recursive enum to represent the nested packet structure
- **Custom Parser**: The parser handles the specific format of the input without relying on external libraries
- **Trait Implementations**: The comparison logic is cleanly implemented using Rust's trait system
- **Functional Style**: The solution uses a functional programming style with iterators and method chaining