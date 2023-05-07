use std::collections::HashMap;
use std::hash::Hash;
use bracket_lib::color::{BLUE, RED, WHITE};
use bracket_lib::prelude::{BResult, BTerm, BTermBuilder, GameState, main_loop};
use specs::prelude::*;
use specs::shred::Fetch;
use specs_derive::*;
use crate::Square::{Black, White};

const CENTER: (i32,i32) = (40,25);

fn main() -> BResult<()> {
    let mut sim = Simulation{ world: World::new() };

    sim.world.register::<Coord>();
    sim.world.register::<Square>();
    sim.world.register::<Direction>();
    sim.world.register::<AntStepMove>();
    sim.world.register::<Ant>();

    sim.world.create_entity()
        .with(Coord(0,0))
        .with(Direction::Down)
        .with(AntStepMove)
        .with(Ant)
        .build();

    sim.world.create_entity()
        .with(Coord(0,0))
        .with(Square::default())
        .build();

    let ctx = BTermBuilder::simple80x50()
        .with_fps_cap(5f32)
        .with_title("Langton Ant - ECS")
        .build()?;

    main_loop(ctx,sim)
}

struct Simulation {
    world: World
}

impl GameState for Simulation {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut antmove = AntStepMove;
        antmove.run_now(&self.world);
        self.world.maintain();

        let pos = self.world.read_storage::<Coord>();
        let square = self.world.read_storage::<Square>();
        let ant = self.world.read_storage::<Ant>();

        (&pos, &square).join()
            // .inspect(|d| println!("Draw: {:?}",d))
            .for_each(|(p, s)|
                ctx.set_bg(p.0 + CENTER.0, p.1 + CENTER.1, match s {
                    Black => BLUE,
                    White => WHITE
                })
            );

        (&ant, &pos).join()
            .for_each(|(_, p)| ctx.set_bg(p.0 + CENTER.0, p.1 + CENTER.1, RED));
    }
}

#[derive(Component,Debug,Default)]
#[storage(NullStorage)]
struct Ant;

#[derive(Component,Debug,Default)]
#[storage(NullStorage)]
struct Board;


#[derive(Component,Debug, Copy, Clone)]
enum Square { Black, White }
impl Default for Square {
    fn default() -> Self {
        Square::Black
    }
}

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

#[derive(Component,Debug)]
struct AntStepMove;

impl<'a> System<'a> for AntStepMove {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Direction>, WriteStorage<'a, Coord>,
        WriteStorage<'a, Square>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            ent,
            mut dir, mut apos,
            mut sqr
        ) = data;
        let mut new_square = (false,None);

        let mut squares = (&apos, &mut sqr).join().map(|d| (*d.0, d.1)).collect::<HashMap<Coord, _>>();

        (&ent, &mut dir, &mut apos).join()
            .inspect(|p| println!("{:?}",&p))
            .for_each(|(.., d, p)| {

                let mut def = Square::default();

                let sqr =
                    if let Some(sqr) = squares.get_mut(p) { sqr }
                    else {
                        new_square = (true, Some(*p));
                        &mut def
                    };

               match match sqr {
                   Black => d.turn_right(),
                   White => d.turn_left(),
               } {
                   Direction::Right => p.0 += 1,
                   Direction::Down => p.1 += 1,
                   Direction::Left => p.0 -= 1,
                   Direction::Up => p.1 -= 1
               };
               *sqr = match *sqr {
                   Black => White,
                   White => Black,
               };
            });

        match new_square {
            (true, Some(pos)) => {
                ent.build_entity()
                    .with(pos, &mut apos)
                    .with(Black, &mut sqr)
                    .build();
            },
            _ => {}
        };
    }
}