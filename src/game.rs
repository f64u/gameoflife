use cellular_automaton::{
    cell::BasicCell,
    common::{Dimensions, Position, Representable},
    world::BasicWorld,
};
use itertools::Itertools;

const PROPORTION: f64 = 0.9;

pub enum Cell {
    Alive,
    Dead,
}

impl BasicCell for Cell {}

impl Cell {
    pub fn is_alive(&self) -> bool {
        match *self {
            Cell::Alive => true,
            _ => false,
        }
    }

    pub fn kill(&mut self) {
        *self = Cell::Dead;
    }

    pub fn resurrect(&mut self) {
        *self = Cell::Alive;
    }
}

pub fn random_cells(dimensions: Dimensions, p: f64) -> impl Iterator<Item = Cell> {
    (0..dimensions.0 * dimensions.1).map(move |_| {
        let x: f64 = rand::random();
        if x < p {
            Cell::Dead
        } else {
            Cell::Alive
        }
    })
}

pub struct World {
    dimensions: Dimensions,
    cells: Vec<Cell>,
}

impl BasicWorld for World {
    type Cell = Cell;

    fn cells(&self) -> &Vec<Self::Cell> {
        &self.cells
    }

    fn cells_mut(&mut self) -> &mut Vec<Self::Cell> {
        &mut self.cells
    }

    fn new(dimensions: Dimensions, initial_cells: Vec<Self::Cell>) -> Self {
        Self {
            dimensions,
            cells: initial_cells,
        }
    }

    fn new_random(dimensions: Dimensions) -> Self {
        Self::new(dimensions, random_cells(dimensions, PROPORTION).collect())
    }

    fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    fn tick(&mut self) {
        for (i, j) in (0..self.dimensions().0).cartesian_product(0..self.dimensions().1) {
            let p = (i as isize, j as isize);
            let count = self.count_alive_neighbors(p);
            let cell = self.get_cell_mut(p).unwrap();

            match cell {
                &mut Cell::Alive if count < 2 || count > 3 => cell.kill(),
                &mut Cell::Dead if count == 3 => cell.resurrect(),
                _ => {}
            }
        }
    }

    fn refresh_random(&mut self) {
        *self.cells_mut() = random_cells(self.dimensions(), PROPORTION).collect();
    }
}

impl World {
    fn count_alive_neighbors(&self, p: Position) -> usize {
        self.moore_neighbors(p)
            .iter()
            .filter(|c| c.is_alive())
            .count()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Cell::*;

    #[test]
    fn count_alive_neighbors_works() {
        let world = World {
            dimensions: Dimensions(3, 3),
            cells: vec![
                Alive, Dead, Dead, // don't auto format
                Alive, Alive, Dead, // please don't auto format
                Dead, Dead, Dead,
            ],
        };

        assert_eq!(world.count_alive_neighbors((0, 0)), 2);
        println!();
        assert_eq!(world.count_alive_neighbors((0, 1)), 2);
        println!();
        assert_eq!(world.count_alive_neighbors((1, 1)), 2);
        println!();
        assert_eq!(world.count_alive_neighbors((2, 2)), 1);
        println!();
    }

    #[test]
    fn get_index_works() {
        let world = World {
            dimensions: Dimensions(3, 5),
            cells: vec![
                Alive, Alive, Alive, // 1
                Alive, Alive, Alive, // 2
                Alive, Alive, Alive, // 3
                Alive, Alive, Alive, // 4
                Alive, Alive, Alive, // 5
            ],
        };

        assert_eq!(world.dimensions().get_index((0, 0)), Some(0));
        assert_eq!(world.dimensions().get_index((2, 4)), Some(3 * 5 - 1));
        assert_eq!(world.dimensions().get_index((1, 2)), Some(3 * 2 + 1));
    }

    #[test]
    fn get_pos_works() {
        let world = World {
            dimensions: Dimensions(3, 5),
            cells: vec![
                Alive, Alive, Alive, // 1
                Alive, Alive, Alive, // 2
                Alive, Alive, Alive, // 3
                Alive, Alive, Alive, // 4
                Alive, Alive, Alive, // 5
            ],
        };

        assert_eq!(world.dimensions().get_pos(0), (0, 0));
        assert_eq!(world.dimensions().get_pos(14), (2, 4));
        assert_eq!(world.dimensions().get_pos(3 * 2 + 1), (1, 2));
    }

    #[test]
    fn both_work() {
        let world = World {
            dimensions: Dimensions(3, 5),
            cells: vec![
                Alive, Alive, Alive, // 1
                Alive, Alive, Alive, // 2
                Alive, Alive, Alive, // 3
                Alive, Alive, Alive, // 4
                Alive, Alive, Alive, // 5
            ],
        };

        for i in 0..3 * 5 {
            let (x, y) = world.dimensions().get_pos(i);
            assert_eq!(world.dimensions().get_index((x, y)), Some(i));
        }
    }
}
