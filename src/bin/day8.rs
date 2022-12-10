use std::intrinsics::offset;
use std::iter::repeat;

#[derive(Debug)]
struct Tree {
    height: u8,
    visible: bool
}
impl From<(u8,bool)> for Tree {
    fn from(t: (u8, bool)) -> Self {
        Tree { height:t.0, visible:t.1 }
    }
}
#[derive(Debug)]
struct Grid {
    grid: Vec<Tree>,
    offset: usize
}

impl Grid {
    fn new(data:&str) -> Grid {
        let mut offset: usize = 0;
        let grid = data.lines()
            .map(|line|
                line.bytes().map(|n| n - b'0').zip(repeat(false)).map(|t| t.into()).collect::<Vec<Tree>>()
            )
            .inspect(|a| offset = a.len())
            .reduce(|mut a,b| {a.extend(b); a})
            .unwrap();

        Grid { grid, offset }
    }
    fn position_mut(&mut self, x:usize, y:usize) -> Option<&mut Tree> {
        let col = y*self.offset;
        if col > self.grid.len() || x > self.offset {
            None
        } else {
            Some(&mut self.grid[col + x])
        }
    }
}

fn main() {
    let data = "30373\n25512\n65332\n33549\n35390";

    let grid = Grid::new(data);

}