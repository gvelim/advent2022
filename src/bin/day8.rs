use std::fmt::Debug;

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize
}
impl From<(usize,usize)> for Position {
    fn from(p: (usize, usize)) -> Self {
        Position { x:p.0, y:p.1 }
    }
}
#[derive(Debug)]
struct Grid<T> {
    width: usize,
    height: usize,
    grid: Vec<T>,
}
impl<T> Grid<T> where T : Default + Copy {
    fn new(height: usize, width: usize) -> Grid<T> {
        Grid {
            height,
            width,
            grid: vec![T::default(); width * height]
        }
    }
    fn in_bounds(&self, p: &Position) -> bool {
        p.y < self.height && p.x < self.width
    }
    fn tree(&self, p: Position) -> Option<&T> {
        if ! self.in_bounds(&p) {
            return None
        }
        Some(&self.grid[p.y * self.width + p.x])
    }
    fn tree_mut(&mut self, p: Position) -> Option<&mut T> {
        if ! self.in_bounds(&p) {
            return None
        }
        Some(&mut self.grid[p.y * self.width + p.x])
    }
}
fn parse_grid(data: &str) -> Grid<u8>  {
    let width = data.lines().next().unwrap().len();
    let height = data.lines().count();
    let mut grid = Grid::new(width,height);

    for (y,line) in data.lines().enumerate() {
        for (x, val) in line.bytes().enumerate() {
            *grid.tree_mut((x,y).into()).unwrap() = val - b'0';
        }
    }
    grid
}

fn main() {
    let data = "30373\n25512\n65332\n33549\n35390";

    let grid = parse_grid(data);
    println!("{:?}",grid);

}