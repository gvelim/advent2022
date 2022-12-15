use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize
}
impl Coord {
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

trait Movable {
    fn move_to(&mut self, s: Command) -> Coord;
    fn position(&self) -> Coord;
}

#[derive(Debug)]
struct Head {
    cpos: Coord,
    lpos: Coord
}
impl Head {
    fn new(pos:Coord) -> Head {
        Head { cpos: pos, lpos: pos }
    }
}
impl Movable for Head {
    fn move_to(&mut self, cmd: Command) -> Coord {
        self.lpos = self.cpos;
        match cmd {
            Command::Left => self.cpos.x -= 1,
            Command::Right => self.cpos.x += 1,
            Command::Up => self.cpos.y -= 1,
            Command::Down => self.cpos.y += 1
        }
        self.position()
    }
    fn position(&self) -> Coord {
        self.cpos
    }
}
struct Tail {
    pos: Coord,
    head: Rc<RefCell<Head>>,
    dist: isize
}
impl Movable for Tail {
    fn move_to(&mut self, _: Command) -> Coord {
        let head = self.head.borrow().cpos;
        let tail = self.pos;
        if tail.distance(head) > self.dist {
            self.pos = self.head.borrow().lpos;
        }
        self.position()
    }
    fn position(&self) -> Coord {
        self.pos
    }
}
struct Rope {
    head: Rc<RefCell<Head>>,
    tail: Box<Tail>
}
impl Rope {
    fn new(p: Coord) -> Rope {
        let head = Rc::new( RefCell::new( Head::new(p) ));
        let tail = Box::new( Tail { pos: p, head: head.clone(), dist: 1 } );

        Rope { tail, head }
    }
}
impl Movable for Rope {
    fn move_to(&mut self, s: Command) -> Coord {
        let pos = self.head.borrow_mut().move_to(s);
        self.tail.move_to(s)
    }
    fn position(&self) -> Coord {
        self.head.borrow().lpos
    }
}

struct Game {
    sprites: Vec<Box<dyn Movable>>,
    unique: HashSet<Coord>
}
impl Game {
    fn new() -> Game {
        Game {
            sprites: vec![
                Box::new(Rope::new(Coord{x:0,y:0})),
            ],
            unique: HashSet::new()
        }
    }
    fn unique_positions(&self) -> usize {
        self.unique.len()
    }
    fn run(&mut self, input: Vec<Step>) -> &Self{
        for step in input {
            (0..step.units).all(|_| {
                self.sprites
                    .iter_mut()
                    .all(|s| {
                        self.unique.insert(
                            s.move_to(step.cmd)
                        );
                        true
                    });
                true
            });
        }
        self
    }
}

fn parse_commands(input: &str) -> Vec<Step> {
    input.lines()
        .map(|line| line.split(" ").into_iter())
        .map(|mut s|
                 (s.next().unwrap(), s.next().unwrap())
        )
        .map(|(cmd, unit)| {
            let cmd = match cmd {
                "R" => Command::Right,
                "U" => Command::Up,
                "D" => Command::Down,
                "L" => Command::Left,
                _ => panic!("Woohaaaa!")
            };
            (cmd, isize::from_str(unit).unwrap())
        })
        .fold(vec![], |mut out, (cmd, units)| {
            out.push( Step{ cmd, units });
            out
        })

}

fn main() {
    // let mut data = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";

    let data = std::fs::read_to_string("src/bin/day9_input.txt").expect("");

    let cmds = parse_commands(data.as_str());

    println!("Unique points: {}",
             Game::new()
                 .run( cmds )
                 .unique_positions()
    );

}