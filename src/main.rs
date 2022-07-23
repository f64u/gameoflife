mod curisive_canvas;
mod game;
mod sdl_canvas;

fn main() -> Result<(), String> {
    sdl_canvas::run()
}
