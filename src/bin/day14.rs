use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use bracket_lib::prelude::*;

fn parse_plines(input:&str) -> (Coord, Coord, Vec<Vec<Coord>>) {
    let mut br = Coord{ x: usize::MIN, y: usize::MIN };
    let mut tl = Coord{ x: usize::MAX, y: 0 };
    let plines =
        input.lines()
            .map(|line|{
                line.split(" -> ")
                    .map(|val| Coord::from_str(val).expect("Ops!"))
                    .inspect(|p|{
                        tl.x = std::cmp::min(tl.x, p.x);
                        br.x = std::cmp::max(br.x, p.x);
                        br.y = std::cmp::max(br.y, p.y);
                    })
                    .collect::<Vec<_>>()
            })
            .fold(vec![],|mut out, pline|{
                out.push(pline);
                out
            });
    (tl, br, plines)
}

fn main() -> BResult<()>{

    // let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9".to_string();
    let input = std::fs::read_to_string("src/bin/day14_input.txt").expect("ops!");

    // parse the board's wall layout
    let (tl, br, plines) = parse_plines(input.as_str());

    let mut board = Board::new(tl, br);

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
    board.toggle_floor();
    // run the sand simulation until grain settled position == starting position
    board.run(
        start, |g| g.pos.eq(&start)
    );

    println!("Scenario 2: Grains Rest: {}\n{:?}", board.grains_at_rest(), board);

    let mut ctx = BTermBuilder::simple(board.width>>1, board.height>>1)?
        .with_simple_console(board.width, board.height, "terminal8x8.png")
        .with_simple_console_no_bg(board.width, board.height, "terminal8x8.png")
        .with_simple_console_no_bg(board.width>>2, board.height>>2, "terminal8x8.png")
        .with_fps_cap(60f32)
        .build()?;

    let app = App::init(input.as_str());

    ctx.set_active_console(1);
    app.board.draw(&mut ctx);

    main_loop(ctx, app)
}

struct App {
    board: Board<Material>,
    grains: VecDeque<Grain>,
    start: Coord
}
impl App {
    fn init(input: &str) -> App {
        // parse the board's wall layout
        let (tl,br, plines) = parse_plines(input);
        let mut board = Board::new(tl, br);
        let centre = board.centre_x;
        // paint layout on the board
        plines.into_iter()
            .for_each(|pline| Painter::rock_walls(&mut board, &pline) );
        App { board, grains: VecDeque::new(), start:(centre,0).into() }
    }
}

