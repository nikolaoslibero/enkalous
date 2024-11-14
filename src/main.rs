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

        Some(PlayerCommand::Aim(mouse_position()))
    }
}

struct CommandByAutomation {
    command_list: Vec<PlayerCommand>,
}

impl CommandByAutomation {
    const fn new(list: Vec<PlayerCommand>) -> Self {
        Self { command_list: list }
    }
}

impl PlayerCommandInterface for CommandByAutomation {
    fn get_command(&mut self) -> Option<PlayerCommand> {
        self.command_list.pop()
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
        match self.command_interface.get_command() {
            Some(PlayerCommand::Aim(target)) => self.target = target,
            Some(PlayerCommand::Attack) | None => {}
            Some(PlayerCommand::Quit) => exit(0),
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
        self.renderer.render(self.position);
    }
}
