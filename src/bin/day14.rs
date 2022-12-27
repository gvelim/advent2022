use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

fn parse_plines(input:&str) -> (Coord, Vec<Vec<Coord>>) {
    let mut max = Coord{ x:0, y:0 };
    let plines =
        input.lines()
        .map(|line|{
            line.split(&[',','-','>'])
                .map(|p| p.trim())
                .filter(|p| !p.is_empty() )
                .map(|val| usize::from_str(val).expect("ops!"))
                .collect::<Vec<_>>()
        })
        .map(|pline| {
            pline
                .chunks(2)
                .map(|p| Coord{ x:p[0], y:p[1] })
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

fn main() {

    // let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9".to_string();
    let input = std::fs::read_to_string("src/bin/day14_input.txt").expect("ops!");

    let (max, plines) = parse_plines(input.as_str());
    println!("Max {:?}",max);

    let mut board: Board<Mat> = Board::new(max.x<<1,max.y+1);
    let painter = board.get_painter();

    plines
        .into_iter()
        .all(|pline| {
            painter.polyline(&pline);
            true
        });

    println!("{:?}",board);
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

#[derive(Copy, Clone)]
enum Mat { Rock, Sand, Air }
impl Default for Mat {
    fn default() -> Self { Mat::Air }
}

impl Board<Mat> {
    fn get_painter(&mut self) -> Painter {
        Painter { board: RefCell::new(self) }
    }
}

struct Painter<'a> {
    board: RefCell<&'a mut Board<Mat>>
}

impl Painter<'_> {
    fn point(&self, p: Coord, mat: Mat) {
        let mut board = self.board.borrow_mut();
        *(*board).square_mut(p).unwrap() = mat;
    }
    fn line(&self, a: Coord, b: Coord) {
        let x_range = if a.x <= b.x { a.x ..= b.x } else { b.x ..= a.x };
        x_range
            .flat_map(|x| {
                let y_range = if a.y <= b.y { a.y..=b.y } else { b.y..=a.y };
                y_range.map(move |y| (x, y).into())
            })
            .all(|p| {
                self.point(p, Mat::Rock);
                true
            });
    }
    fn polyline(&self, c: &[Coord]) {
        c.windows(2)
            .all(| p| {
                self.line(p[0], p[1]);
                true
            });
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

impl Debug for Board<Mat> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (0..self.height).for_each(|y|{
            write!(f,"{y}|").expect("failed in y");
            (0..self.width).for_each(|x| {
                write!(f, "{:^2}",
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