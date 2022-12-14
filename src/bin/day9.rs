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
impl From<(isize,isize)> for Coord {
    fn from(pos: (isize, isize)) -> Self {
        Coord{x:pos.0, y:pos.1}
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
    pos: Coord
}
impl Link {
    fn new(pos:Coord) -> Link {
        Link { pos }
    }
    fn move_to(&mut self, cmd: Command) -> Coord {
        match cmd {
            Command::Left => self.pos.x -= 1,
            Command::Right => self.pos.x += 1,
            Command::Up => self.pos.y += 1,
            Command::Down => self.pos.y -= 1
        }
        self.position()
    }
    fn move_relative(&mut self, front: &Link) -> Coord {
        let dist = front.position() - self.position();
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
        self.pos.x += dx;
        self.pos.y += dy;
        self.position()
    }
    fn position(&self) -> Coord {
        self.pos
    }
}

struct Chain {
    links: Vec<Link>
}
impl Chain {
    fn new(pos:Coord, size:usize) -> Chain {
        Chain {
            links: vec![Link::new(pos); size]
        }
    }
    fn move_to(&mut self, cmd: Command) -> Coord {

        self.links[0].move_to(cmd);
        self.links
            .iter_mut()
            .reduce(|front,tail|{
                tail.move_relative(front);
                tail
            })
            .unwrap()
            .position()
    }
}

struct Game {
    rope: Chain,
    unique: HashSet<Coord>
}
impl Game {
    fn new(rope: Chain) -> Game {
        Game { rope, unique: HashSet::new() }
    }
    fn unique_positions(&self) -> usize {
        self.unique.len()
    }
    fn run(&mut self, input: &Vec<Step>) -> &Self{
        for step in input {
            (0..step.units).all(|_| {
                self.unique.insert(
                    self.rope.move_to( step.cmd )
                );
                true
            });
        }
        self
    }
}

fn parse_commands(input: &str) -> Vec<Step> {
    input.lines()
        .map(|line| line.split(' '))
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
//     let data = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2".to_string();
//     let data = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n".to_string();

    let data = std::fs::read_to_string("src/bin/day9_input.txt").expect("");

    let cmds = parse_commands(data.as_str());

    println!("2 Link Chain - Unique points: {}",
             Game::new( Chain::new((0, 0).into(), 2))
                 .run( &cmds )
                 .unique_positions()
    );
    println!("10 Links Chain - Unique points: {}",
             Game::new( Chain::new((0, 0).into(), 10))
                 .run( &cmds )
                 .unique_positions()
    );
}