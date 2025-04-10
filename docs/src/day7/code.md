# Day 7: Code

Below is the complete code for Day 7's solution, which parses terminal output to build a directory tree and analyze directory sizes.

## Full Solution

```advent2022/src/bin/day7.rs#L1-138
use std::cell::RefCell;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum ResultType {
    File(String, usize),
    Dir(String)
}
#[derive(Debug)]
enum CommandType {
    Cd(String),
    List
}
#[derive(Debug)]
enum LineType {
    Cmd(CommandType),
    Rst(ResultType)
}
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
#[derive(Debug)]
struct Node {
    parent: Path,
    content: Vec<ResultType>,
    size: usize
}
#[derive(Debug)]
struct Tree {
    map: HashMap<Path,Node>,
    totals: RefCell<Vec<(Path,usize)>>
}
impl Tree {
    fn children(&self, path: &Path) -> Vec<&String> {
        self.map[path]
            .content
            .iter()
            .filter_map(|rt|
                if let ResultType::Dir(dir) = rt {
                    Some(dir)
                } else {
                    None
                }
            )
            .collect()
    }
    fn dir_size(&self, path: &Path) -> usize {
        self.map[path].size
    }
    fn totals(&self) -> Vec<(Path, usize)> {
        self.totals.take()
    }
    fn parse_history(history: impl Iterator<Item=LineType>) -> Tree {
        use LineType::*;

        let mut map = HashMap::<Path,Node>::new();
        let mut path = Path::new("".to_string());

        history
            // .inspect(|line| println!("{:?}",line))
            .for_each(|lt| {
                match lt {
                    Cmd(CommandType::Cd(dir)) if dir.contains("..") => path = map[&path].parent.clone(),
                    Cmd(CommandType::Cd(dir)) => {
                        let cpath = path.append(dir.as_str());
                        println!("{:?}",cpath);
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
    fn calc_dirs_totals(&self, path: &Path) -> usize {
        let mut sum = self.dir_size(path);
        for dir in self.children(path) {
            let cpath = path.append(dir);
            sum += self.calc_dirs_totals(&cpath);
        }
        // println!("{:?}:{:?}", path, sum);
        self.totals.borrow_mut().push((path.clone(), sum));
        sum
    }
}

struct History();
impl History {
    fn iterator(history:&str) -> impl Iterator<Item=LineType> + '_{{
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

fn main() {

    let history = std::fs::read_to_string("src/bin/day7_input.txt").expect("");

    let tree = Tree::parse_history(
        History::iterator(history.as_str())
    );

    tree.calc_dirs_totals(&Path::new("/".to_string()));
    let dirs = tree.totals();

    println!("Directories < 100000 \n====================");
    println!("{:?}",
             dirs.iter()
                 .filter(|(_,size)| *size < 100000 )
                 .inspect(|&p| println!("{:?}",p))
                 .map(|&(_,size)| size)
                 .sum::<usize>()
    );

    let total_space = 70000000;
    let min_free_space = 30000000;
    let &(_,total_used) = dirs.last().unwrap();
    let min_space_to_free = min_free_space - (total_space - total_used);
    println!("Directories ~ 30000000 \n====================");
    println!("{:?}",
             dirs.iter()
                 .filter(|(_,size)| *size >= min_space_to_free )
                 .inspect(|&p| println!("{:?}",p))
                 .min_by(|&a,&b| a.1.cmp(&b.1))
    );
}
```

## Code Walkthrough

### Data Types

The solution defines several types to represent the file system and terminal output:

```advent2022/src/bin/day7.rs#L5-19
#[derive(Debug, Clone)]
enum ResultType {
    File(String, usize),
    Dir(String)
}
#[derive(Debug)]
enum CommandType {
    Cd(String),
    List
}
#[derive(Debug)]
enum LineType {
    Cmd(CommandType),
    Rst(ResultType)
}
```

These enums represent:
- `ResultType`: Either a file (with name and size) or a directory (with name)
- `CommandType`: Either a change directory command or a list command
- `LineType`: Either a command or a result

### Path Representation

