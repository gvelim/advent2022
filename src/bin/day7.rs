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
    fn children(&self, dir: &String) -> Vec<&String> {
        self.map[dir]
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
    fn dir_size(&self, dir: &String) -> usize {
        self.map[dir].size
    }
    fn totals(&self) -> Vec<(String,usize)> {
        self.totals.take()
    }
    fn parse_history(history: &Vec<LineType>) -> Tree {
        use LineType::*;

        let mut map = HashMap::<String,Node>::new();
        let mut dir: String = "".to_string();

        history.iter()
            // .inspect(|line| println!("{:?}",line))
            .for_each(|lt| {
                match lt {
                    Cmd(CommandType::Cd(d)) if d.contains("..") => dir = map[&dir].parent.clone(),
                    Cmd(CommandType::Cd(d)) => {
                        let id = format!("{}{}",dir,d);
                        println!("{id}");
                        map.entry(id.clone())
                            .or_insert(Node { parent: dir.clone(), content: Vec::new(), size: 0 });
                        dir = id;
                    }
                    Rst(res) => {
                        let node = map.get_mut(&dir).unwrap();
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
    fn calc_dirs_totals(&self, dir: &String) -> usize {
        let mut sum = self.dir_size(dir);
        for d in self.children(dir) {
            let key = format!("{}{}",dir,d);
            sum += self.calc_dirs_totals(&key);
        }
        println!("{:?}:{:?}",dir, sum);
        self.totals.borrow_mut().push((dir.clone(),sum));
        sum
    }
}


fn main() {
    let history = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    // let history = std::fs::read_to_string("src/bin/day7_input.txt").expect("");

    let out = history.lines()
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
        .inspect(|e| println!("{:?}",e))
        .collect::<Vec<_>>();

    let tree = Tree::parse_history(&out);
    println!("{:?}",tree);
    tree.calc_dirs_totals(&"/".to_string());
    println!("{:?}", tree.totals());
}