use bracket_lib::color::{BLUE, WHITE};
use bracket_lib::prelude::{BResult, BTerm, BTermBuilder, GameState, main_loop};
use specs::prelude::*;
use specs_derive::*;
use crate::Square::{Black, White};

const CENTER: (i32,i32) = (40,25);

fn main() -> BResult<()> {
    let mut sim = Simulation{ world: World::new() };

    sim.world.register::<Coord>();
    sim.world.register::<Square>();
    sim.world.register::<Direction>();
    sim.world.register::<AntMove>();

    sim.world.create_entity()
        .with(Coord(0,0))
        .with(White)
        .with(Direction::Down)
        .with(AntMove)
        .build();

    let ctx = BTermBuilder::simple80x50()
        .with_fps_cap(1f32)
        .with_title("Langton Ant - ECS")
        .build()?;

    main_loop(ctx,sim)
}

struct Simulation {
    world: World
}

impl GameState for Simulation {
    fn tick(&mut self, ctx: &mut BTerm) {

        let mut ant = AntMove;
        ant.run_now(&self.world);
        self.world.maintain();

        let pos = self.world.read_storage::<Coord>();
        let square = self.world.read_storage::<Square>();

        (&pos,&square).join()
            .inspect(|d| println!("Draw: {:?}",d))
            .for_each(|(p,s)|
                ctx.set_bg(p.0 + CENTER.0,p.1 + CENTER.1, match s { Black => BLUE, White => WHITE } )
            );
    }
}
#[derive(Component,Debug)]
enum Square { Black, White }

#[derive(Component,Debug)]
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
struct AntMove;
impl<'a> System<'a> for AntMove {
    type SystemData = (ReadStorage<'a, AntMove>, WriteStorage<'a, Direction>, WriteStorage<'a, Coord>, WriteStorage<'a, Square>);

    fn run(&mut self, data: Self::SystemData) {
        let (ant, mut dir, mut pos, mut sqr) = data;

        (&ant, &mut dir, &mut sqr, &mut pos).join()
            .inspect(|d| println!("Run: {:?}",d))
            .for_each(|(_ant, dir, sqr, pos)| {
                *dir = match sqr {
                    Black => dir.turn_right(),
                    White => dir.turn_left()
                };
                match dir {
                    Direction::Right => pos.0 += 1,
                    Direction::Down => pos.1 += 1,
                    Direction::Left => pos.0 -= 1,
                    Direction::Up => pos.1 -= 1
                }
                *sqr = match sqr {
                    Black => White,
                    White => Black
                };
            });
    }
}