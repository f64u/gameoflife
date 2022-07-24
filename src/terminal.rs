use cellular_automaton::{
    common::{Dimensions, Representable},
    world::BasicWorld,
};
use spaces::curisive_canvas::{self, StringCell, StringWorld};

use crate::game::{Cell, World};

impl Representable<String> for Cell {
    fn represent(&self) -> String {
        if self.is_alive() { "#" } else { " " }.into()
    }
}

impl StringCell for Cell {}

impl Representable<String> for World {
    fn represent(&self) -> String {
        self.cells()
            .chunks(self.dimensions().0)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|c| Representable::<String>::represent(c))
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl StringWorld for World {}

pub(crate) fn run() -> Result<(), String> {
    let world = World::new_random(Dimensions(70, 70));
    curisive_canvas::run(world)?;

    Ok(())
}
