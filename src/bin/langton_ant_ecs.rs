use std::cmp::{max, min};
use std::collections::HashMap;
use std::hash::Hash;
use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::*;
use crate::Square::{Black, White};

const CENTER: (i32,i32) = (40,25);

fn main() -> BResult<()> {
    let mut sim = Simulation{ world: World::new() };

    sim.world.register::<Coord>();
    sim.world.register::<Square>();
    sim.world.register::<Direction>();
    sim.world.register::<Ant>();
    sim.world.insert::<Area>( Area::default() );

    sim.world.create_entity()
        .with(Coord(0,0))
        .with(Square::default())
        .build();

    let ctx = BTermBuilder::simple80x50()
        .with_simple_console(80,50,"terminal8x8.png")
        .with_fps_cap(30f32)
        .with_title("Langton Ant - ECS")
        .build()?;

    main_loop(ctx,sim)
}

struct Simulation {
    world: World
}

impl GameState for Simulation {
    fn tick(&mut self, ctx: &mut BTerm) {
        match ctx.key {
            Some(VirtualKeyCode::A) => InsertAnt.run_now(&mut self.world),
            Some(VirtualKeyCode::Q) => ctx.quit(),
            _ => {}
        }
        AntStepMove.run_now(&self.world);
        self.world.maintain();
        self.draw(ctx);
    }
}

impl Simulation {
    fn draw(&self, ctx: &mut BTerm) {
        let pos = self.world.read_storage::<Coord>();
        let square = self.world.read_storage::<Square>();
        let ant = self.world.read_storage::<Ant>();
        let area = self.world.read_resource::<Area>();

        ctx.set_active_console(1);
        (&pos, &square).join()
            // .inspect(|d| println!("Draw: {:?}",d))
            .for_each(|(.., p, s)|
                ctx.set_bg(p.0 + CENTER.0, p.1 + CENTER.1, match s {
                    Black => BLACK,
                    White => WHITE
                })
            );

        (&ant, &pos).join()
            .for_each(|(_, p)| ctx.set_bg(p.0 + CENTER.0, p.1 + CENTER.1, RED));

        ctx.set_active_console(1);
        ctx.print(1, 1, format!("Area: {:?}", (area.width(), area.height())));
    }
}

struct Area {
    tl: Coord,
    br: Coord
}
impl Default for Area {
    fn default() -> Self {
        Area{ tl: Coord(-1,-1), br: Coord(1,1) }
    }
}
impl Area {
    fn capture(&mut self, p: &Coord) -> &mut Area {
        self.br.0 = max( self.br.0, p.0);
        self.br.1 = max( self.br.1, p.1);
        self.tl.0 = min( self.tl.0, p.0);
        self.tl.1 = min( self.tl.1, p.1);
        self
    }
    fn width(&self) -> i32 {
        self.br.0 - self.tl.0 + 1
    }
    fn height(&self) -> i32 {
        self.br.1 - self.tl.1 + 1
    }
}

#[derive(Component,Debug,Default)]
#[storage(NullStorage)]
struct Ant;

#[derive(Component,Debug, Copy, Clone, Default)]
enum Square { #[default] Black, White }

#[derive(Component,Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Coord(i32,i32);

#[derive(Component, Copy, Clone, Debug)]
enum Direction { Right, Down, Left, Up }
impl Direction {
    fn turn_right(&mut self) -> Direction {
        *self = match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right
        };
        *self
    }
    fn turn_left(&mut self) -> Direction {
        *self = match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left
        };
        *self
    }
}

struct InsertAnt;
impl<'a> System<'a> for InsertAnt {
    type SystemData = (Entities<'a>, WriteStorage<'a, Direction>, WriteStorage<'a, Coord>, WriteStorage<'a, Ant>);

    fn run(&mut self, data: Self::SystemData) {
        let (ent, mut dir, mut pos, mut ant) = data;
        ent.build_entity()
            .with::<Direction>( Direction::Down, &mut dir)
            .with::<Coord>(Coord(0,0), &mut pos)
            .with::<Ant>(Ant, &mut ant)
            .build();
    }
}

struct AntStepMove;
impl<'a> System<'a> for AntStepMove {
    type SystemData = (
        Entities<'a>, Write<'a, Area>,
        WriteStorage<'a, Direction>, WriteStorage<'a, Coord>,
        WriteStorage<'a, Square>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            ent, mut area,
            mut dir, mut xy,
            mut sqr
        ) = data;

        let mut new_squares = Vec::new();
        let mut squares = (&xy, &mut sqr)
                .join()
                .map(|d| (*d.0, d.1))
                .collect::<HashMap<Coord, _>>();

        (&mut dir, &mut xy).join()
            .for_each(|(.., d, p)| {
                let mut default = Square::default();
                let sqr =
                    if let Some(sqr) = squares.get_mut(p) { sqr }
                    else {
                        new_squares.push(*p );
                        &mut default
                    };
                area.capture(p);
                match match sqr {
                    Black => d.turn_right(),
                    White => d.turn_left(),
                } {
                    Direction::Right => p.0 += 1,
                    Direction::Down => p.1 += 1,
                    Direction::Left => p.0 -= 1,
                    Direction::Up => p.1 -= 1
                };

                *sqr = match sqr {
                    Black => White,
                    White => Black,
                };
            });

        new_squares.iter()
            .for_each(|&pos| {
                    ent.build_entity()
                        .with(pos, &mut xy)
                        .with(White, &mut sqr)
                        .build();
            });
    }
}