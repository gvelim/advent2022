#[derive(Debug, Copy, Clone)]
struct Coord {
    x: isize,
    y: isize
}
#[derive(Debug, Copy, Clone)]
enum Command {
    Left,
    Right,
    Up,
    Down
}
#[derive(Debug, Copy, Clone)]
struct Step {
    cmd: Command,
    units: isize
}

trait Movable {
    fn move_to(&mut self, s: Command) -> Coord;
    fn position(&self) -> Coord;
}

#[derive(Debug)]
struct Head {
    cpos: Coord,
    lpos: Coord
}
impl Head {
    fn new(pos:Coord) -> Head {
        Head { cpos: pos, lpos: pos }
    }
}
impl Movable for Head {
    fn move_to(&mut self, cmd: Command) -> Coord {
        self.lpos = self.cpos;
        match cmd {
            Command::Left => self.cpos.x -= 1,
            Command::Right => self.cpos.x += 1,
            Command::Up => self.cpos.y -= 1,
            Command::Down => self.cpos.y += 1
        }
        self.position()
    }
    fn position(&self) -> Coord {
        self.cpos
    }
}


struct Game {
    sprites: Vec<Box<dyn Movable>>
}
impl Game {
    fn new() -> Game {
        Game {
            sprites: vec![
                Box::new(Head::new(Coord{x:0,y:0})),
            ]
        }
    }
    fn run(&mut self, mut input: Vec<Step>) {
        for step in input {
            (0..step.units).all(|_| {
                self.sprites
                    .iter_mut()
                    .all(|s| {
                        s.move_to(step.cmd);
                        println!("{:?}",s.position());
                        true
                    });
                true
            });
        }
    }
}
fn main() {
    // let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    let input = vec![
        Step { cmd: Command::Left, units: 1},
        Step { cmd: Command::Right, units: 3},
        Step { cmd: Command::Up, units: 2},
        Step { cmd: Command::Down, units: 5},
        Step { cmd: Command::Left, units: 1},
        Step { cmd: Command::Right, units: 3},
        Step { cmd: Command::Up, units: 2},
        Step { cmd: Command::Down, units: 5}
    ];

    Game::new().run( input );
}