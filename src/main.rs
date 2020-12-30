// this links the map module to the main project
mod map;
mod map_builder;
mod options;
mod components;
mod spawner;
mod camera;
mod systems;

// this module is convenient for library users and includes most necessary things
pub mod prelude {
    // re-export bracket lib
    pub use bracket_lib::prelude::*;

    // re-export legion
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

    // re-export map/player as a public modules
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::options::*;
    pub use crate::components::*;
    pub use crate::systems::*;
    pub use crate::spawner::*;
}

// use our own prelude to make it available in main
use crate::prelude::*;

// crate:: accesses the root of the tree
// super:: accesses the parent module (module immediately above current module)

enum GameMode {
    Play, Menu, Quit, Restart
}

struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
    options: GameOptions
}

impl State {
    /// creates a new empty state for our game
    fn new() -> Self {
        let options = GameOptions::new();

        let mut ecs = World::default();

        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::build(&mut rng, &options);
        let mut camera = Camera::new();

        // since we only have one player, we can add them here
        spawn_player(&mut ecs, &mut camera, map_builder.player_start);

        // create monsters
        map_builder.rooms.iter()
            .skip(1) // we are in the first room
            .map(|r| r.center())// put monster in the center
            .for_each(|pos| spawner::spawn_monster(&mut ecs, &mut rng, pos));

        resources.insert(map_builder.map);
        resources.insert(camera);

        Self {
            ecs,
            resources,
            systems: build_scheduler(),
            options
        }
    }

    // fn restart(&mut self) {
    //     let mut rng = RandomNumberGenerator::new();
    //     let map_builder = MapBuilder::build(&mut rng, &self.options);
    //
    //     self.map = map_builder.map;
    //     self.camera = Camera::new(map_builder.player_start);
    //     self.mode = GameMode::Play;
    // }

    // fn handle_input(&mut self, ctx: &mut BTerm) {
    //     if let Some(key) = ctx.key {
    //         match key {
    //             VirtualKeyCode::R => self.mode = GameMode::Restart,
    //             VirtualKeyCode::Q => self.mode = GameMode::Quit,
    //             VirtualKeyCode::M => self.mode = GameMode::Menu,
    //             VirtualKeyCode::P => self.mode = GameMode::Play,
    //             _ => {}
    //         }
    //     }
    // }

    fn show_menu(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.print_centered(5, "Rust Dungeon Crawler");
        ctx.print(10, 7, "[P] Play / Resume");
        ctx.print(10, 8, "[R] Save / Restart");
        ctx.print(10,9, "[Q] Quit");
        ctx.print_color(10, 11, GREEN, BLACK, "Options");
        ctx.print(12, 12, format!("> [<, >] Max rooms: {}", self.options.max_rooms));
        ctx.print(12, 13, format!("> [[, ]] Room size: {}", self.options.room_size));

        self.options.handle_input(ctx);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        // add current key resource (replaces any previous resource of same type)
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);

        render_draw_buffer(ctx)
            .expect("Could not render draw buffer");
    }
}

fn main() -> BError {
    let font = "dungeonfont.png";
    let context = BTermBuilder::new()
        .with_title("Rust Dungeon Crawler")
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font(font, 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, font)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, font)
        .build()?;

    let state = State::new();
    //state.restart();

    main_loop(context, state)
}
