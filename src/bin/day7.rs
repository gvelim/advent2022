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
    map: HashMap<String,Node>
}
impl Tree {
    fn parse_history(history: &Vec<LineType>) -> Tree {
        use LineType::*;

        let mut map = HashMap::<String,Node>::new();
        let mut dir: String = "/".to_string();

        history.iter()
            .inspect(|line| println!("{:?}",line))
            .for_each(|lt| {
                match lt {
                    Cmd(CommandType::Cd(d)) if d.contains("..") => dir = map[&dir].parent.clone(),
                    Cmd(CommandType::Cd(d)) => {
                            map.entry(d.clone()).or_insert(Node { parent: dir.clone(), content: Vec::new(), size: 0 });
                            dir = d.clone();
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
        Tree { map }
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
        // .inspect(|e| println!("{:?}",e))
        .collect::<Vec<_>>();

    println!("{:?}", Tree::parse_history(&out));

}