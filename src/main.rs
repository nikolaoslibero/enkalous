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
    #[expect(clippy::infinite_loop, reason = "this loop is the game loop")]
    loop {
        next_frame().await;
    }
}
