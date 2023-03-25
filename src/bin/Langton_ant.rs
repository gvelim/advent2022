use std::cmp::{max, min};
use std::collections::HashMap;

fn main() {
    let mut board = Board::new();
    let mut ant = Ant::land(&mut board,(0,0));

    ant.take(10)
        .for_each(|x| {
            println!("{:?}",x);
        });
    print!("{:?}",board.border);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Square { White, Black }
impl Default for Square {
    fn default() -> Self { Square::White }
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
}

#[derive(Copy, Clone, Debug)]
enum Orient { Right, Down, Left, Up }
const ORIENTATION: [Orient;4] = [Orient::Down,Orient::Left,Orient::Up,Orient::Right];

#[derive(Debug)]
struct Ant<'a> {
    board: &'a mut Board,
    pos: (isize,isize),
    orient: usize
}
impl Ant<'_> {
    fn land(board: &mut Board, pos: (isize,isize)) -> Ant {
        Ant { board, pos, orient: 0 }
    }
    fn turn_right(&mut self) -> Orient {
        self.orient = match self.orient {
            0..=2 => self.orient + 1,
            3 => 0,
            _ => unreachable!()
        };
        self.step_move(ORIENTATION[self.orient] );
        ORIENTATION[self.orient]
    }
    fn turn_left(&mut self) -> Orient {
        self.orient = match self.orient {
            0 => 3,
            1..=3 => self.orient - 1,
            _ => unreachable!()
        };
        self.step_move(ORIENTATION[self.orient] );
        ORIENTATION[self.orient]
    }
    fn step_move(&mut self, orient:Orient) -> (isize, isize) {
        self.board.inverse_square(self.pos);
        match orient {
            Orient::Right => self.pos.0 += 1,
            Orient::Down => self.pos.1 -= 1,
            Orient::Left => self.pos.0 -= 1,
            Orient::Up => self.pos.1 += 1
        }
        self.pos
    }
}

impl Iterator for Ant<'_> {
    type Item = Orient;
    fn next(&mut self) -> Option<Self::Item> {
        match self.board.square_colour(self.pos) {
            Square::White => Some(self.turn_right()),
            Square::Black => Some(self.turn_left())
        }
    }
}