```advent2022/src/bin/day7.rs#L20-27
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

The `Path` struct encapsulates a string representing a file path and provides methods to create and append to paths.

### Directory Tree

```advent2022/src/bin/day7.rs#L28-37
#[derive(Debug)]
struct Node {
    parent: Path,
    content: Vec<ResultType>,
    size: usize
}
#[derive(Debug)]
struct Tree {
    map: HashMap<Path,Node>,
    totals: RefCell<Vec<(Path,usize)>>
}
```

The directory tree consists of:
- `Node`: Represents a directory with its parent, contents, and direct size
- `Tree`: Contains a map of paths to nodes and a list of total sizes

### Directory Tree Methods

```advent2022/src/bin/day7.rs#L38-53
fn children(&self, path: &Path) -> Vec<&String> {
    self.map[path]
        .content
        .iter()
        .filter_map(|rt|
            if let ResultType::Dir(dir) = rt {
                Some(dir)
            } else {
                None
            }
        )
        .collect()
}
fn dir_size(&self, path: &Path) -> usize {
    self.map[path].size
}
fn totals(&self) -> Vec<(Path, usize)> {
    self.totals.take()
}
```

These methods provide functionality to:
- Get a list of child directories
- Get the direct size of a directory
- Take the list of total sizes

### Parsing Terminal Output

```advent2022/src/bin/day7.rs#L54-84
fn parse_history(history: impl Iterator<Item=LineType>) -> Tree {
    use LineType::*;

    let mut map = HashMap::<Path,Node>::new();
    let mut path = Path::new("".to_string());

    history
        // .inspect(|line| println!("{:?}",line))
        .for_each(|lt| {
            match lt {
                Cmd(CommandType::Cd(dir)) if dir.contains("..") => path = map[&path].parent.clone(),
                Cmd(CommandType::Cd(dir)) => {
                    let cpath = path.append(dir.as_str());
                    println!("{:?}",cpath);
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

This method builds a directory tree by processing terminal commands:
- For `cd ..` commands, it moves up to the parent directory
- For other `cd` commands, it creates a new directory if needed and moves into it
- For result lines, it adds files or directories to the current directory's contents

### Calculating Total Sizes

```advent2022/src/bin/day7.rs#L85-94
fn calc_dirs_totals(&self, path: &Path) -> usize {
    let mut sum = self.dir_size(path);
    for dir in self.children(path) {
        let cpath = path.append(dir);
        sum += self.calc_dirs_totals(&cpath);
    }
    // println!("{:?}:{:?}", path, sum);
    self.totals.borrow_mut().push((path.clone(), sum));
    sum
}
```

This recursive method calculates the total size of each directory by adding its direct size to the total sizes of its subdirectories.

### Creating the Line Iterator

```advent2022/src/bin/day7.rs#L97-111
struct History();
impl History {
    fn iterator(history:&str) -> impl Iterator<Item=LineType> + '_{{
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

This creates an iterator that converts terminal output lines into `LineType` values by parsing each line based on its format.

### Main Function

```advent2022/src/bin/day7.rs#L115-138
let history = std::fs::read_to_string("src/bin/day7_input.txt").expect("");

let tree = Tree::parse_history(
    History::iterator(history.as_str())
);

tree.calc_dirs_totals(&Path::new("/".to_string()));
let dirs = tree.totals();

// Part 1: Find directories smaller than 100,000
println!("Directories < 100000 \n====================");
println!("{:?}",
         dirs.iter()
             .filter(|(_,size)| *size < 100000 )
             .inspect(|&p| println!("{:?}",p))
             .map(|&(_,size)| size)
             .sum::<usize>()
);

// Part 2: Find smallest directory to delete
let total_space = 70000000;
let min_free_space = 30000000;
let &(_,total_used) = dirs.last().unwrap();
let min_space_to_free = min_free_space - (total_space - total_used);
println!("Directories ~ 30000000 \n====================");
println!("{:?}",
         dirs.iter()
             .filter(|(_,size)| *size >= min_space_to_free )
             .inspect(|&p| println!("{:?}",p))
             .min_by(|&a,&b| a.1.cmp(&b.1))
);
```

The main function:
1. Reads the terminal output from a file
2. Creates an iterator to parse the output
3. Builds a directory tree using the parsed commands
4. Calculates the total size of each directory
5. For Part 1: Finds directories smaller than 100,000 and sums their sizes
6. For Part 2: Finds the smallest directory that would free enough space when deleted

## Implementation Notes

- **RefCell Usage**: The solution uses a `RefCell` to store the list of total sizes, allowing it to be modified during the recursive calculation
- **Path Representation**: Paths are represented as strings for simplicity, but with a custom wrapper type for safety
- **Tree Structure**: The directory tree uses a map-based representation with explicit parent references, making it easy to navigate up and down the tree
- **Functional Approach**: The solution makes extensive use of iterators and functional programming patterns