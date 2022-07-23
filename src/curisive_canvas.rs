use std::time::Duration;

use cursive::views::{TextContent, TextView};

use crate::game::World;

struct Canvas {
    world: World,
    text: TextContent,
}

impl Canvas {
    pub fn new(world: World, text: TextContent) -> Self {
        return Self { world, text };
    }

    pub fn update(&mut self) {
        self.world.tick();
        self.text.set_content(self.world.to_string());
    }
}

pub fn run() {
    let mut siv = cursive::default();
    siv.set_autorefresh(true);

    let text = TextContent::new("");
    let textbox = TextView::new_with_content(text.clone());
    siv.add_layer(textbox);

    let mut canvas = Canvas::new(World::new(70, 70), text);
    siv.add_global_callback('q', |s| s.quit());

    std::thread::spawn(move || loop {
        canvas.update();
        std::thread::sleep(Duration::from_millis(100));
    });

    siv.run();
}
