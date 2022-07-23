use itertools::Itertools;

const PROPORTION: f64 = 0.9;

#[derive(Clone)]
pub enum Cell {
    Alive,
    Dead,
}

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

    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn get_index(&self, i: i32, j: i32) -> Option<usize> {
        if i < 0 || j < 0 || i as usize > self.width || j as usize > self.height {
            None
        } else {
            Some(self.width * j as usize + i as usize)
        }
    }

    pub fn get_pos(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
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
                    .join("")
            })
            .join("\n")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Cell::*;

    #[test]
    fn count_alive_neighbors_works() {
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
        assert_eq!(world.count_alive_neighbors(0, 1), 2);
        println!();
        assert_eq!(world.count_alive_neighbors(1, 1), 2);
        println!();
        assert_eq!(world.count_alive_neighbors(2, 2), 1);
        println!();
    }

    #[test]
    fn get_index_works() {
        let world = World {
            width: 3,
            height: 5,
            cells: vec![
                Alive, Alive, Alive, // 1
                Alive, Alive, Alive, // 2
                Alive, Alive, Alive, // 3
                Alive, Alive, Alive, // 4
                Alive, Alive, Alive, // 5
            ],
        };

        assert_eq!(world.get_index(0, 0), Some(0));
        assert_eq!(world.get_index(2, 4), Some(3 * 5 - 1));
        assert_eq!(world.get_index(1, 2), Some(3 * 2 + 1));
    }

    #[test]
    fn get_pos_works() {
        let world = World {
            width: 3,
            height: 5,
            cells: vec![
                Alive, Alive, Alive, // 1
                Alive, Alive, Alive, // 2
                Alive, Alive, Alive, // 3
                Alive, Alive, Alive, // 4
                Alive, Alive, Alive, // 5
            ],
        };

        assert_eq!(world.get_pos(0), (0, 0));
        assert_eq!(world.get_pos(14), (2, 4));
        assert_eq!(world.get_pos(3 * 2 + 1), (1, 2));
    }

    #[test]
    fn both_work() {
        let world = World {
            width: 3,
            height: 5,
            cells: vec![
                Alive, Alive, Alive, // 1
                Alive, Alive, Alive, // 2
                Alive, Alive, Alive, // 3
                Alive, Alive, Alive, // 4
                Alive, Alive, Alive, // 5
            ],
        };

        for i in 0..3 * 5 {
            let (x, y) = world.get_pos(i);
            assert_eq!(world.get_index(x as i32, y as i32), Some(i));
        }
    }
}
