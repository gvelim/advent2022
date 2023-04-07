use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use bracket_lib::prelude::*;

fn main() -> BResult<()> {

    let input = std::fs::read_to_string("src/bin/day12_input.txt").expect("ops!");

    // parse elevations onto a grid
    let (mut grid,start, target) = parse_elevation(input.as_str());

    // find path with closure fn() goal set at reaching the target coordinate
    let path = grid.shortest_path(start,|cs| cs.eq(&target));

    // visualise path produced
    grid.visualise_path(path);

    // reverse the elevation so E(0) and S(27)
    grid.reverse_elevation();

    // find path with closure fn() goal set as reaching elevation(26) = a
    let path = grid.shortest_path(target, |cs| 26.eq(grid.square(cs).unwrap()));

    // visualise path produced
    grid.visualise_path(path);

    let mut ctx = BTermBuilder::simple(160,60)?
        .with_simple_console(grid.width,grid.height, "terminal8x8.png")
        .with_simple_console_no_bg(grid.width,grid.height, "terminal8x8.png")
        .with_simple_console_no_bg(grid.width,grid.height, "terminal8x8.png")
        .with_fps_cap(480f32)
        .with_title("Day12: Path Search")
        .build()?;

    let app = App::init(input.as_str());
    ctx.set_active_console(1);
    app.grid.draw(&mut ctx);
    main_loop(ctx, app)
}

#[derive(Copy, Clone, Debug)]
enum Level {Menu, Level1, Level2}
#[derive(Copy, Clone, Debug)]
enum State {Init, Run, Finish}
struct App {
    grid: Grid<u8>,
    target: Coord,
    start: Coord,
    ps: PathSearch,
    state: (Level,State)
}

impl App {
    fn init(input: &str) -> App {
        // parse elevations onto a grid
        let (grid,start, target) = parse_elevation(input);
        // set start state for PathSearch
        let ps = PathSearch::init(&grid);
        App { grid, target, start, ps, state: (Level::Menu,State::Init) }
    }
    fn menu(&mut self, ctx: &mut BTerm) -> (Level,State) {
        ctx.set_active_console(3);
        match ctx.key {
            Some(VirtualKeyCode::Key1) => (Level::Level1,State::Init),
            Some(VirtualKeyCode::Key2) => (Level::Level2,State::Init),
            Some(VirtualKeyCode::Q) => {ctx.quit(); (Level::Menu, State::Run)}
            _ => {
                ctx.print_centered( 22, "Press '1' : Lowest to highest elevation");
                ctx.print_centered( 24, "Press '2' : Highest to lowest elevation ");
                ctx.print_centered( 26, "Press 'Q' to exit");
                (Level::Menu, State::Run)
            }
        }
    }
    fn level1(&mut self, ctx: &mut BTerm) -> (Level,State) {
        ctx.set_active_console(2);
        match self.state {
            (l, State::Init) => {
                self.ps.reset();
                self.ps.queue.push_back(self.start);
                ctx.set_active_console(3);
                ctx.cls();
                (l, State::Run)
            },
            (l, State::Run) => {
                match ctx.key {
                    Some(VirtualKeyCode::Q) => ctx.quit(),
                    _ => {}
                }
                match self.ps.tick(&self.grid, |cs| cs.eq(&self.target)) {
                    None => {
                        ctx.cls();
                        self.ps.draw(ctx);
                        (l, State::Run)
                    }
                    Some(target) => {
                        self.ps.queue.clear();
                        self.ps.queue.push_back(target);
                        ctx.cls();
                        self.ps.draw(ctx);
                        (l, State::Finish)
                    }
                }
            }
            (_, State::Finish) => {
                ctx.set_active_console(3);
                ctx.print_centered(10, "Path Found !!");
                (Level::Menu, State::Init)
            }
        }
    }
    fn level2(&mut self, ctx: &mut BTerm) -> (Level,State) {
        ctx.set_active_console(2);
        match self.state {
            (l, State::Init) => {
                self.ps.reset();
                self.ps.queue.push_back(self.target);
                self.grid.reverse_elevation();
                ctx.set_active_console(3);
                ctx.cls();
                (l, State::Run)
            },
            (l, State::Run) => {
                match ctx.key {
                    Some(VirtualKeyCode::Q) => ctx.quit(),
                    _ => {}
                }
                match self.ps.tick(&self.grid, |cs| 26.eq(self.grid.square(cs).unwrap())) {
                    None => {
                        ctx.cls();
                        self.ps.draw(ctx);
                        (l, State::Run)
                    }
                    Some(target) => {
                        self.ps.queue.clear();
                        self.ps.queue.push_back(target);
                        ctx.cls();
                        self.ps.draw(ctx);
                        self.grid.reverse_elevation();
                        (l, State::Finish)
                    }
                }
            }
            (_, State::Finish) => {
                ctx.set_active_console(3);
                ctx.print_centered(10, "Path Found !!");
                (Level::Menu, State::Init)
            }
        }
    }
}
impl GameState for App {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.state = match self.state {
            (Level::Menu, _) => self.menu(ctx),
            (Level::Level1, _) => self.level1(ctx),
            (Level::Level2, _) => self.level2(ctx),
        };
        ctx.set_active_console(3);
        ctx.print(0,0, format!("FPS: {}",ctx.fps));
        ctx.print(0,1, format!("State: {:?}",self.state));
    }
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

