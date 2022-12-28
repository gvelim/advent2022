use std::fmt::{Debug, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {

    // let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9".to_string();
    let input = std::fs::read_to_string("src/bin/day14_input.txt").expect("ops!");

    // parse the board's wall layout
    let (max, plines) = parse_plines(input.as_str());

    let mut board: Board<Mat> = Board::new((max.x<<1)+1,max.y+2+1);

    // paint layout on the board
    let mut painter = board.new_painter();
    plines
        .into_iter()
        .for_each(|pline|
            painter.rock_walls(&pline)
        );

    // run the sand simulation until we reach the abyss, that is, grain stopped but not settled
    let start = (500,0).into();
    board.run(
        start, |g| !g.is_settled()
    );
    println!("Scenario 1: Grains Rest: {}\n{:?}", board.grains_rest()-1, board);

    board.reset();
    // add rock floor
    board.new_painter().rock_wall((0, max.y+2).into(), (max.x<<1, max.y+2).into());
    // run the sand simulation until grain settled position == starting position
    board.run(
        start, |g| g.pos.eq(&start)
    );

    println!("Scenario 2: Grains Rest: {}\n{:?}", board.grains_rest(), board);
}

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

struct Grain<'a> {
    pos: Coord,
    settled: bool,
    board: &'a Board<Mat>
}

impl Grain<'_> {
    fn fall(&mut self) -> Option<Coord> {

        if self.settled { return None }

        let Coord{ x, y} = self.pos;

        let [lc, uc, rc] = [(x-1, y+1).into(), (x, y+1).into(), (x+1, y+1).into()];

        let l = self.board.square( lc );
        let u = self.board.square( uc );
        let r = self.board.square( rc );

        match (l,u,r) {
            (_, None, _) => None,
            (_, Some(Mat::Air), _) => { self.pos = uc; Some(self.pos) },
            (Some(Mat::Air), _, _) => { self.pos = lc; Some(self.pos) },
            (_, _, Some(Mat::Air)) => { self.pos = rc; Some(self.pos) },
            (_, _, _) => { self.settled = true; None }
        }
    }
    fn is_settled(&self) -> bool {
        self.settled
    }
}


#[derive(PartialEq, Copy, Clone)]
enum Mat { Rock, Sand, Air }
impl Default for Mat {
    fn default() -> Self { Mat::Air }
}

impl Board<Mat> {
    fn new_painter(&mut self) -> Painter {
        Painter { board: self }
    }
    fn release_grain(&mut self, pos: Coord) -> Grain {
        Grain { pos, settled: false, board: self }
    }
    fn grains_rest(&self) -> usize {
        self.grid.iter()
            .filter(|&s| *s == Mat::Sand )
            .count()
    }
    fn reset(&mut self) -> usize {
        self.grid.iter_mut()
            .filter(|s| **s == Mat::Sand )
            .map(|s| *s = Mat::Air)
            .count()
    }
    fn run<F>(&mut self, start: Coord, check_goal: F) where F: Fn(&Grain) -> bool {

        loop {
            let mut grain = self.release_grain(start);

            // let the grain fall until it either (a) settles or (b) falls off the board
            while let Some(_) = grain.fall() {};

            let pos = grain.pos;
            // Have we reached an end state ?
                // we use a closure that passes the stopped grain
                // for checking whether (a) it has fallen in the abyss or (b) reached the starting position
            if check_goal(&grain) {
                // Mark settled grain position on the board
                *self.square_mut(pos).unwrap() = Mat::Sand;
                break
            }

            // Mark settled grain position on the board
            *self.square_mut(pos).unwrap() = Mat::Sand;
        }
    }
}

struct Painter<'a> {
    board: &'a mut Board<Mat>
}

impl Painter<'_> {
    fn rock_wall(&mut self, a: Coord, b: Coord) {
        let x_range = if a.x <= b.x { a.x ..= b.x } else { b.x ..= a.x };
        x_range
            .flat_map(|x| {
                let y_range = if a.y <= b.y { a.y..=b.y } else { b.y..=a.y };
                y_range.map(move |y| (x, y).into())
            })
            .for_each(|p|
                *self.board.square_mut(p).unwrap() = Mat::Rock
            );
    }
    fn rock_walls(&mut self, c: &[Coord]) {
        c.windows(2)
            .for_each(| p|
                self.rock_wall(p[0], p[1])
            );
    }
}

impl Debug for Board<Mat> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"   |").expect("failed in y");
        (0..self.width).for_each(|x| { write!(f, "{:^3}",x).expect("ops") });
        writeln!(f).expect("");
        (0..self.height).for_each(|y|{
            write!(f,"{y:3}|").expect("failed in y");
            (0..self.width).for_each(|x| {
                write!(f, "{:^3}",
                       match self.square((x, y).into()).unwrap() {
                           Mat::Air => '.',
                           Mat::Rock => '#',
                           Mat::Sand => 'o',
                       }).expect("Ops!")
            });
            writeln!(f).expect("failed in y");
        });
        Ok(())
    }
}

/// Generics
///

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
