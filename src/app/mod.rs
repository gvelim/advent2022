use std::collections::HashMap;
use bracket_lib::prelude::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash )]
pub enum Level { MENU, LEVEL1, LEVEL2 }

#[derive(Copy, Clone, Debug)]
pub enum State { INIT, RUN, FINISH }

/// Trait for implementing a level that can be (a) registered against and (b) called by the App Object tick function
///
pub trait AppLevel {
    /// the Global Store owned by the App and which will be passed on each function call below
    type Store;
    /// Called during level initialisation. Must return a target level/state
    fn init(&mut self, ctx: &mut BTerm, store: &mut Self::Store) -> (Level, State);
    /// Called continuously and while in a run state. Must return a target level/state
    fn run(&mut self, ctx: &mut BTerm, store: &mut Self::Store) -> (Level, State);
    /// Called at termination to clean up level state. Must return a target level/state
    fn term(&mut self, ctx: &mut BTerm, store: &mut Self::Store) -> (Level, State);
}
/// App wrapper for the bracket_lib GameState::tick() function
/// Holds the global store for the application; generic type
/// Handles the routing between level states
///
pub struct App<Store> {
    store: Store,
    levels: HashMap<Level, Box<dyn AppLevel<Store=Store>>>,
    state: (Level,State)
}
impl<Store> App<Store> {
    /// Take ownership of the data store that will be used for passing between levels
    /// sets initial state
    pub fn init(store: Store) -> App<Store> {
        App {
            store,
            levels: HashMap::new(),
            state: (Level::MENU, State::INIT)
        }
    }
    /// Registers a level Handler;
    /// A Level Handler is anything that implements the AppLevel trait
    pub fn register_level(&mut self, level: Level, exec: impl AppLevel<Store=Store> + 'static) {
        self.levels.insert(level, Box::new(exec));
    }
    /// return a mutable reference to the global data store
    pub fn store(&mut self) -> &mut Store {
        &mut self.store
    }
}
/// Wrapper of the bracket_lib tick() function
/// that enables level state management and routing
///
impl<Store: 'static> GameState for App<Store> {
    fn tick(&mut self, ctx: &mut BTerm) {
        if let Some(VirtualKeyCode::Q) = ctx.key {
            ctx.quit()
        }
        let level = self.state.0;
        self.state = match self.state.1 {
            State::INIT => self.levels.get_mut(&level)
                .expect(format!("App::tick() - Level \"{:?}\" not registered", level).as_str())
                .init(ctx, &mut self.store),
            State::RUN => self.levels.get_mut(&level)
                .expect(format!("App::tick() - Level \"{:?}\" not registered", level).as_str())
                .run(ctx, &mut self.store),
            State::FINISH => self.levels.get_mut(&level)
                .expect(format!("App::tick() - Level \"{:?}\" not registered", level).as_str())
                .term(ctx, &mut self.store),
        };
        ctx.set_active_console(3);
        ctx.print(0,0, format!("FPS: {}",ctx.fps));
        ctx.print(0,1, format!("State: {:?}       ",self.state));
    }
}
