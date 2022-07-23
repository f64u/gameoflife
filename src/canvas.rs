use cursive::views::TextContent;

use crate::game::World;

pub struct Canvas {
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
