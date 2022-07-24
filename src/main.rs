mod curisive_canvas;
mod game;
mod sdl_canvas;

fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args[0] == "in_terminal" {
        //   curisive_canvas::run();
        Ok(())
    } else {
        //  sdl_canvas::run()
        Ok(())
    }
}
