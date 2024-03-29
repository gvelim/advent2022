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
    fn iterator(history:&str) -> impl Iterator<Item=LineType> + '_{
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