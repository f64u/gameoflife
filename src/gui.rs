use cellular_automaton::{
    common::{Dimensions, Representable},
    world::BasicWorld,
};
use sdl2::pixels::Color;
use spaces::sdl_canvas::{self, ColoredCell, ColoredWorld, Config, RepresentedWorld};

use crate::game::{Cell, World};

impl Representable<Color> for Cell {
    fn represent(&self) -> Color {
        if self.is_alive() {
            Color::RGB(0, 0, 0)
        } else {
            Color::RGB(255, 255, 255)
        }
    }
}

impl ColoredCell for Cell {}

impl Representable<RepresentedWorld> for World {
    fn represent(&self) -> RepresentedWorld {
        ColoredWorld::represent(self)
    }
}

impl ColoredWorld for World {}

pub(crate) fn run() -> Result<(), String> {
    let config = Config::new(Dimensions(1000, 800), 5);
    let world = World::new_random(Dimensions(1000 / 5, 800 / 5));
    sdl_canvas::run(config, world, "Game of Life")?;

    Ok(())
}
