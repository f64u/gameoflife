use std::time::Duration;

use cursive::views::{TextContent, TextView};

/*

impl RepresentAs<String> for Cell {
    fn represent(&self) -> String {
        match *self {
            Cell::Alive => "#",
            Cell::Dead => " ",
        }
        .into()
    }
}

impl RepresentAs<String> for World {
    fn represent(&self) -> String {
        self.cells()
            .chunks(self.width())
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|c| c.represent())
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

struct Canvas {
    world: World,
    text: TextContent,
}

impl Portable<String> for TextContent {
    fn portray(&mut self, data: String) -> Result<(), String> {
        self.set_content(data);
        Ok(())
    }
}

impl Controller<String, World> for Canvas {
    type Adapter = TextContent;

    fn new(world: World, text: TextContent) -> Self {
        return Self { world, text };
    }

    pub fn world(&mut self) -> &mut World {

    }

    fn update(&mut self) -> {
        self.world.tick();
        self.text.set_content(self.world.represent());
        Ok(())
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

*/
