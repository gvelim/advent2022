extern crate core;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::{Debug, Formatter};

fn main() {
    let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";
    // let input = std::fs::read_to_string("src/bin/day12_input.txt").expect("ops!");

    let (grid,start,finish) = parse_elevation(input);
    let mut visited: Grid<(bool,Option<Coord>)> = Grid::new(grid.width, grid.height);
    let mut queue = BinaryHeap::<Step>::new();

    // push start in the queue
    queue.push(Step(*grid.cell(start).unwrap(), start) );
    let mut path =  Vec::<_>::new();
    // pop from top & while still nodes in the queue
    while let Some(Step(_,p)) = queue.pop() {
        println!("Popped: {:?}",p);
        if p.eq( &finish) {
            // found target node
            let mut cur = p;
            while let Some(par) = visited.cell(cur).unwrap().1 {
                path.push(*grid.cell(par).unwrap());
                cur = par;
            }
            break
        }
        // mark position as visited
        visited.cell_mut(p).unwrap().0 = true;
        let &node = grid.cell(p).unwrap();

        // find near-by squares
        let delta = [(-1,0), (1,0), (0,-1), (0,1)];
        delta.iter()
            .filter_map(|&d| {
                let c = (
                    p.x.saturating_add_signed(d.0),
                    p.y.saturating_add_signed(d.1)
                ).into();
                let val = grid.cell(c);
                if val.is_some() && (node..=node+1).contains(val.unwrap()) {
                    match visited.cell(c) {
                        Some((false, _)) => {
                            visited.cell_mut(c).unwrap().1 = Some(p);
                            Some((c,*val.unwrap()))
                        },
                        _ => None
                    }
                } else { None }
            })
            .inspect(|e| println!("{:?}",e))
            .for_each(|(c,val)|
                queue.push(Step(val, c))
            );
        println!("==============")
    }
    println!("{:?}\n{:?}",grid,visited);
    println!("Path: {}:{:?}",path.len(),path);

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
                    *grid.cell_mut(start).unwrap() = 0;
                },
                b'E' => {
                    finish = (x, y).into();
                    *grid.cell_mut(finish).unwrap() = b'z'-b'a'+2;
                }
                _ => *grid.cell_mut((x,y).into()).unwrap() = val - b'a' + 1
            }
        }
    }
    (grid, start, finish)
}

struct Step(u8, Coord);
impl PartialEq<Self> for Step {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl Eq for Step {}
impl PartialOrd<Self> for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}
impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
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
    where T : Default + Debug + Copy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (0..self.height).for_each(|y|{
            (0..self.width).for_each(|x| {
                let cell = self.cell((x,y).into()).unwrap();
                write!(f, "{:4?}|",cell).expect("failed in x");
            });
            writeln!(f).expect("failed in y");
        });
        Ok(())
    }
}