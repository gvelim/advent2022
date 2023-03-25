use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

fn main() {
    let mut board = Board::new();
    let mut ant = Ant::land((0, 0));

    (0..600).for_each(|_| {
        board.step_run(&mut ant);
        println!("{board}");
    });

    println!("\n{}",board);
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
        Ant { pos, orient: 0 }
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
    fn step_run(&mut self, board: &mut Board) -> (isize, isize) {
        let orient = match board.square_colour(self.pos) {
            Square::White => self.turn_right(),
            Square::Black => self.turn_left()
        };
        match orient {
            Direction::Right => self.pos.0 += 1,
            Direction::Down => self.pos.1 -= 1,
            Direction::Left => self.pos.0 -= 1,
            Direction::Up => self.pos.1 += 1
        }
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
        Board{ border:(0,0,0,0), map:HashMap::new() }
    }
    fn square_colour(&mut self, p: (isize, isize)) -> Square {
        self.border.0 = min(p.0,self.border.0);
        self.border.2 = max(p.0,self.border.2);
        self.border.1 = min(p.1,self.border.1);
        self.border.3 = max(p.1,self.border.3);
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
        let pos = ant.pos;
        ant.step_run(self);
        self.inverse_square(pos);
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let area_w: isize = 20;
        let area_h : isize = 20;

        for y in 0..area_h {
            for x in 0..area_w {
                write!(f,"{:3}",
                       match self.map.get(&(x-10,y-10)) {
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
