use itertools::Itertools;

const PROPORTION: f64 = 0.9;

#[derive(Clone)]
pub enum Cell {
    Alive,
    Dead,
}

impl Cell {
    fn is_alive(&self) -> bool {
        match *self {
            Cell::Alive => true,
            _ => false,
        }
    }

    fn kill(&mut self) {
        *self = Cell::Dead;
    }

    fn resurrect(&mut self) {
        *self = Cell::Alive;
    }
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        match self {
            Cell::Alive => "#",
            Cell::Dead => " ",
        }
        .into()
    }
}

pub struct World {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: (0..width * height)
                .map(|_| {
                    let x: f64 = rand::random();
                    if x < PROPORTION {
                        Cell::Dead
                    } else {
                        Cell::Alive
                    }
                })
                .collect(),
        }
    }

    fn get_index(&self, i: i32, j: i32) -> Option<usize> {
        if i < 0 || j < 0 || i as usize > self.width || j as usize > self.height {
            None
        } else {
            Some(self.width * i as usize + j as usize)
        }
    }

    pub fn get_cell(&self, i: i32, j: i32) -> Option<&Cell> {
        self.get_index(i, j).map(|index| &self.cells[index])
    }

    pub fn get_cell_mut(&mut self, i: i32, j: i32) -> Option<&mut Cell> {
        self.get_index(i, j).map(|index| &mut self.cells[index])
    }

    pub fn count_alive_neighbors(&self, i: usize, j: usize) -> usize {
        (i.max(1) - 1..=(i + 1).min(self.width - 1))
            .cartesian_product(j.max(1) - 1..=(j + 1).min(self.height - 1))
            .filter(move |&item| item != (i, j))
            .filter_map(|(x, y)| self.get_cell(x as i32, y as i32))
            .filter(|cell| cell.is_alive())
            .count()
    }

    pub fn tick(&mut self) {
        for (i, j) in (0..self.width).cartesian_product(0..self.height) {
            let count = self.count_alive_neighbors(i, j);
            let cell = self.get_cell_mut(i as i32, j as i32).unwrap();

            match cell {
                &mut Cell::Alive if count < 2 || count > 3 => cell.kill(),
                &mut Cell::Dead if count == 3 => cell.resurrect(),
                _ => {}
            }
        }
    }
}

impl ToString for World {
    fn to_string(&self) -> String {
        self.cells
            .chunks(self.width)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .join("\n")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn count_alive_neighbors_works() {
        use Cell::*;
        let world = World {
            width: 3,
            height: 3,
            cells: vec![
                Alive, Dead, Dead, // don't auto format
                Alive, Alive, Dead, // please don't auto format
                Dead, Dead, Dead,
            ],
        };

        assert_eq!(world.count_alive_neighbors(0, 0), 2);
        println!();
        assert_eq!(world.count_alive_neighbors(0, 1), 3);
        println!();
        assert_eq!(world.count_alive_neighbors(1, 1), 2);
        println!();
        assert_eq!(world.count_alive_neighbors(2, 2), 1);
        println!();
    }
}
