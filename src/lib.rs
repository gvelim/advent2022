use std::fmt::{Debug, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

pub mod app;

#[derive(Ord, PartialOrd,Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize
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

pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<T>,
}
impl<T> Grid<T>
    where T : Default + Copy {
    pub fn new(width: usize, height: usize) -> Grid<T> {
        Grid {
            height,
            width,
            grid: vec![T::default(); width * height],
        }
    }
    pub fn in_bounds(&self, p:Coord) -> bool {
        p.x < self.width && p.y < self.height
    }
    pub fn square(&self, p: Coord) -> Option<&T> {
        if !self.in_bounds(p) {
            return None
        }
        Some(&self.grid[p.y * self.width + p.x])
    }
    pub fn square_mut(&mut self, p: Coord) -> Option<&mut T> {
        if !self.in_bounds(p) {
            return None
        }
        Some(&mut self.grid[p.y * self.width + p.x])
    }
    pub fn neighbouring(&self, cs:Coord) -> impl Iterator<Item=(Coord,&'_ T)> {
        let delta = [(0, -1), (1, 0), (-1, 0), (0, 1)];
        delta.into_iter()
            .filter_map(move |d|{
                let ns = Coord {
                    x: cs.x.saturating_add_signed(d.0),
                    y: cs.y.saturating_add_signed(d.1)
                };
                self.square(ns)
                    .map(|val| (ns,val) )
            })
    }
}