struct Grid<T> {
    width: usize,
    height: usize,
    grid: Vec<T>,
}
impl<T> Grid<T>
    where T : Default + Copy {
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
    fn neighbouring(&self, cs:Coord) -> impl Iterator<Item=(Coord,&'_ T)> {
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

struct PathSearch {
    queue: VecDeque<Coord>,
    visited: Grid<(bool,Option<Coord>)>,
    path: Vec<Coord>
}
impl PathSearch {
    fn init(grid: &Grid<u8>) -> PathSearch {
        PathSearch {
            queue: VecDeque::<Coord>::new(),
            visited: Grid::new(grid.width, grid.height),
            path: Vec::<_>::new()
        }
    }
    fn reset(&mut self) {
        self.queue.clear();
        self.visited.grid.iter_mut().for_each(|val| *val = (false, None) );
        self.path.clear();
    }
    fn tick<F>(&mut self, grid: &Grid<u8>, goal: F) -> Option<Coord> where F: Fn(Coord)->bool {
        let Some(cs) = self.queue.pop_front() else { return None };

        // position matches target
        if goal(cs) {
            return Some(cs);
        }
        // mark square as visited
        self.visited.square_mut(cs).unwrap().0 = true;

        let &square = grid.square(cs).unwrap();

        // evaluate neighbour squares and
        // push to the queue if the have elevation delta <= 1
        grid.neighbouring(cs)
            .for_each(|(ns, &elevation)| {
                if let Some((false, None)) = self.visited.square(ns) {
                    if elevation <= square + 1 {
                        // capture the square we arrived from
                        self.visited.square_mut(ns).unwrap().1 = Some(cs);
                        self.queue.push_back(ns)
                    }
                }
            });
        None
    }
    fn extract_path(&self, start:Coord) -> PathIter {
        PathIter { ps: self, cur: start }
    }
    fn draw(&self,ctx: &mut BTerm) {
        self.queue.iter()
            .for_each(|&cs| {
                ctx.set(cs.x,cs.y,RED,BLACK,to_cp437('\u{2588}'));
                self.extract_path(cs)
                    .for_each(|Coord{x,y}|
                        ctx.set(x,y,ORANGE, BLACK,to_cp437('\u{2588}'))
                    )
            })
    }
}
struct PathIter<'a> {
    ps: &'a PathSearch,
    cur: Coord
}
impl Iterator for PathIter<'_> {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        match self.ps.visited.square(self.cur).unwrap().1 {
            Some(par) => {
                self.cur = par;
                Some(par)
            }
            _ => None
        }
    }
}

impl Grid<u8> {
    fn reverse_elevation(&mut self) {
        let &max = self.grid.iter().max().unwrap();
        self.grid.iter_mut()
            .map(|val|{
                *val = max - *val;
            })
            .all(|_| true);
    }
    fn visualise_path(&self, path:Vec<Coord>) {
        let mut gpath: Grid<u8> = Grid::new(self.width, self.height);
        path.iter().for_each(|&a| *gpath.square_mut(a).unwrap() = *self.square(a).unwrap() );
        println!("Path length: {}\n{:?}",path.len(),gpath);
    }
    fn shortest_path<F>(&self, start: Coord, goal:F ) -> Vec<Coord> where F: Fn(Coord)->bool {

        let mut ps = PathSearch::init(self);
        // push start in the queue
        ps.queue.push_back(start);

        // pop from top & while still nodes in the queue
        while let Some(cs) = ps.queue.pop_front() {

            // position matches target
            if goal(cs) {
                // extract parent position from target
                let mut cur = cs;
                while let Some(par) = ps.visited.square(cur).unwrap().1 {
                    ps.path.push(par);
                    cur = par;
                }
                // remove start position from path
                ps.path.pop();
                break
            }

            // mark square as visited
            ps.visited.square_mut(cs).unwrap().0 = true;

            let &square = self.square(cs).unwrap();

            // evaluate neighbour squares and
            // push to the queue if the have elevation delta <= 1
            self.neighbouring(cs)
                .for_each(|(ns, &elevation)| {
                    if let Some((false, None)) = ps.visited.square(ns) {
                        if elevation <= square + 1 {
                            // capture the square we arrived from
                            ps.visited.square_mut(ns).unwrap().1 = Some(cs);
                            ps.queue.push_back(ns)
                        }
                    }
                })
        }
        ps.path
    }
    fn draw(&self, ctx: &mut BTerm) {
        let rgb: Vec<_> = RgbLerp::new(CADETBLUE.into(), WHITESMOKE.into(), 27).collect();
        (0..self.height).for_each(|y|{
            (0..self.width).for_each(|x|
                ctx.set_bg(x, y, self.square((x, y).into()).map(|&cell| rgb[cell as usize]).unwrap_or(BLACK.into()))
            );
        });
    }
}

impl Debug for Grid<u8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (0..self.height).for_each(|y|{
            (0..self.width).for_each(|x|
                write!(f, "{:^2}",
                       self.square((x, y).into())
                           .map(|&cell| match cell { 0 => '.', _=> 'x'})
                           .expect("TODO: panic message")
                ).expect("failed in x")
            );
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