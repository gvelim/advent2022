use std::fmt::{Debug, Formatter};
use std::num::ParseIntError;
use std::ptr::addr_of_mut;
use std::str::FromStr;
use bracket_lib::prelude::*;

fn parse_plines(input:&str) -> (Coord, Vec<Vec<Coord>>) {
    let mut max = Coord{ x:0, y:0 };
    let plines =
        input.lines()
            .map(|line|{
                line.split(" -> ")
                    .map(|val| Coord::from_str(val).expect("Ops!"))
                    .inspect(|p|{
                        if max.x < p.x { max.x = p.x }
                        if max.y < p.y { max.y = p.y }
                    })
                    .collect::<Vec<_>>()
            })
            .fold(vec![],|mut out, pline|{
                out.push(pline);
                out
            });
    (max, plines)
}

fn main() -> BResult<()>{

    // let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9".to_string();
    let input = std::fs::read_to_string("src/bin/day14_input.txt").expect("ops!");

    // parse the board's wall layout
    let (max, plines) = parse_plines(input.as_str());

    let mut board = Board::new((max.x<<1)+1,max.y+2+1);

    // paint layout on the board
    plines.into_iter()
        .for_each(|pline|
            Painter::rock_walls(&mut board, &pline)
        );

    // run the sand simulation until we reach the abyss, that is, grain stopped but not settled
    let start = (500,0).into();
    board.run(
        start, |g| !g.is_settled()
    );
    println!("Scenario 1: Grains Rest: {}\n{:?}", board.grains_at_rest()-1, board);

    board.empty_sand();
    // add rock floor
    Painter::rock_wall(&mut board,(0, max.y+2).into(), (max.x<<1, max.y+2).into());
    // run the sand simulation until grain settled position == starting position
    board.run(
        start, |g| g.pos.eq(&start)
    );

    println!("Scenario 2: Grains Rest: {}\n{:?}", board.grains_at_rest(), board);

    let mut ctx = BTermBuilder::simple(max.x/3, max.y/3)?
        .with_simple_console(max.x, max.y, "terminal8x8.png")
        .with_simple_console_no_bg(max.x, max.y, "terminal8x8.png")
        .with_fps_cap(200f32)
        .build()?;

    let app = App::init(input.as_str());
    main_loop(ctx, app)
}

struct App {
    board: Board<Material>,
    init: Coord
}
impl App {
    fn init(input: &str) -> App {
        // parse the board's wall layout
        let (max, plines) = parse_plines(input);
        let mut board = Board::new((max.x<<1)+1,max.y+2+1);
        // paint layout on the board
        plines.into_iter()
            .for_each(|pline| Painter::rock_walls(&mut board, &pline) );
        App { board, init: (500,0).into() }
    }
}

