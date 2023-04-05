use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Div;
use std::vec;
use bracket_lib::prelude::*;

fn main() -> BResult<()> {
    let board = Board::init();

    let ctx = BTermBuilder::simple(160, 100)?
        .with_simple_console(640,480,"terminal8x8.png")
        .with_sparse_console_no_bg(80, 50, "terminal8x8.png")
        .with_fps_cap(30f32)
        .with_vsync(true)
        .with_title("Langton's Ant - Press 'Q':exit, 'A':Ant, 'I':Invert")
        .build()?;

    main_loop(ctx, board)
}

impl GameState for Board {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.tick();
        self.draw(ctx);
        self.draw_stats(ctx);
        match ctx.key {
            Some(VirtualKeyCode::Q) => ctx.quit(),
            Some(VirtualKeyCode::A) => self.inject_ant(),
            Some(VirtualKeyCode::I) => self.invert_board(),
            _ => {}
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Square { White, Black }
impl Default for Square {
    fn default() -> Self { Square::White }
}
impl Square {
    fn inverse(&mut self) {
        *self = match self {
            Square::White => Square::Black,
            Square::Black => Square::White
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction { Right, Down, Left, Up }
impl Direction {
    fn turn_right(&mut self) -> Direction {
        *self = match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right
        };
        *self
    }
    fn turn_left(&mut self) -> Direction {
        *self = match self {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right
        };
        *self
    }
}

#[derive(Debug)]
struct Ant {
    pos: (i32,i32),
    dir: Direction
}
impl Ant {
    fn init(pos: (i32, i32)) -> Ant {
        Ant { pos, dir: Direction::Down }
    }
    fn step(&mut self) {
        match self.dir {
            Direction::Right => self.pos.0 += 1,
            Direction::Down => self.pos.1 += 1,
            Direction::Left => self.pos.0 -= 1,
            Direction::Up => self.pos.1 -= 1
        }
    }
    fn tick(&mut self, sqr: Square) -> (i32, i32) {
        match sqr {
            Square::White => self.dir.turn_right(),
            Square::Black => self.dir.turn_left()
        };
        self.step();
        self.pos
    }
}

#[derive(Debug)]
struct Board {
    border: (i32,i32,i32,i32),
    area: (i32,i32),
    map: HashMap<(i32,i32),Square>,
    ant: Vec<Ant>
}
impl Board {
    fn init() -> Board {
        Board {
            border:(-1,-1,1,1),
            area: (3,3),
            map:HashMap::new(),
            ant: vec![Ant::init((0, 0))]
        }
    }
    fn inject_ant(&mut self) {
        self.ant.push(Ant::init((0, 0)))
    }
    fn square_colour(&mut self, p: (i32, i32)) -> Square {
        *self.map.entry(p).or_insert(Square::default())
    }
    fn inverse_square(&mut self, p: (i32, i32)) {
        self.map.get_mut(&p).map( Square::inverse );
    }
    fn invert_board(&mut self) {
        self.map.values_mut().for_each(Square::inverse )
    }
    fn capture_point(&mut self, p: (i32,i32)) {
        self.border.0 = min(p.0,self.border.0);
        self.border.1 = min(p.1,self.border.1);
        self.border.2 = max(p.0,self.border.2);
        self.border.3 = max(p.1,self.border.3);
        self.area.0 = self.border.2 - self.border.0 + 1;
        self.area.1 = self.border.3 - self.border.1 + 1;
    }
    fn tick(&mut self) {
        (0..self.ant.len()).for_each(|i|{
            let p = self.ant[i].pos;
            self.capture_point(p);
            let sqr = self.square_colour(p);
            self.ant[i].tick(sqr);
            self.inverse_square(p);
        })
    }
    fn draw(&self, ctx:&mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls_bg(BLACK);
        let (off_x, off_y) = (320 - self.border.2 - self.border.0, 240 - self.border.3 - self.border.1);
        ctx.set_scale(
            f32::min(640f32.div((self.area.0+10) as f32),480f32.div((self.area.1+10) as f32)),
            off_x, off_y
        );
        for y in self.border.1-1 ..= self.border.3+1 {
            for x in self.border.0-1 ..= self.border.2+1 {
                ctx.set_bg(x + off_x, y + off_y,
                           self.map
                               .get(&(x,y))
                               .map(|&sqr| match sqr { Square::Black => BLACK, _ => WHITE } )
                               .unwrap_or(BLUE)
                )
            }
        }
        self.ant.iter()
            .for_each(|ant|
                ctx.set_bg( ant.pos.0+off_x, ant.pos.1+off_y, RED )
            );
    }
    fn draw_stats(&self, ctx:&mut BTerm) {
        ctx.set_active_console(2);
        ctx.cls_bg(BLACK);
        ctx.print(0,0,format!("Corners: {:?}",self.border));
        ctx.print(0,2,format!("Offset: {:?}",(
            (self.border.0 + self.border.2, self.border.1 + self.border.3),
            (320 - self.border.2 - self.border.0, 240 - self.border.3 - self.border.1)
        )
        ));
        ctx.print(0,4,format!("Area: {:?}",self.area));
        ctx.print(0,6,format!("Population: {:?}",self.ant.len()));
        ctx.print(0,47,format!("FPS: {:?}",ctx.fps));
        ctx.print(0,49,format!("Render time: {:?}(ms)",ctx.frame_time_ms));
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
