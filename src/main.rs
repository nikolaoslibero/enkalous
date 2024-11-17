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

    let mut game = Game::new();

    #[expect(clippy::infinite_loop, reason = "this loop is the game loop")]
    loop {
        clear_background(BLACK);

        if let Some(state) = game.update() {
            game.change_state(state);
        }

        next_frame().await;
    }
}

enum GameState {
    Play,
    Paused,
    Start,
    End,
}

struct Game {
    state: GameState,
    circle: Circle,
}

impl Game {
    const fn new() -> Self {
        Self {
            state: GameState::Start,
            circle: Circle::new(),
        }
    }
    fn update(&mut self) -> Option<GameState> {
        match self.state {
            GameState::Start => Some(Self::start()),
            GameState::Play => self.play(),
            GameState::End => {
                Self::end();
                None
            }
            GameState::Paused => Self::paused(),
        }
    }
    fn change_state(&mut self, new_state: GameState) {
        match (&self.state, new_state) {
            (&GameState::Play | &GameState::Paused, GameState::End) => {
                self.state = GameState::End;
            }
            (&GameState::Play, GameState::Paused) => {
                self.state = GameState::Paused;
            }
            (&GameState::Paused | &GameState::Start, GameState::Play) => {
                self.state = GameState::Play;
            }
            _ => {}
        }
    }
    const fn start() -> GameState {
        GameState::Play
    }
    fn end() {
        exit(0);
    }
    fn play(&mut self) -> Option<GameState> {
        if is_key_pressed(KeyCode::Escape) {
            return Some(GameState::End);
        }
        if is_key_pressed(KeyCode::Space) {
            return Some(GameState::Paused);
        }

        self.check_for_hit_on_click();
        self.update_circle_state();
        self.reset_disposed_circle();
        self.circle.draw();

        None
    }
    fn update_circle_state(&mut self) {
        if let Some(state) = self.circle.update() {
            self.circle.change_state(state);
        }
    }
    fn check_for_hit_on_click(&mut self) {
        let mouse_position = Vec2::from(mouse_position());
        if is_mouse_button_pressed(MouseButton::Left) {
            if mouse_position.distance(self.circle.position) < 50.0 {
                self.circle.change_state(CircleState::Hit);
            } else {
                self.circle.change_state(CircleState::Missed);
            }
        }
    }
    fn reset_disposed_circle(&mut self) {
        if matches!(self.circle.state, CircleState::Dispose) {
            self.circle = Circle::new();
        }
    }
    fn paused() -> Option<GameState> {
        draw_text("paused", 0.0, screen_height(), 32.0, WHITE);
        if is_key_pressed(KeyCode::Escape) {
            return Some(GameState::End);
        }
        is_key_pressed(KeyCode::Space).then_some(GameState::Play)
    }
}

enum CircleState {
    Hit,
    Missed,
    Idle,
    Dead,
    Dispose,
}

struct Circle {
    state: CircleState,
    state_timer: f32,
    position: Vec2,
    color: Color,
}

impl Circle {
    const fn new() -> Self {
        Self {
            state: CircleState::Idle,
            state_timer: 0.0,
            position: vec2(50.0, 50.0),
            color: BLUE,
        }
    }
    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, 50.0, self.color);
    }
    fn update(&mut self) -> Option<CircleState> {
        match self.state {
            CircleState::Hit => self.hit(),
            CircleState::Missed => self.miss(),
            CircleState::Dead => self.dead(),
            CircleState::Dispose => None,
            CircleState::Idle => self.idle(),
        }
    }
    fn change_state(&mut self, new_state: CircleState) {
        self.state_timer = 0.0;
        match (&self.state, new_state) {
            (&CircleState::Idle, CircleState::Hit) => {
                self.state = CircleState::Hit;
            }
            (&CircleState::Idle, CircleState::Missed) => {
                self.state = CircleState::Missed;
            }
            (&CircleState::Hit | &CircleState::Missed | &CircleState::Idle, CircleState::Dead) => {
                self.state = CircleState::Dead;
            }
            (&CircleState::Dead, CircleState::Dispose) => {
                self.state = CircleState::Dispose;
            }
            _ => {}
        }
    }
    fn hit(&mut self) -> Option<CircleState> {
        let timer_limit = 0.25;
        if self.state_timer == 0.0 {
            self.color = GREEN;
        }
        if self.state_timer >= timer_limit {
            Some(CircleState::Dead)
        } else {
            self.state_timer += get_frame_time();
            None
        }
    }
    fn miss(&mut self) -> Option<CircleState> {
        let timer_limit = 0.25;
        if self.state_timer == 0.0 {
            self.color = RED;
        }
        if self.state_timer >= timer_limit {
            Some(CircleState::Dead)
        } else {
            self.state_timer += get_frame_time();
            None
        }
    }
    fn dead(&mut self) -> Option<CircleState> {
        let timer_limit = 0.25;
        if self.state_timer == 0.0 {
            self.color = DARKGRAY;
        }
        if self.state_timer >= timer_limit {
            Some(CircleState::Dispose)
        } else {
            self.state_timer += get_frame_time();
            None
        }
    }
    fn idle(&mut self) -> Option<CircleState> {
        let timer_limit = 2.0;
        if self.state_timer >= timer_limit {
            Some(CircleState::Dead)
        } else {
            self.state_timer += get_frame_time();
            None
        }
    }
}
