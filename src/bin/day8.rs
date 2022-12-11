type ScanSequence = Vec<Vec<Coord>>;

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
    fn tree(&self, p: Coord) -> &T {
        &self.grid[p.y * self.width + p.x]
    }
    fn tree_mut(&mut self, p: Coord) -> &mut T {
        &mut self.grid[p.y * self.width + p.x]
    }
}

#[derive(Debug)]
struct Visibility<'a> {
    forest: &'a Grid<i32>,
    visible: Grid<bool>,
    scenic: Grid<usize>
}
impl Visibility<'_> {
    fn new(forest: &Grid<i32>) -> Visibility {
        Visibility {
            forest,
            visible: Grid::new(forest.width, forest.height),
            scenic: Grid::new(forest.width, forest.height)
        }
    }
    fn count_visible(&self) -> usize {
        self.visible.grid.iter()
            .filter(|&e| *e)
            .count()
    }
    fn scan_visibility(&mut self, direction: ScanSequence) -> &mut Self {
        direction.into_iter()
            .for_each(|pos| {
                let mut tallest = -1;
                pos.into_iter().for_each(|e| {
                    let tree = self.visible.tree_mut(e);
                    let t= self.forest.tree(e);
                    if tallest.lt(t) {
                        tallest = *t;
                        *tree = *tree || true;
                    } else {
                        *tree = *tree || false;
                    }
                });
            });
        self
    }
}

fn main() {
    // let data = "30373\n25512\n65332\n33549\n35390".to_string();
    let data = std::fs::read_to_string("src/bin/day8_input.txt").expect("Ops!");

    let grid = parse_forest(data.as_str());

    let count = Visibility::new(&grid)
        .scan_visibility(left_to_right(&grid))
        .scan_visibility(top_to_bottom(&grid))
        .scan_visibility(right_to_left(&grid))
        .scan_visibility(bottom_to_up(&grid))
        .count_visible();
    println!("Total Visible = {:?}", count);
}

fn parse_forest(data: &str) -> Grid<i32>  {
    let width = data.lines().next().unwrap().len();
    let height = data.lines().count();
    let mut grid = Grid::new(width,height);

    for (y,line) in data.lines().enumerate() {
        for (x, val) in line.bytes().enumerate() {
            *grid.tree_mut((x,y).into()) = (val - b'0') as i32;
        }
    }
    grid
}

fn left_to_right(f: &Grid<i32>) -> ScanSequence {
    (0..f.height)
        .map(|y| (0..f.width).map(move |x| (x, y).into()).collect::<Vec<Coord>>() )
        .collect::<Vec<_>>()
}
fn right_to_left(f: &Grid<i32>) -> ScanSequence {
    (0..f.height)
        .map(|y| (0..f.width).rev().map(move |x| (x, y).into()).collect::<Vec<Coord>>() )
        .collect::<Vec<_>>()
}
fn top_to_bottom(f: &Grid<i32>) -> ScanSequence {
    (0..f.width)
        .map(|x| (0..f.height).map(move |y| (x,y).into()).collect::<Vec<Coord>>() )
        .collect::<Vec<_>>()
}
fn bottom_to_up(f: &Grid<i32>) -> ScanSequence {
    (0..f.width)
        .map(|x| (0..f.height).rev().map(move |y| (x,y).into()).collect::<Vec<Coord>>() )
        .collect::<Vec<_>>()
}
