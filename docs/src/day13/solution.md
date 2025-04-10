# Day 13: Solution Explanation

## Approach

Day 13 involves parsing and comparing nested lists according to specific rules. The solution breaks down into three main components:

1. **Parsing the nested list structure**: We need to parse strings like `[1,[2,3],4]` into a structured representation
2. **Implementing the comparison logic**: We need to define how to compare two list structures following the given rules
3. **Processing the input data**: We need to handle the pairs of packets for Part 1 and sort all packets for Part 2

The solution uses a recursive approach for parsing and a structured type system with trait implementations for comparison.

## Implementation Details

### Data Structure

First, we define a data structure to represent the packet data, which can be either a number or a list of items:

```rust
enum ListItem {
    N(u8),       // A number
    L(Vec<ListItem>)  // A list
}
```

This recursive enum allows representing any nested list structure. We use `N` for numbers and `L` for lists.

### Parsing

The solution uses a custom parser implemented with the `FromStr` trait to convert string input into `ListItem` structures:

```rust
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

This parsing logic works by:
1. Creating a `Scanner` that processes characters from a peekable iterator
2. Implementing a recursive `parse_list` method that handles nested lists
3. Processing each character based on whether it's an opening bracket, digit, comma, or closing bracket
4. Building up the nested `ListItem` structure as it parses

The parser handles the specific format of the packets as described in the problem.

### Comparison Logic

The core of the solution is implementing the comparison logic between `ListItem` values. This is done by implementing the `Ord` trait:

```rust
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

This implementation follows the rules specified in the problem:

1. **For two lists**: Compare items one by one until a difference is found or one list runs out of items
2. **For two integers**: Compare them directly
3. **For a list and an integer**: Convert the integer to a single-item list and compare

The `PartialOrd` trait is also implemented to support comparison operators:

```rust
impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```

And for completeness, the `PartialEq` and `Eq` traits are implemented using the comparison logic:

```rust
impl PartialEq<Self> for ListItem {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl Eq for ListItem {}
```

### Processing Pairs (Part 1)

For Part 1, we need to find the pairs of packets that are in the right order (left < right) and sum their indices:

```rust
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

This function:
1. Splits the input by double newlines to get pairs of packets
2. Parses each packet into a `ListItem`
3. Compares each pair using the `lt` method (less than)
4. Keeps track of indices (1-based) for pairs in the right order
5. Sums the indices

### Sorting Packets (Part 2)

For Part 2, we need to sort all packets, including two divider packets, and find the product of the indices of the divider packets:

```rust
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

This function:
1. Creates the two divider packets (`[[2]]` and `[[6]]`)
2. Parses all packets from the input, ignoring blank lines
3. Adds the divider packets to the list
4. Sorts all packets using the implemented comparison logic
5. Finds the indices of the divider packets (1-based)
6. Multiplies the indices to get the decoder key

## Algorithmic Analysis

### Time Complexity

- **Parsing**: O(n) for each packet, where n is the length of the packet string
- **Comparison**: O(n) for two packets of total size n
- **Part 1**: O(p × n) where p is the number of pairs and n is the average packet size
- **Part 2**: O(p × n × log(p)) due to the sorting operation

### Space Complexity

- O(n) to store the parsed packet structures
- O(p) for the list of all packets in Part 2

## Alternative Approaches

### Using JSON Parsing

Since the packet format is essentially JSON, we could use a JSON parsing library:

```rust
use serde_json::Value;

fn compare_values(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Array(l), Value::Array(r)) => {
            // Compare arrays
            // ...
        },
        (Value::Number(l), Value::Number(r)) => {
            // Compare numbers
            // ...
        },
        (Value::Array(_), Value::Number(_)) => {
            // Convert number to array
            // ...
        },
        (Value::Number(_), Value::Array(_)) => {
            // Convert number to array
            // ...
        },
        _ => unreachable!()
    }
}
```

This approach would rely on an external library but could be more robust for complex inputs.

### Recursive Descent Parser

Another approach would be to use a more structured recursive descent parser:

```rust
fn parse_packet(s: &str) -> ListItem {
    let mut chars = s.chars().peekable();
    parse_list(&mut chars)
}

fn parse_list(chars: &mut Peekable<Chars>) -> ListItem {
    // Expect opening bracket
    assert_eq!(chars.next().unwrap(), '[');
    
    let mut list = vec![];
    
    // Parse items until closing bracket
    while chars.peek() != Some(&']') {
        if chars.peek() == Some(&'[') {
            list.push(parse_list(chars));
        } else {
            list.push(parse_number(chars));
        }
        
        // Skip comma if present
        if chars.peek() == Some(&',') {
            chars.next();
        }
    }
    
    // Skip closing bracket
    chars.next();
    
    L(list)
}

fn parse_number(chars: &mut Peekable<Chars>) -> ListItem {
    // Parse digits into a number
    // ...
}
```

This would be more structured but essentially accomplish the same thing as the current scanner approach.

## Conclusion

This solution demonstrates how to parse and compare nested data structures according to complex rules. The use of enums and trait implementations creates a clean, type-safe solution that directly models the problem domain. The comparison logic is implemented recursively to handle the nested nature of the data, and the solution efficiently processes both parts of the problem.