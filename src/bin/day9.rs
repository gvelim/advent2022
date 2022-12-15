use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Sub;
use std::str::FromStr;
use std::vec;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize
}
impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Coord {
    fn new(x:isize,y:isize) -> Coord {
        Coord {x,y}
    }
    fn distance(&self, other:Self) -> isize {
        isize::abs(self.y - other.y)
            .max(isize::abs(self.x - other.x) )
    }
}
#[derive(Debug, Copy, Clone)]
enum Command {
    Left,
    Right,
    Up,
    Down
}
#[derive(Debug, Copy, Clone)]
struct Step {
    cmd: Command,
    units: isize
}

#[derive(Debug, Copy, Clone)]
struct Link {
    cpos: Coord,
    // lpos: Coord,
    dist: isize,
}
impl Link {
    fn new(pos:Coord, dist: isize) -> Link {
        Link {
            cpos: pos,
            // lpos: pos,
            dist
        }
    }
    fn move_to(&mut self, cmd: Command) -> Coord {
        // self.lpos = self.cpos;
        match cmd {
            Command::Left => self.cpos.x -= 1,
            Command::Right => self.cpos.x += 1,
            Command::Up => self.cpos.y += 1,
            Command::Down => self.cpos.y -= 1
        }
        println!("{:?}",self);
        self.cur_position()
    }
    fn move_relative(&mut self, front: &Link) -> Coord {
        let dist = front.cur_position() - self.cur_position();
        let (dx,dy) = match (dist.x, dist.y) {
            // overlapping
            (0, 0) => (0, 0),
            // touching up/left/down/right
            (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
            // touching diagonally
            (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
            // need to move up/left/down/right
            (0, 2) => (0, 1),
            (0, -2) => (0, -1),
            (2, 0) => (1, 0),
            (-2, 0) => (-1, 0),
            // need to move to the right diagonally
            (2, 1) => (1, 1),
            (2, -1) => (1, -1),
            // need to move to the left diagonally
            (-2, 1) => (-1, 1),
            (-2, -1) => (-1, -1),
            // need to move up/down diagonally
            (1, 2) => (1, 1),
            (-1, 2) => (-1, 1),
            (1, -2) => (1, -1),
            (-1, -2) => (-1, -1),
            // need to move diagonally
            (-2, -2) => (-1, -1),
            (-2, 2) => (-1, 1),
            (2, -2) => (1, -1),
            (2, 2) => (1, 1),
            _ => panic!("unhandled case: tail - head = {dist:?}"),
        };
        print!("{:?}, ",(dx,dy));
        // self.lpos = self.cpos;
        self.cpos.x += dx;
        self.cpos.y += dy;
        self.cur_position()
    }
    fn cur_position(&self) -> Coord {
        self.cpos
    }
    fn last_position(&self) -> Coord {
        self.cpos
    }
}

struct Rope {
    links: Vec<Link>
}
impl Rope {
    fn new() -> Rope {
        Rope {
            links: vec![
                Link::new(Coord::new(0,0),1),
                Link::new(Coord::new(0,0),1),
                Link::new(Coord::new(0,0),1),
                Link::new(Coord::new(0,0),1),
                Link::new(Coord::new(0,0),1),
                Link::new(Coord::new(0,0),1),
                Link::new(Coord::new(0,0),1),
                Link::new(Coord::new(0,0),1),
                Link::new(Coord::new(0,0),1),
                Link::new(Coord::new(0,0),1)
            ]
        }
    }
    fn move_to(&mut self, cmd: Command) -> Coord {

        self.links[0].move_to(cmd);

        for i in 1..self.links.len() {
            let front = self.links[i-1].clone();
            print!("{i} - ");
            self.links[i].move_relative(&front);
            println!();
        }
        println!();
        self.last_link_pos()
    }
    fn last_link_pos(&self) -> Coord {
        self.links.last().unwrap().cur_position()
    }
}

struct Game {
    unique: HashSet<Coord>
}
impl Game {
    fn new() -> Game {
        Game { unique: HashSet::new() }
    }
    fn unique_positions(&self) -> usize {
        self.unique.len()
    }
    fn run(&mut self, input: Vec<Step>) -> &Self{
        let mut rope = Rope::new();
        for step in input {
            (0..step.units).all(|_| {
                rope.move_to(step.cmd);
                self.unique.insert(rope.last_link_pos());
                true
            });
        }
        self
    }
}

fn parse_commands(input: &str) -> Vec<Step> {
    input.lines()
        .map(|line| line.split(" ").into_iter())
        .map(|mut s| {
            let cmd = match s.next() {
                Some("R") => Command::Right,
                Some("U") => Command::Up,
                Some("D") => Command::Down,
                Some("L") => Command::Left,
                _ => panic!("Woohaaaa!")
            };
            (cmd, isize::from_str(s.next().unwrap()).unwrap())
        })
        .fold(vec![], |mut out, (cmd, units)| {
            out.push( Step{ cmd, units });
            out
        })

}

fn main() {
    // let data = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2".to_string();
    // let data = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n".to_string();

    let data = std::fs::read_to_string("src/bin/day9_input.txt").expect("");

    let cmds = parse_commands(data.as_str());

    println!("Unique points: {}",
             Game::new()
                 .run( cmds )
                 .unique_positions()
    );

}