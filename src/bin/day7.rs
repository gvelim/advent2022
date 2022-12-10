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
#[derive(Debug)]
struct Node {
    parent: String,
    content: Vec<ResultType>,
    size: usize
}
#[derive(Debug)]
struct Tree {
    map: HashMap<String,Node>,
    totals: RefCell<Vec<(String,usize)>>
}
impl Tree {
    fn children(&self, path: &String) -> Vec<&String> {
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
    fn dir_size(&self, path: &String) -> usize {
        self.map[path].size
    }
    fn totals(&self) -> Vec<(String,usize)> {
        self.totals.take()
    }
    fn parse_history(history: &Vec<LineType>) -> Tree {
        use LineType::*;

        let mut map = HashMap::<String,Node>::new();
        let mut path: String = "".to_string();

        history.iter()
            // .inspect(|line| println!("{:?}",line))
            .for_each(|lt| {
                match lt {
                    Cmd(CommandType::Cd(dir)) if dir.contains("..") => path = map[&path].parent.clone(),
                    Cmd(CommandType::Cd(dir)) => {
                        let cpath = format!("{}{}", path, dir);
                        // println!("{cpath}");
                        map.entry(cpath.clone())
                            .or_insert(Node { parent: path.clone(), content: Vec::new(), size: 0 });
                        path = cpath;
                    }
                    Rst(res) => {
                        let node = map.get_mut(&path).unwrap();
                        node.content.push(res.clone());
                        if let &ResultType::File(_,fsize) = res {
                            node.size += fsize;
                        }
                    }
                    Cmd(CommandType::List) => {},
                }
            });
        Tree { map, totals: RefCell::new(Vec::new()) }
    }
    fn calc_dirs_totals(&self, path: &String) -> usize {
        let mut sum = self.dir_size(path);
        for dir in self.children(path) {
            let cpath = format!("{}{}", path, dir);
            sum += self.calc_dirs_totals(&cpath);
        }
        // println!("{:?}:{:?}", path, sum);
        self.totals.borrow_mut().push((path.clone(), sum));
        sum
    }
}

struct History();
impl History {
    fn parse(history:&str) -> Vec<LineType> {
        history.lines()
            .filter_map(|e| {
                let p:Vec<_> = e.split(" ").collect();
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
            // .inspect(|e| println!("{:?}",e))
            .collect::<Vec<_>>()
    }
}

fn main() {

    let history = std::fs::read_to_string("src/bin/day7_input.txt").expect("");

    let hst_lines = History::parse(history.as_str());

    let tree = Tree::parse_history(&hst_lines);

    tree.calc_dirs_totals(&"/".to_string());

    println!("Directories < 100000 \n====================");
    println!("{:?}",
             tree.totals()
                 .iter()
                 .filter(|(_,size)| *size < 100000 )
                 .inspect(|&p| println!("{:?}",p))
                 .map(|&(_,size)| size)
                 .sum::<usize>());
}