impl GameState for App {
    fn tick(&mut self, ctx: &mut BTerm) {
        let App{ board, grains, start } = self;

        match ctx.key {
            Some(VirtualKeyCode::G) => grains.push_back(Grain::release_grain(*start)),
            Some(VirtualKeyCode::Q) => ctx.quit(),
            _ => {}
        }
        grains.iter_mut()
            .filter(|grain| !grain.is_settled())
            .for_each(|grain|{
                match (grain.fall(board), grain.settled) {
                    // grain in motion
                    (Some(_), _) => {},
                    // grain settled
                    (None, true) => {
                        *board.square_mut(grain.pos).unwrap() = Material::Sand;
                    }
                    // Grain fallen on the abyss
                    (None, _) => { }
                }
            });
        ctx.set_active_console(2);
        ctx.cls();
        grains.iter()
            .for_each(|grain| {
                let Coord{x,y} = grain.pos;
                ctx.set( x - board.offset_x, y,
                        if grain.is_settled() { YELLOW } else { RED },BLACK,
                        to_cp437('\u{2588}')
                );
            });

        ctx.set_active_console(3);
        ctx.print(1,1, format!("Grains @rest: {}  ", board.grains_at_rest()));
        ctx.print(1,38, format!("FPS: {} ",ctx.fps));
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Material { Rock, Sand, Air }
impl Default for Material {
    fn default() -> Self { Material::Air }
}
impl Board<Material> {
    fn grains_at_rest(&self) -> usize {
        self.grid.values()
            .filter(|&s| Material::Sand.eq(s) )
            .count()
    }
    fn empty_sand(&mut self) -> usize {
        self.grid.values_mut()
            .filter(|s| Material::Sand.eq(s) )
            .map(|s| *s = Material::Air)
            .count()
    }
    fn toggle_floor(&mut self) {
        let height = self.height-1;
        let left = Coord { x: self.offset_x, y: height };
        let right = Coord { x: self.offset_x + self.width - 1, y : height };
        match self.square(left) {
            Some(Material::Rock) => Painter::wall(self, left, right, Material::Air),
            _ => Painter::wall(self, left, right, Material::Rock)
        }
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
    fn draw(&self, ctx: &mut BTerm) {
        (0..self.width)
            .flat_map(|x|
                (0..self.height).map(move |y| (x,y))
            )
            .for_each(|(x,y)|{
                let (symbol, fg) = match self.square((x+self.offset_x, y).into()) {
                    Some(Material::Rock) => ('\u{2588}', GREEN),
                    Some(Material::Sand) => ('\u{2588}', YELLOW),
                    // Some(Material::Air) =>
                    _ => (' ', DARK_BLUE)
                };
                ctx.set(x,y, fg, DARK_BLUE, to_cp437(symbol) )
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
    fn wall(board: &mut Board<Material>, a: Coord, b: Coord, mat: Material) {
        let x_range = if a.x <= b.x { a.x ..= b.x } else { b.x ..= a.x };
        x_range
            .flat_map(|x| {
                let y_range = if a.y <= b.y { a.y..=b.y } else { b.y..=a.y };
                y_range.map(move |y| (x, y).into())
            })
            .for_each(|p|
                *board.square_mut(p).unwrap() = mat
            );
    }
    fn rock_walls(board: &mut Board<Material>, c: &[Coord]) {
        c.windows(2)
            .for_each(| p|
                Painter::wall(board, p[0], p[1], Material::Rock)
            );
    }
}

impl Debug for Board<Material> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"   |").expect("failed in y");
        (0..self.width).for_each(|x| { write!(f, "{:^3}", x + self.offset_x).expect("ops") });
        writeln!(f).expect("");
        (0..self.height).for_each(|y|{
            write!(f,"{y:3}|").expect("failed in y");
            (0..self.width).for_each(|x| {
                write!(f, "{:^3}",
                       match self.square((x + self.offset_x, y).into()) {
                           Some(Material::Rock) => '#',
                           Some(Material::Sand) => 'o',
                           // Material::Air => '.',
                           _ => '.'
                       }).expect("Ops!")
            });
            writeln!(f).expect("failed in y");
        });
        Ok(())
    }
}

#[derive(Ord, PartialOrd,Copy, Clone, Eq, PartialEq, Hash)]
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
        let mut iter = s.trim().split(',').map(usize::from_str );
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
    centre_x: usize,
    offset_x: usize,
    grid: HashMap<Coord,T>,
}
impl<T> Board<T>
    where T : Copy + Default {
    fn new(top_left: Coord, bottom_right: Coord) -> Board<T> {
        Board {
            height: bottom_right.y + 3,
            width : (bottom_right.y + 3) << 1,
            centre_x: (top_left.x + bottom_right.x) >> 1,
            offset_x: ((top_left.x + bottom_right.x) >> 1) - bottom_right.y - 2,
            grid: HashMap::new(),
        }
    }
    fn in_bounds(&self, p:Coord) -> bool {
        p.x < self.offset_x + self.width && p.y < self.height
    }
    fn square(&self, p: Coord) -> Option<T> {
        if !self.in_bounds(p) {
            return None
        }
        self.grid.get(&p).map(|p| *p).or(Some(T::default()))
    }
    fn square_mut(&mut self, p: Coord) -> Option<&mut T> {
        if !self.in_bounds(p) {
            return None
        }
        Some(self.grid.entry(p).or_insert(T::default()))
    }
}
