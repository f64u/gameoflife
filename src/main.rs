use std::time::Duration;

use cursive::views::{TextContent, TextView};

mod canvas;
mod game;

fn main() {
    let mut siv = cursive::default();
    siv.set_autorefresh(true);
    siv.add_global_callback('q', |s| s.quit());

    let text = TextContent::new("");
    let textbox = TextView::new_with_content(text.clone());
    siv.add_layer(textbox);

    let mut canvas = canvas::Canvas::new(game::World::new(50, 50), text);

    std::thread::spawn(move || loop {
        canvas.update();
        std::thread::sleep(Duration::from_millis(300));
    });

    siv.run();
}
