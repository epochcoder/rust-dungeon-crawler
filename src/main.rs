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

struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
    options: GameOptions
}

impl State {
    /// creates a new empty state for our game
    fn new() -> Self {
        Self {
            ecs: World::default(),
            resources: Resources::default(),
            systems: build_scheduler(),
            options: GameOptions::new()
        }
    }

    fn restart(&mut self) {
        self.ecs.clear();

        let mut rng = RandomNumberGenerator::new();
        let mut camera = Camera::new();
        let map_builder = MapBuilder::build(&mut rng, &self.options);

        // since we only have one player, we can add them here
        spawn_player(&mut self.ecs, &mut camera, map_builder.player_start);

        // create monsters
        map_builder.rooms.iter()
            .skip(1) // we are in the first room
            .map(|r| r.center())// put monster in the center
            .for_each(|pos| spawner::spawn_monster(&mut self.ecs, &mut rng, pos));

        self.options.mode = GameMode::Play;
        self.resources.insert(map_builder.map);
        self.resources.insert(camera);
    }

    fn handle_main_input(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::R | VirtualKeyCode::S => self.options.mode = GameMode::Restart,
                VirtualKeyCode::Q => self.options.mode = GameMode::Quit,
                VirtualKeyCode::M => self.options.mode = GameMode::Menu,
                VirtualKeyCode::P => self.options.mode = GameMode::Play,
                _ => {}
            }
        }
    }

    fn show_menu(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.print_centered(5, "Rust Dungeon Crawler");
        ctx.print(10, 7, "[P] Play / Resume");
        ctx.print(10, 8, "[R] Save / Restart");
        ctx.print(10,9, "[Q] Quit");
        ctx.print_color(10, 11, GREEN, BLACK, "Options");
        ctx.print(12, 12, format!("> [<, >] Max rooms: {}", self.options.max_rooms));
        ctx.print(12, 13, format!("> [[, ]] Room size: {}", self.options.room_size));

        self.options.handle_input(ctx);
    }

    fn run_systems(&mut self, ctx: &mut BTerm) {
        self.systems.execute(&mut self.ecs, &mut self.resources);

        render_draw_buffer(ctx)
            .expect("Could not render draw buffer");
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        // handle main game input
        self.handle_main_input(ctx);

        // add current key resource (replaces any previous resource of same type)
        self.resources.insert(ctx.key);
        // clone a set of options for our systems
        self.resources.insert(self.options.clone());

        match self.options.mode {
            GameMode::Play => self.run_systems(ctx),
            // TODO: add menu as system or somehow run with legion
            GameMode::Menu => self.show_menu(ctx),
            GameMode::Quit => ctx.quitting = true,
            GameMode::Restart => self.restart()
        }
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

    let mut state = State::new();
    state.restart();

    main_loop(context, state)
}
