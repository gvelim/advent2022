use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use bracket_lib::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum State { INIT, RUN, FINISH }

/// Trait for implementing a level that can be (a) registered against and (b) called by the App Object tick function
/// ```
/// use advent2022::app::{AppLevel, App, State};
/// use bracket_lib::prelude::*;
///
/// enum MyLevels { Menu }
/// struct MyGlobalStore;
/// struct Menu;
///
/// impl AppLevel for Menu {
///     type GStore = MyGlobalStore;
///     type GLevel = MyLevels;
///
///     fn init(&mut self, ctx: &mut BTerm, store: &mut Self::GStore) -> (Self::GLevel, State) { todo!() }
///     fn run(&mut self, ctx: &mut BTerm, store: &mut Self::GStore) -> (Self::GLevel, State) { todo!() }
///     fn term(&mut self, ctx: &mut BTerm, store: &mut Self::GStore) -> (Self::GLevel, State) { todo!() }
/// }
///
///
/// fn main() -> BTResult<()> {
///     let mut app = App::init( GStore{}, Level::Menu );
///     let menu = Menu;
///
///     app.register_level( Level::Menu, Menu );
///
///     main_loop(ctx, app)
/// }
/// ```
///
pub trait AppLevel {
    /// the Global Store owned by the App and which will be passed on each function call below
    type GStore;
    type GLevel;
    /// Called during level initialisation. Must return a target level/state
    fn init(&mut self, ctx: &mut BTerm, store: &mut Self::GStore) -> (Self::GLevel, State);
    /// Called continuously and while in a run state. Must return a target level/state
    fn run(&mut self, ctx: &mut BTerm, store: &mut Self::GStore) -> (Self::GLevel, State);
    /// Called at termination to clean up level state. Must return a target level/state
    fn term(&mut self, ctx: &mut BTerm, store: &mut Self::GStore) -> (Self::GLevel, State);
}
/// App wrapper for the bracket_lib GameState::tick() function
/// Holds the global store for the application; generic type
/// Handles the routing between level states
///
pub struct App<Store, Level>
    where Level: Copy + Eq + PartialEq + Hash + Debug {
    store: Store,
    levels: HashMap<Level, Box<dyn AppLevel<GStore=Store, GLevel=Level>>>,
    state: (Level,State)
}
impl<Store, Level> App<Store, Level>
    where Level: Copy + Eq + PartialEq + Hash + Debug {
    /// Take ownership of the data store that will be used for passing between levels
    /// sets initial state
    pub fn init(store: Store, start_level: Level) -> App<Store, Level> {
        App {
            store,
            levels: HashMap::new(),
            state: (start_level, State::INIT)
        }
    }
    /// Registers a level Handler;
    /// A Level Handler is anything that implements the AppLevel trait
    pub fn register_level(&mut self, level: Level, exec: impl AppLevel<GStore=Store, GLevel=Level> + 'static) {
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
impl<Level: 'static, Store: 'static, > GameState for App<Store, Level>
    where Level: Copy + Eq + PartialEq + Hash + Debug {
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