impl GameState for App {
    fn tick(&mut self, ctx: &mut BTerm) {
        let App{ board, init } = self;
        ctx.set_active_console(1);
        self.board.draw(ctx, 200);
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Material { Rock, Sand, Air }
impl Default for Material {
    fn default() -> Self { Material::Air }
}
impl Board<Material> {
    fn grains_at_rest(&self) -> usize {
        self.grid.iter()
            .filter(|&s| Material::Sand.eq(s) )
            .count()
    }
    fn empty_sand(&mut self) -> usize {
        self.grid.iter_mut()
            .filter(|s| Material::Sand.eq(s) )
            .map(|s| *s = Material::Air)
            .count()
    }
    fn run<F>(&mut self, start: Coord, check_goal: F) where F: Fn(&Grain) -> bool {

        loop {
            let mut grain = Grain::release_grain(start);

            // let the grain fall until it either (a) settles or (b) falls off the board
            while grain.fall(self).is_some() {};

            // Have we reached an end state ?
                // we use a closure that passes the stopped grain
                // for checking whether (a) it has fallen in the abyss or (b) reached the starting position
            if check_goal(&grain) {
                // Mark settled grain position on the board
                *self.square_mut(grain.pos).unwrap() = Material::Sand;
                break
            }

            // Mark settled grain position on the board
            *self.square_mut(grain.pos).unwrap() = Material::Sand;
        }
    }
    fn draw(&self, ctx: &mut BTerm, offset: usize) {
        (offset..self.width+offset)
            .flat_map(|x|
                (0..self.height).map(move |y| Coord{x, y})
            )
            .for_each(|p|{
                let (symbol, fg) = match self.square(p) {
                    Some(Material::Air) => (' ', BLACK),
                    Some(Material::Rock) => ('\u{2588}', WHITE),
                    Some(Material::Sand) => ('\u{2588}',YELLOW),
                    _ => unreachable!()
                };
                ctx.set(p.x,p.y, fg, BLACK, to_cp437(symbol) )
            });
    }
}

struct Grain {
    pos: Coord,
    settled: bool
}
impl Grain {
    /// Grain constructor given a starting position
    fn release_grain(pos: Coord) -> Grain {
        Grain { pos, settled: false }
    }
    /// Returns
    /// - Coord : new position
    /// - None / settled : if has landed on a rock or another sand grain
    /// - None / not settled: if it has fallen off the cliff
    fn fall(&mut self, board: &Board<Material>) -> Option<Coord> {

        if self.settled { return None }

        let Coord{ x, y} = self.pos;

        let [lc, uc, rc] = [(x-1, y+1).into(), (x, y+1).into(), (x+1, y+1).into()];

        let l = board.square( lc );
        let u = board.square( uc );
        let r = board.square( rc );

        match (l,u,r) {
            (_, None, _) => None,
            (_, Some(Material::Air), _) => { self.pos = uc; Some(self.pos) },
            (Some(Material::Air), _, _) => { self.pos = lc; Some(self.pos) },
            (_, _, Some(Material::Air)) => { self.pos = rc; Some(self.pos) },
            (_, _, _) => { self.settled = true; None }
        }
    }
    fn is_settled(&self) -> bool {
        self.settled
    }
}

struct Painter();
impl Painter {
    fn rock_wall(board: &mut Board<Material>, a: Coord, b: Coord) {
        let x_range = if a.x <= b.x { a.x ..= b.x } else { b.x ..= a.x };
        x_range
            .flat_map(|x| {
                let y_range = if a.y <= b.y { a.y..=b.y } else { b.y..=a.y };
                y_range.map(move |y| (x, y).into())
            })
            .for_each(|p|
                *board.square_mut(p).unwrap() = Material::Rock
            );
    }
    fn rock_walls(board: &mut Board<Material>, c: &[Coord]) {
        c.windows(2)
            .for_each(| p|
                Painter::rock_wall(board,p[0], p[1])
            );
    }
}

impl Debug for Board<Material> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"   |").expect("failed in y");
        (0..self.width).for_each(|x| { write!(f, "{:^3}",x).expect("ops") });
        writeln!(f).expect("");
        (0..self.height).for_each(|y|{
            write!(f,"{y:3}|").expect("failed in y");
            (0..self.width).for_each(|x| {
                write!(f, "{:^3}",
                       match self.square((x, y).into()).unwrap() {
                           Material::Air => '.',
                           Material::Rock => '#',
                           Material::Sand => 'o',
                       }).expect("Ops!")
            });
            writeln!(f).expect("failed in y");
        });
        Ok(())
    }
}

#[derive(Ord, PartialOrd,Copy, Clone, Eq, PartialEq)]
struct Coord {
    x: usize,
    y: usize
}
impl Debug for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})",self.x,self.y)
    }
}
impl From<(usize,usize)> for Coord {
    fn from(p: (usize, usize)) -> Self {
        Coord { x:p.0, y:p.1 }
    }
}
impl FromStr for Coord {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().split(',').map(|val| usize::from_str(val) );
        Ok(Coord{
            x: iter.next().unwrap()?,
            y: iter.next().unwrap()?,
        })

    }
}

/// Generics
///
struct Board<T> {
    width: usize,
    height: usize,
    grid: Vec<T>,
}
impl<T> Board<T>
    where T : Copy + Default {
    fn new(width: usize, height: usize) -> Board<T> {
        Board {
            height,
            width,
            grid: vec![T::default(); width * height],
        }
    }
    fn in_bounds(&self, p:Coord) -> bool {
        p.x < self.width && p.y < self.height
    }
    fn square(&self, p: Coord) -> Option<&T> {
        if !self.in_bounds(p) {
            return None
        }
        Some(&self.grid[p.y * self.width + p.x])
    }
    fn square_mut(&mut self, p: Coord) -> Option<&mut T> {
        if !self.in_bounds(p) {
            return None
        }
        Some(&mut self.grid[p.y * self.width + p.x])
    }
}
