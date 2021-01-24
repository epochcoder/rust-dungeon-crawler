// use our own prelude to make it available in main
use crate::prelude::*;

// this links the map module to the main project
mod map;
mod map_builder;
mod options;
mod components;
mod spawner;
mod camera;
mod systems;
mod turn_state;

// this module is convenient for library users and includes most necessary things
pub mod prelude {
    // re-export bracket lib
    pub use bracket_lib::prelude::*;
    // re-export legion
    pub use legion::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;

    pub use crate::camera::*;
    pub use crate::components::*;
    // re-export map/player as a public modules
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::options::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

// crate:: accesses the root of the tree
// super:: accesses the parent module (module immediately above current module)

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
    options: GameOptions,
}

impl State {
    /// creates a new empty state for our game
    fn new() -> Self {
        Self {
            ecs: World::default(),
            resources: Resources::default(),
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
            options: GameOptions::new(),
        }
    }

    fn restart(&mut self) {
        self.ecs.clear();
        self.ecs = World::default();

        let mut rng = RandomNumberGenerator::new();
        let mut camera = Camera::new();
        let map_builder = MapBuilder::build(&mut rng, &self.options);

        // since we only have one player, we can add them here
        spawn_player(&mut self.ecs, &mut camera, map_builder.player_start);
        spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);

        // create monsters
        let monster_locations = map_builder.rooms.iter()
            .skip(1) // we are in the first room
            .map(|r| {
                let x = &rng.range(r.x1, r.x2);
                let y = &rng.range(r.y1, r.y2);
                Point::new(*x, *y)
            }).collect::<Vec<Point>>();

        monster_locations.into_iter()
            .for_each(|pos| spawner::spawn_monster(&mut self.ecs, &mut rng, pos));

        // map_builder.rooms.iter()
        //     .skip(1) // we are in the first room
        //     .map(|r| r.center())// put monster in the center
        //     .for_each(|pos| spawner::spawn_monster(&mut self.ecs, &mut rng, pos));

        self.options.mode = GameMode::Play;

        self.resources.insert(map_builder.map);
        self.resources.insert(camera);

        // initial turn state resource
        self.resources.insert(TurnState::AwaitingInput);
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
        ctx.set_active_console(2);
        ctx.print_centered(5, "The Rusty Amulet");
        ctx.print(10, 7, "[P] Play / Resume");
        ctx.print(10, 8, "[R] Save / Restart");
        ctx.print(10, 9, "[Q] Quit");
        ctx.print_color(10, 11, GREEN, BLACK, "Options");
        ctx.print(12, 12, format!("> [<, >] Max rooms: {}", self.options.max_rooms));
        ctx.print(12, 13, format!("> [[, ]] Room size: {}", self.options.room_size));

        self.options.handle_input(ctx);
    }

    fn clear_consoles(ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        ctx.set_active_console(3);
        ctx.cls();
    }

    fn run_systems(&mut self, ctx: &mut BTerm) {
        // determine which systems to execute bases on the current turn state
        let state = self.resources.get::<TurnState>().unwrap().clone();
        match state {
            TurnState::AwaitingInput => self.input_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self.player_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self.monster_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
        }

        // how to build menu system in to this? (render menu, with background, etc...

        render_draw_buffer(ctx)
            .expect("Could not render draw buffer");
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(3);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(4, WHITE, BLACK,
                                 "Slain by a monster, your hero's \njourney has come to premature end.", );
        ctx.print_color_centered(5, WHITE, BLACK,
                                 "The Amulet of YALA remains unclaimed, \nand your home town has not been saved by the onslaught", );
        ctx.print_color_centered(8, YELLOW, BLACK,
                                 "Dont worry, you can always try again with a new hero!", );
        ctx.print_color_centered(9, YELLOW, BLACK,
                                 "Press 'R' to play again", );
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(3);
        ctx.print_color_centered(2, GREEN, BLACK, "Your quest has ended.");
        ctx.print_color_centered(4, WHITE, BLACK,
                                 "Victorious, your hero's \njourney has come to an end.", );
        ctx.print_color_centered(5, WHITE, BLACK,
                                 "The Amulet of YALA has been claimed, \nand your home town has been saved by the monster onslaught", );
        ctx.print_color_centered(9, YELLOW, BLACK,
                                 "Press 'R' to play again, or 'Q' to quit", );
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        State::clear_consoles(ctx);

        // handle main game input
        self.handle_main_input(ctx);

        // add current key resource (replaces any previous resource of same type)
        self.resources.insert(ctx.key);
        // clone a set of options for our systems
        self.resources.insert(self.options.clone());

        // set active console to fetch mouse input in correctly scaled form
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

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
    let term_font = "terminal8x8.png";

    let context = BTermBuilder::new()
        .with_title("Rust Dungeon Crawler")
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font(font, 32, 32)
        .with_font(term_font, 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, font)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, font)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, term_font) // large font
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, term_font) // smaller hud font
        .build()?;

    let mut state = State::new();
    state.restart();

    main_loop(context, state)
}
