use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};

fn main() {
    // let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi".to_string();
    let input = std::fs::read_to_string("src/bin/day12_input.txt").expect("ops!");

    let (grid,start, target) = parse_elevation(input.as_str());
    
    let path = grid.shortest_path(start, target);
    
    let mut gpath: Grid<u8> = Grid::new(grid.width, grid.height);

    path.iter().for_each(|&a| *gpath.square_mut(a).unwrap() = *grid.square(a).unwrap() );
    println!("{}\n{:?}",path.len(),gpath);
}

fn parse_elevation(data: &str) -> (Grid<u8>, Coord, Coord) {
    let width = data.lines().next().unwrap().len();
    let height = data.lines().count();
    let mut grid = Grid::new(width,height);
    let (mut start, mut finish) = ((0,0).into(),(0,0).into());

    for (y,line) in data.lines().enumerate() {
        for (x, val) in line.bytes().enumerate() {
            match val {
                b'S' => {
                    start = (x, y).into();
                    *grid.square_mut(start).unwrap() = 0;
                },
                b'E' => {
                    finish = (x, y).into();
                    *grid.square_mut(finish).unwrap() = b'z'-b'a'+2;
                }
                _ => *grid.square_mut((x, y).into()).unwrap() = val - b'a' + 1
            }
        }
    }
    (grid, start, finish)
}

#[derive(Copy, Clone, Eq, PartialEq)]
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

struct Grid<T>
    where T : Default + Debug + Copy {
    width: usize,
    height: usize,
    grid: Vec<T>,
}
impl<T> Grid<T>
    where T : Default + Debug + Copy {
    fn new(width: usize, height: usize) -> Grid<T> {
        Grid {
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

impl Grid<u8> {
    fn shortest_path(&self, start: Coord, target: Coord) -> Vec<Coord> {
        let mut queue = VecDeque::<Coord>::new();
        let mut visited: Grid<(bool, Option<Coord>)> = Grid::new(self.width, self.height);

        // push start in the queue
        queue.push_back(start);
        let mut path = Vec::<_>::new();
        // pop from top & while still nodes in the queue
        while let Some(cs) = queue.pop_front() {

            // position matches target
            if cs.eq(&target) {
                // extract parent position from target
                let mut cur = visited.square(target).unwrap().1.unwrap();
                while let Some(par) = visited.square(cur).unwrap().1 {
                    path.push(par);
                    cur = par;
                }
                // remove start position from path
                path.pop();
                break
            }

            visited.square_mut(cs).unwrap().0 = true;
            let &square = self.square(cs).unwrap();

            // evaluate neighbour squares and
            // push to the queue the ones elevation delta <= 1
            let delta = [(0, -1), (1, 0), (-1, 0), (0, 1)];
            delta.iter()
                .for_each(|&d| {
                    let ns = Coord {
                        x: cs.x.saturating_add_signed(d.0),
                        y: cs.y.saturating_add_signed(d.1)
                    };
                    match visited.square(ns) {
                        Some((false, None)) => {
                            let &elevation = self.square(ns).unwrap();
                            if elevation <= square + 1 {
                                visited.square_mut(ns).unwrap().1 = Some(cs);
                                queue.push_back(ns)
                            }
                        }
                        _ => {}
                    };
                });
        }
        path
    }
}

impl Debug for Grid<u8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (0..self.height).for_each(|y|{
            (0..self.width).for_each(|x| {
                let &cell = self.square((x, y).into()).unwrap();
                if cell == 0 { write!(f, "{:^3}",'.') } else { write!(f, "{:^3?}", cell) }.expect("TODO: panic message");
            });
            writeln!(f).expect("failed in y");
        });
        Ok(())
    }
}

impl Debug for Grid<bool> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (0..self.height).for_each(|y|{
            (0..self.width).for_each(|x| {
                let &cell = self.square((x, y).into()).unwrap();
                write!(f, "{:^2}",if cell {'*'} else {'.'} ).expect("failed in x");
            });
            writeln!(f).expect("failed in y");
        });
        Ok(())
    }
}