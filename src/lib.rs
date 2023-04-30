use std::fmt::{Debug, Formatter};

pub mod app;

#[derive(Copy, Clone, Eq, PartialEq)]
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

