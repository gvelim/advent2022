use std::cmp::{max, min};
use std::collections::HashMap;
use std::hash::Hash;
use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::*;

const WIDTH: i32 = 160;
const HEIGHT: i32 = 100;
const CENTER: (i32,i32) = (WIDTH>>1,HEIGHT>>1);

fn main() -> BResult<()> {
    let mut sim = Simulation{ db: World::new(), disp: None };

    sim.db.register::<Coord>();
    sim.db.register::<Square>();
    sim.db.register::<Direction>();
    sim.db.register::<Ant>();
    sim.db.insert::<Area>( Area::default() );
    sim.db.insert::<Vec<Coord>>(Vec::new());

    sim.disp = Some(
        DispatcherBuilder::new()
            .with(AntStepMove, "MoveAnt", &[])
            .with( AntEvents::default(), "Ant Events", &["MoveAnt"])
            .build()
    );

    sim.disp.as_mut().unwrap().setup(&mut sim.db);
    InsertAnt.run_now(&mut sim.db);

    sim.db.create_entity()
        .with(Coord(0,0))
        .with(Square::default())
        .build();

    let ctx = BTermBuilder::simple(WIDTH,HEIGHT)?
        .with_simple_console_no_bg(WIDTH,HEIGHT,"terminal8x8.png")
        .with_fps_cap(30f32)
        .with_title("Langton Ant - ECS")
        .build()?;

    main_loop(ctx,sim)
}

struct Simulation<'a,'b> {
    db: World,
    disp: Option<Dispatcher<'a,'b>>
}

impl GameState for Simulation<'static,'static> {
    fn tick(&mut self, ctx: &mut BTerm) {
        match ctx.key {
            Some(VirtualKeyCode::A) => InsertAnt.run_now(&mut self.db),
            Some(VirtualKeyCode::Q) => ctx.quit(),
            _ => {}
        }
        // AntStepMove.run_now(&self.db);
        self.disp.as_mut().unwrap().run_now(&mut self.db);
        self.db.maintain();
        self.draw(ctx);
    }
}

impl Simulation<'_,'_> {
    fn draw(&self, ctx: &mut BTerm) {
        let pos = self.db.read_storage::<Coord>();
        let square = self.db.read_storage::<Square>();
        let ant = self.db.read_storage::<Ant>();
        let area = self.db.read_resource::<Area>();

        ctx.set_active_console(0);
        (&pos, &square).join()
            // .inspect(|d| println!("Draw: {:?}",d))
            .for_each(|(.., p, s)|
                ctx.set_bg(p.0 + CENTER.0, p.1 + CENTER.1, match s {
                    Square::BLACK => BLACK,
                    Square::WHITE => WHITE
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

#[derive(Component,Debug, Copy, Clone, Default, Eq, PartialEq)]
enum Square { #[default] BLACK, WHITE }

impl Square {
    fn flip(&mut self) -> Square {
        *self = match self {
            Square::BLACK => Square::WHITE,
            Square::WHITE => Square::BLACK
        };
        *self
    }

}
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Coord(i32,i32);
impl Component for Coord {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

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
        Entities<'a>,
        Write<'a, Area>, Write<'a, Vec<Coord>>,
        WriteStorage<'a, Direction>, WriteStorage<'a, Coord>,
        WriteStorage<'a, Square>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            ent,
            mut area,
            mut new_squares, // <- store new Ant positions with no corresponding squares
            mut dir, mut xy,
            mut sqr
        ) = data;

        let mut squares = (&xy, &mut sqr)
                .join()
                .map(|d| (*d.0, d.1))
                .collect::<HashMap<_, _>>();

        (&mut dir, &mut xy).join()
            .for_each(|(d, p)| {
                let mut default = Square::default();
                let sqr =
                    if let Some(sqr) = squares.get_mut(p) { sqr }
                    else {
                        // There is not square for this position hence we save the position
                        // as we cannot create the square within the closure, while "xy" is borrowed mutable
                        new_squares.push(*p );
                        &mut default
                    };

                match match sqr {
                    Square::BLACK => d.turn_right(),
                    Square::WHITE => d.turn_left(),
                } {
                    Direction::Right => p.0 += 1,
                    Direction::Down => p.1 += 1,
                    Direction::Left => p.0 -= 1,
                    Direction::Up => p.1 -= 1
                };
                sqr.flip();
                area.capture(p);
            });

        // Put missing squares against the new Ant positions
        while let Some(pos) = new_squares.pop() {
            ent.build_entity()
                .with(pos, &mut xy)
                .with(Square::WHITE, &mut sqr)
                .build();
        }
    }
}

#[derive(Default)]
struct AntEvents {
    modified: BitSet,
    reader_id: Option<ReaderId<ComponentEvent>>
}
impl<'a> System<'a> for AntEvents {
    type SystemData = (
        WriteStorage<'a, Coord>, WriteStorage<'a, Square>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut xy, mut sqr) = data;

        self.modified.clear();

        // find which entities had changed positions
        // and save them in HiBitSet do we can use in join() later
        xy.channel()
            .read(self.reader_id.as_mut().unwrap())
            .for_each(|&event|{
                match event {
                    ComponentEvent::Modified(id) => {
                        // print!("M({:?}), ",(id,xy.get(ent.entity(id)),sqr.get(ent.entity(id))));
                        self.modified.add(id);
                    }
                    _ => {}
                }
            });
        // println!();

        // //
        // let squares =
        //     // Get all modified positions
        //     (&xy, &self.modified).join()
        //         .filter(|(ap, ..)| {
        //             // get all remaining not modified positions
        //             (&xy, &sqr, !&self.modified).join()
        //                 // if there a matching Square position for the changed Ant position?
        //                 .find(|(&sp, ..)| sp.eq(ap)).is_none()
        //         })
        //         .inspect(|p| print!("{:?},",p))
        //         // If not, then the Ant landed on a non-existing Square
        //         .map(|t| *t.0)
        //         // save the position
        //         .collect::<Vec<_>>();
        // println!();
        // // create the missing squares
        // squares.into_iter()
        //     .for_each(|p| {
        //         ent.build_entity()
        //             .with(p, &mut xy)
        //             .with(Square::default(), &mut sqr)
        //             .build();
        //     });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader_id = Some(
            world.write_storage::<Coord>().register_reader()
        )
    }
}