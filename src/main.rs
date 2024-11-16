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
            game.transition(state);
        }

        next_frame().await;
    }
}

enum GameState {
    CircleHit,
    CircleMiss,
    NewCircle,
    PlayerAttempt,
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
            GameState::NewCircle => Some(self.create_new_circle()),
            GameState::PlayerAttempt => self.player_attempt(),
            GameState::CircleHit => Some(self.circle_hit()),
            GameState::CircleMiss => Some(self.circle_miss()),
            GameState::End => {
                Self::end();
                None
            }
        }
    }
    fn transition(&mut self, new_state: GameState) {
        match (&self.state, new_state) {
            (
                &GameState::Start | &GameState::CircleHit | &GameState::CircleMiss,
                GameState::NewCircle,
            ) => {
                self.state = GameState::NewCircle;
            }
            (&GameState::NewCircle, GameState::PlayerAttempt) => {
                self.state = GameState::PlayerAttempt;
            }
            (&GameState::PlayerAttempt, GameState::CircleHit) => {
                self.state = GameState::CircleHit;
            }
            (&GameState::PlayerAttempt, GameState::CircleMiss) => {
                self.state = GameState::CircleMiss;
            }
            _ => {}
        }
    }
    const fn start() -> GameState {
        GameState::NewCircle
    }
    fn end() {
        exit(0);
    }
    fn create_new_circle(&mut self) -> GameState {
        self.circle = Circle::new();
        GameState::PlayerAttempt
    }
    fn player_attempt(&self) -> Option<GameState> {
        todo!()
    }
    fn circle_hit(&mut self) -> GameState {
        self.circle.state = CircleState::Hit;
        GameState::NewCircle
    }
    fn circle_miss(&mut self) -> GameState {
        self.circle.state = CircleState::Miss;
        GameState::NewCircle
    }
}

enum CircleState {
    Hit,
    Miss,
    New,
}

struct Circle {
    state: CircleState,
}

impl Circle {
    const fn new() -> Self {
        Self {
            state: CircleState::New,
        }
    }
}
