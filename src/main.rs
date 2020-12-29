// this links the map module to the main project
mod map;
mod map_builder;
mod player;
mod options;
mod camera;

// this module is convenient for library users and includes most necessary things
pub mod prelude {
    // re-export bracket lib
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    // re-export map/player as a public modules
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::options::*;
}

// use our own prelude to make it available in main
use crate::prelude::*;

// crate:: accesses the root of the tree
// super:: accesses the parent module (module immediately above current module)

enum GameMode {
    Play, Menu, Quit, Restart
}

struct State {
    map: Map,
    mode: GameMode,
    options: GameOptions,
    player: Player,
    camera: Camera,
}

impl State {
    fn new() -> Self {
        Self {
            map: Map::new(),
            mode: GameMode::Menu,
            options: GameOptions::new(),
            player: Player::new(Point::zero()),
            camera: Camera::new(Point::zero())
        }
    }

    fn play_game(&mut self, ctx: &mut BTerm) {
        self.player.update(ctx, &self.map, &mut self.camera);
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);
    }

    fn restart(&mut self) {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::build(&mut rng, &self.options);

        self.map = map_builder.map;
        self.player = Player::new(map_builder.player_start);
        self.camera = Camera::new(map_builder.player_start);
        self.mode = GameMode::Play;
    }

    fn handle_input(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::R => self.mode = GameMode::Restart,
                VirtualKeyCode::Q => self.mode = GameMode::Quit,
                VirtualKeyCode::M => self.mode = GameMode::Menu,
                VirtualKeyCode::P => self.mode = GameMode::Play,
                _ => {}
            }
        }
    }

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

        self.handle_input(ctx);

        match self.mode {
            GameMode::Play => self.play_game(ctx),
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
