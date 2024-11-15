use std::process::exit;

use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        window_title: String::from("enkalous"),
        sample_count: 2,
        window_width: 1920,
        window_height: 1080,
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    show_mouse(false);

    let mut player = Player::new(
        Box::new(CommandByKBM::new()),
        Box::new(PlayerMacroquadRenderer),
    );
    let mut circle = Circle::new(Box::new(CircleMacroquadRenderer), (100.0, 100.0), 64.0);

    #[expect(clippy::infinite_loop, reason = "this loop is the game loop")]
    loop {
        clear_background(BLACK);

        circle.update();
        player.update();

        if circle.is_dead() {
            circle = Circle::new(Box::new(CircleMacroquadRenderer), (100.0, 100.0), 64.0);
        }

        circle.render();
        player.render();

        next_frame().await;
    }
}

enum PlayerCommand {
    Aim((f32, f32)),
    Attack,
    Quit,
}

trait PlayerCommandInterface {
    fn get_command(&mut self) -> Option<PlayerCommand>;
    fn get_position(&self) -> (f32, f32);
}

struct CommandByKBM;

impl CommandByKBM {
    const fn new() -> Self {
        Self
    }
}

impl PlayerCommandInterface for CommandByKBM {
    fn get_command(&mut self) -> Option<PlayerCommand> {
        if is_key_pressed(KeyCode::Escape) {
            return Some(PlayerCommand::Quit);
        }

        if is_mouse_button_down(MouseButton::Left) {
            return Some(PlayerCommand::Attack);
        }

        None
    }
    fn get_position(&self) -> (f32, f32) {
        mouse_position()
    }
}

struct CommandByAutomation {
    command_list: Vec<PlayerCommand>,
    position: (f32, f32),
}

impl CommandByAutomation {
    const fn new(list: Vec<PlayerCommand>) -> Self {
        let position = (0.0, 0.0);
        Self {
            command_list: list,
            position,
        }
    }
}

impl PlayerCommandInterface for CommandByAutomation {
    fn get_command(&mut self) -> Option<PlayerCommand> {
        self.command_list.pop()
    }
    fn get_position(&self) -> (f32, f32) {
        self.position
    }
}

trait Renderable {
    fn render(&self, position: (f32, f32), color: Option<Color>);
}

trait Updatable {
    fn update(&mut self);
}

struct Player {
    command_interface: Box<dyn PlayerCommandInterface + Send>,
    renderer: Box<dyn Renderable + Send>,
    target: (f32, f32),
}

// SAFETY: I have no idea what I'm doing
unsafe impl Send for Player {}

struct PlayerMacroquadRenderer;

impl Renderable for PlayerMacroquadRenderer {
    fn render(&self, position: (f32, f32), _color: Option<Color>) {
        draw_circle_lines(position.0, position.1, 8.0, 2.0, BLUE);
    }
}

struct PlayerTerminalRenderer;

#[expect(clippy::print_stdout, reason = "this is a terminal renderer")]
impl Renderable for PlayerTerminalRenderer {
    fn render(&self, position: (f32, f32), _color: Option<Color>) {
        println!("Player is aiming at ({0}, {1})", position.0, position.1);
    }
}

impl Updatable for Player {
    fn update(&mut self) {
        self.target = self.command_interface.get_position();
        #[expect(
            clippy::single_match,
            reason = "this won't be single once attack is implemented"
        )]
        match self.command_interface.get_command() {
            Some(PlayerCommand::Quit) => exit(0),
            _ => {}
        }
    }
}

impl Player {
    fn new(
        command_interface: Box<dyn PlayerCommandInterface + Send>,
        renderer: Box<dyn Renderable + Send>,
    ) -> Self {
        Self {
            command_interface,
            renderer,
            target: (0.0, 0.0),
        }
    }
    fn render(&self) {
        self.renderer.render(self.target, None);
    }
}

struct Circle {
    renderer: Box<dyn Renderable + Send>,
    position: (f32, f32),
    radius: f32,
    color: Color,
    time_to_live: f32,
}

// SAFETY: I have no idea what I'm doing
unsafe impl Send for Circle {}

struct CircleMacroquadRenderer;

impl Renderable for CircleMacroquadRenderer {
    fn render(&self, position: (f32, f32), color: Option<Color>) {
        draw_circle(position.0, position.1, 64.0, color.unwrap_or(MAGENTA));
    }
}

struct CircleTerminalRenderer;

#[expect(clippy::print_stdout, reason = "this is a terminal renderer")]
impl Renderable for CircleTerminalRenderer {
    fn render(&self, position: (f32, f32), _color: Option<Color>) {
        println!(
            "Circle is at ({0}, {1}) with radius ({2})",
            position.0, position.1, 64.0f64
        );
    }
}

impl Updatable for Circle {
    fn update(&mut self) {
        self.time_to_live -= get_frame_time();

        if self.time_to_live <= 0.25 && self.color != RED {
            self.color = RED;
        }
    }
}

impl Circle {
    fn new(renderer: Box<dyn Renderable + Send>, position: (f32, f32), radius: f32) -> Self {
        let color = GREEN;
        let time_to_live = 5.0;
        Self {
            renderer,
            position,
            radius,
            color,
            time_to_live,
        }
    }
    fn render(&self) {
        self.renderer.render(self.position, Some(self.color));
    }
    fn is_dead(&self) -> bool {
        self.time_to_live <= 0.0
    }
}
