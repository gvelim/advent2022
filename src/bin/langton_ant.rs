use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

fn main() {
    let mut board = Board::new();
    let mut ant = Ant::land((0, 0));

    (0..500).for_each(|_| {
        board.step_run(&mut ant);
        board.draw();
    });
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Square { White, Black }
impl Default for Square {
    fn default() -> Self { Square::White }
}

#[derive(Copy, Clone, Debug)]
enum Direction { Right, Down, Left, Up }
const DIRECTION: [Direction;4] = [Direction::Down, Direction::Left, Direction::Up, Direction::Right];

#[derive(Debug)]
struct Ant {
    pos: (isize,isize),
    orient: usize
}
impl Ant {
    fn land(pos: (isize,isize)) -> Ant {
        Ant { pos, orient: 1 }
    }
    fn turn_right(&mut self) -> Direction {
        self.orient = match self.orient {
            0..=2 => self.orient + 1,
            3 => 0,
            _ => unreachable!()
        };
        DIRECTION[self.orient]
    }
    fn turn_left(&mut self) -> Direction {
        self.orient = match self.orient {
            0 => 3,
            1..=3 => self.orient - 1,
            _ => unreachable!()
        };
        DIRECTION[self.orient]
    }
    fn step(&mut self, dir: Direction) {
        match dir {
            Direction::Right => self.pos.0 += 1,
            Direction::Down => self.pos.1 -= 1,
            Direction::Left => self.pos.0 -= 1,
            Direction::Up => self.pos.1 += 1
        }
    }
    fn step_run(&mut self, board: &mut Board) -> (isize, isize) {
        let dir = match board.square_colour(self.pos) {
            Square::White => self.turn_right(),
            Square::Black => self.turn_left()
        };
        self.step( dir );
        self.pos
    }
}

#[derive(Debug)]
struct Board {
    border: (isize,isize,isize,isize),
    map: HashMap<(isize,isize),Square>
}
impl Board {
    fn new() -> Board {
        Board{ border:(-1,1,1,-1), map:HashMap::new() }
    }
    fn square_colour(&mut self, p: (isize, isize)) -> Square {
        *self.map.entry(p).or_insert(Square::default())
    }
    fn inverse_square(&mut self, p: (isize, isize)) {
        if let Some(sqr) = self.map.get_mut(&p) {
            *sqr = match *sqr {
                Square::White => Square::Black,
                Square::Black => Square::White
            }
        }
    }
    fn step_run(&mut self, ant:&mut Ant) {
        let p = ant.pos;

        self.border.0 = min(p.0,self.border.0);
        self.border.1 = max(p.1,self.border.1);
        self.border.2 = max(p.0,self.border.2);
        self.border.3 = min(p.1,self.border.3);

        ant.step_run(self);
        self.inverse_square(p);
    }
    fn draw(&self) {
        println!("{self}");
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        writeln!(f,"{:?}", (&self.border))?;
        for y in self.border.3-1 ..= self.border.1+1 {
            for x in self.border.0-1 ..= self.border.2+1 {
                write!(f,"{:3}",
                       match self.map.get(&(x,y)) {
                           None => '.',
                           Some(&sqr) => if sqr == Square::Black { 'B' } else { 'w' }
                       }
                )?
            }
            writeln!(f)?
        }
        Ok(())
    }
}
