use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Div;
use bracket_lib::prelude::*;

fn main() -> BResult<()> {
    let board = Board::init();

    let ctx = BTermBuilder::simple(80,50)?
        .with_fps_cap(60f32)
        .with_title("Langton's Ant - Press 'Q' to exit")
        .build()?;

    main_loop(ctx, board)
}

impl GameState for Board {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.tick();
        self.draw(ctx);
        if ctx.key == Some(VirtualKeyCode::Q) {
            ctx.quit();
        }
    }
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
    fn init(pos: (isize, isize)) -> Ant {
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
    fn step(&mut self, dir: Direction) {
        match dir {
            Direction::Right => self.pos.0 += 1,
            Direction::Down => self.pos.1 -= 1,
            Direction::Left => self.pos.0 -= 1,
            Direction::Up => self.pos.1 += 1
        }
    }
    fn tick(&mut self, sqr: Square) -> (isize, isize) {
        let dir = match sqr {
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
    area: (isize,isize),
    map: HashMap<(isize,isize),Square>,
    ant: Ant
}
impl Board {
    fn init() -> Board {
        Board {
            border:(-1,1,1,-1),
            area: (3,3),
            map:HashMap::new(),
            ant: Ant::init((0, 0))
        }
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
    fn tick(&mut self) {
        let p = self.ant.pos;

        self.border.0 = min(p.0,self.border.0);
        self.border.1 = max(p.1,self.border.1);
        self.border.2 = max(p.0,self.border.2);
        self.border.3 = min(p.1,self.border.3);
        self.area.0 = self.border.2 - self.border.0;
        self.area.1 = self.border.1 - self.border.3;

        let sqr = self.square_colour(p);
        self.ant.tick(sqr);
        self.inverse_square(p);
    }
    fn draw(&self, ctx:&mut BTerm) {
        ctx.set_scale(
            f32::min(80f32.div((self.area.0+3) as f32),40f32.div((self.area.1+3) as f32)),
            40, 25,
        );
        for y in self.border.3-1 ..= self.border.1+1 {
            for x in self.border.0-1 ..= self.border.2+1 {
                let (sqr, char) =
                    match self.map.get(&(x,y)) {
                        Some(Square::Black) => (GRAY1,'B'),
                        Some(Square::White) => (WHITE, 'w'),
                        None => ( GREEN, '.' )
                    };
                ctx.set(x+40, y+25, sqr, BLUE, to_cp437(char) );
            }
        }
        let (x,y) = self.ant.pos;
        ctx.set( x+40, y+25, RED, BLUE, to_cp437('@'));
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
