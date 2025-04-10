# Day 7: Solution Explanation

## Approach

Day 7's problem involves building a directory tree and calculating directory sizes from terminal output. The solution breaks down into several key steps:

1. Parse the terminal output into commands and results
2. Build a directory tree structure based on the commands
3. Calculate the total size of each directory (including its subdirectories)
4. Find directories matching the specified size criteria

The solution uses a tree structure with nodes representing directories, where each node keeps track of its contents and size.

## Implementation Details

### Data Structures

The solution uses several custom types to represent the file system:

```rust
#[derive(Debug, Clone)]
enum ResultType {
    File(String, usize),  // File name and size
    Dir(String)           // Directory name
}

#[derive(Debug)]
enum CommandType {
    Cd(String),  // Change directory with target
    List          // List directory contents
}

#[derive(Debug)]
enum LineType {
    Cmd(CommandType),  // A command
    Rst(ResultType)    // Output from a command
}
```

These enums represent the different types of lines in the terminal output.

### Path Representation

A custom `Path` struct is used to represent file paths:

```rust
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Path(String);

impl Path {
    fn new(path:String) -> Path {
        Path(path)
    }
    fn append(&self, dir: &str) -> Path {
        Path(format!("{}{}",self.0,dir))
    }
}
```

This struct wraps a string and provides methods for creating and appending to paths. It's also made to be hashable so it can be used as a key in a `HashMap`.

### Directory Tree Structure

The directory tree is represented by two structures:

```rust
#[derive(Debug)]
struct Node {
    parent: Path,            // Parent directory path
    content: Vec<ResultType>, // Contents (files and subdirectories)
    size: usize              // Size of files directly in this directory
}

#[derive(Debug)]
struct Tree {
    map: HashMap<Path,Node>, // Maps paths to nodes
    totals: RefCell<Vec<(Path,usize)>> // Stores total sizes for each directory
}
```

The `Node` structure represents a directory with its parent, contents, and direct file size. The `Tree` structure contains a map from paths to nodes and a list of total sizes for all directories.

### Parsing the Terminal Output

The terminal output is parsed line by line using an iterator:

```rust
struct History();
impl History {
    fn iterator(history:&str) -> impl Iterator<Item=LineType> + '_ {
        history.lines()
            .filter_map(|e| {
                let p:Vec<_> = e.split(' ').collect();
                match p[0] {
                    "$" => match p[1] {
                        "ls" => Some(LineType::Cmd(CommandType::List)),
                        "cd" => Some(LineType::Cmd(CommandType::Cd(String::from(p[2])))),
                        _ => None
                    }
                    "dir" => Some(LineType::Rst(ResultType::Dir(p[1].to_string()))),
                    _ => Some(LineType::Rst(ResultType::File(p[1].to_string(), usize::from_str(p[0]).unwrap())))
                }
            })
    }
}
```

This iterator converts each line into a `LineType` (either a command or a result) based on the line format.

### Building the Directory Tree

The directory tree is built by processing each command and its results:

```rust
fn parse_history(history: impl Iterator<Item=LineType>) -> Tree {
    use LineType::*;

    let mut map = HashMap::<Path,Node>::new();
    let mut path = Path::new("".to_string());

    history
        .for_each(|lt| {
            match lt {
                Cmd(CommandType::Cd(dir)) if dir.contains("..") => path = map[&path].parent.clone(),
                Cmd(CommandType::Cd(dir)) => {
                    let cpath = path.append(dir.as_str());
                    map.entry(cpath.clone())
                        .or_insert(Node { parent: path.clone(), content: Vec::new(), size: 0 });
                    path = cpath;
                }
                Rst(res) => {
                    let node = map.get_mut(&path).unwrap();
                    node.content.push(res.clone());
                    if let ResultType::File(_,fsize) = res {
                        node.size += fsize;
                    }
                }
                Cmd(CommandType::List) => {},
            }
        });
    Tree { map, totals: RefCell::new(Vec::new()) }
}
```

As commands are processed, a current path is maintained, and nodes are added to the tree as needed. When file results are encountered, they're added to the current directory's contents and their sizes are added to the directory's direct size.

### Calculating Directory Sizes

To calculate the total size of each directory (including subdirectories), a recursive function is used:

```rust
fn calc_dirs_totals(&self, path: &Path) -> usize {
    let mut sum = self.dir_size(path);
    for dir in self.children(path) {
        let cpath = path.append(dir);
        sum += self.calc_dirs_totals(&cpath);
    }
    self.totals.borrow_mut().push((path.clone(), sum));
    sum
}
```

This function calculates the total size of a directory by adding its direct size to the total sizes of its subdirectories. It also stores the total size in the `totals` list.

### Solving Part 1

For Part 1, the solution finds all directories with a total size of at most 100,000 and sums their sizes:

```rust
dirs.iter()
    .filter(|(_,size)| *size < 100000 )
    .map(|&(_,size)| size)
    .sum::<usize>()
```

### Solving Part 2

For Part 2, the solution finds the smallest directory that, when deleted, would free enough space:

```rust
let total_space = 70000000;
let min_free_space = 30000000;
let &(_,total_used) = dirs.last().unwrap();
let min_space_to_free = min_free_space - (total_space - total_used);

dirs.iter()
    .filter(|(_,size)| *size >= min_space_to_free )
    .min_by(|&a,&b| a.1.cmp(&b.1))
```

It calculates the minimum amount of space that needs to be freed, then finds the smallest directory that is at least that size.

## Algorithmic Analysis

### Time Complexity

- Parsing the terminal output: O(n), where n is the number of lines
- Building the directory tree: O(n)
- Calculating directory sizes: O(d), where d is the number of directories
- Finding directories by size: O(d)

Overall time complexity: O(n + d), which simplifies to O(n) since d ≤ n

### Space Complexity

- Directory tree: O(n) to store all files and directories
- Totals list: O(d) to store the size of each directory

Overall space complexity: O(n)

## Alternative Approaches

### Using a Real File System Library

Instead of implementing a custom file system representation, we could use a file system library that supports virtual file systems:

```rust
use std::path::PathBuf;
use memfs::MemFs;

let fs = MemFs::new();

// Process commands and build the file system
for line in input.lines() {
    // Parse and execute commands...
}

// Calculate directory sizes
fn dir_size(fs: &MemFs, path: &PathBuf) -> usize {
    // Calculate size recursively
}
```

This would leverage existing file system implementations but might be more complex to set up.

### Using a Graph Library

Another approach would be to use a graph library to represent the directory structure:

```rust
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::DfsPostOrder;

let mut graph = DiGraph::new();
let mut node_map = HashMap::new();

// Build the graph
// ...

// Calculate sizes with a post-order traversal
let mut dfs = DfsPostOrder::new(&graph, root);
while let Some(node) = dfs.next(&graph) {
    // Calculate size based on children's sizes
}
```

This would use well-tested graph algorithms but adds an external dependency.

## Conclusion

This solution demonstrates how to parse structured text and build a tree representation of a file system. The use of custom types like `Path`, `Node`, and `Tree` makes the code expressive and organized. The recursive calculation of directory sizes is a natural fit for the hierarchical nature of the file system.