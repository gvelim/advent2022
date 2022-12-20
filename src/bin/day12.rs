use std::fmt::{Debug, Display, Formatter};

fn main() {
    let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";

    let grid = parse_forest(input);
    println!("{:?}",grid);

}

fn parse_forest(data: &str) -> Grid<u8> {
    let width = data.lines().next().unwrap().len();
    let height = data.lines().count();
    let mut grid = Grid::new(width,height);

    for (y,line) in data.lines().enumerate() {
        for (x, val) in line.bytes().enumerate() {
            *grid.cell_mut((x,y).into()).unwrap() = val;
        }
    }
    grid
}

#[derive(Debug,Copy, Clone)]
struct Coord {
    x: usize,
    y: usize
}
impl From<(usize,usize)> for Coord {
    fn from(p: (usize, usize)) -> Self {
        Coord { x:p.0, y:p.1 }
    }
}

struct Grid<T>
    where T : Default + Display + Copy {
    width: usize,
    height: usize,
    grid: Vec<T>,
    visited: Vec<bool>
}
impl<T> Grid<T>
    where T : Default + Display + Copy {
    fn new(width: usize, height: usize) -> Grid<T> {
        Grid {
            height,
            width,
            grid: vec![T::default(); width * height],
            visited: vec![false; width * height]
        }
    }
    fn in_bounds(&self, p:Coord) -> bool {
        p.x < self.width && p.y < self.height
    }
    fn cell(&self, p: Coord) -> Option<&T> {
        if !self.in_bounds(p) {
            return None
        }
        Some(&self.grid[p.y * self.width + p.x])
    }
    fn cell_mut(&mut self, p: Coord) -> Option<&mut T> {
        if !self.in_bounds(p) {
            return None
        }
        Some(&mut self.grid[p.y * self.width + p.x])
    }
}

impl<T> Debug for Grid<T>
    where T : Default + Display + Copy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (0..self.height).for_each(|y|{
            (0..self.width).for_each(|x| {
                let cell = self.cell((x,y).into()).unwrap();
                write!(f, "{cell:03}|").expect("failed in x");
            });
            writeln!(f).expect("failed in y");
        });
        Ok(())
    }
}