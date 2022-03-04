use crate::tile::TileKind;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

#[derive(Component, Default, Debug)]
pub struct Map {
    pub bombs: usize,
    pub height: usize,
    pub width: usize,
    pub map: Vec<TileKind>,
}

impl Map {
    pub fn new(height: usize, width: usize) -> Self {
        let map = (0..height * width).map(|_| TileKind::Fine).collect();

        Self {
            bombs: 0,
            height,
            width,
            map,
        }
    }

    pub fn index_to_coord(&self, index: usize) -> Option<(usize, usize)> {
        let len = self.map.len();
        if index > len {
            warn!("tried to fetch coords for index {index} in map of length {len}");
            return None;
        }
        let (x, y) = (index / self.width, index % self.width);

        Some((x, y))
    }

    pub fn coord_to_index(&self, (x, y): (usize, usize)) -> Option<usize> {
        // let (x, y) = (row - 1, column - 1);
        let width = self.width;
        if x > self.width {
            warn!(
                "tried to fetch index for coords {x},{y} but {x} is greater than map width {width}"
            );
            return None;
        }
        let height = self.height;
        if x > self.height {
            warn!("tried to fetch index for coords {x},{y} but {x} is greater than map height {height}");
            return None;
        }

        let index = x * width + y;

        let len = self.map.len();
        if index > len {
            warn!("tried to fetch index for {x},{y} in map of length {len} but calculated index {index} is out of bounds");
            return None;
        }

        Some(index)
    }

    pub fn at_coords(&self, coords: (usize, usize)) -> Option<TileKind> {
        let index = self.coord_to_index(coords)?;
        self.map.get(index).copied()
    }

    pub fn set_bombs(&mut self, count: usize) {
        self.bombs = count;
        let mut remaining_bombs = count;
        let mut rng = thread_rng();
        // Place bombs
        while remaining_bombs > 0 {
            let (x, y) = (
                rng.gen_range(0..self.width) as usize,
                rng.gen_range(0..self.height) as usize,
            );

            if let Some(x @ TileKind::Fine) = self.map.get_mut(y * self.width + x) {
                *x = TileKind::Boom;
                remaining_bombs -= 1;
            }
        }

        // Place bomb neighbors
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.coord_to_index((x, y)).unwrap();
                if self.map[idx] == TileKind::Boom {
                    continue;
                }
                let num = self.bomb_count_at(idx);
                if num > 0 {
                    self.map[idx] =
                        TileKind::Danger(num.try_into().expect("more than 8 bombs around me? wow"));
                }
            }
        }
    }

    pub fn neighbors(&self, index: usize) -> Vec<((usize, usize), usize, TileKind)> {
        let around = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let (target_x, target_y) = self.index_to_coord(index).unwrap();

        around
            .into_iter()
            .filter_map(|(offset_x, offset_y)| {
                let x = match (target_x as isize).checked_add(offset_x) {
                    None => return None,
                    Some(x) if x as usize > self.width - 1 => return None,
                    Some(x) => x as usize,
                };
                let y = match (target_y as isize).checked_add(offset_y) {
                    None => return None,
                    Some(y) if y as usize > self.height - 1 => return None,
                    Some(y) => y as usize,
                };

                Some((x, y))
            })
            .map(|coords| {
                (
                    coords,
                    self.coord_to_index(coords).unwrap(),
                    self.at_coords(coords).unwrap(),
                )
            })
            .collect()
    }

    fn bomb_count_at(&self, index: usize) -> usize {
        self.neighbors(index)
            .iter()
            .filter(|(_, _, tile)| *tile == TileKind::Boom)
            .count()
    }
}

#[test]
fn test_index_to_coord() {
    let map = Map::new(8, 8);
    assert_eq!(map.index_to_coord(0), Some((0, 0)));
    assert_eq!(map.index_to_coord(1), Some((0, 1)));
    assert_eq!(map.index_to_coord(8), Some((1, 0)));
    assert_eq!(map.index_to_coord(63), Some((7, 7)));
}

#[test]
fn test_coord_to_index() {
    let map = Map::new(8, 8);
    assert_eq!(map.coord_to_index((0, 0)), Some(0));
    assert_eq!(map.coord_to_index((0, 1)), Some(1));
    assert_eq!(map.coord_to_index((1, 0)), Some(8));
    assert_eq!(map.coord_to_index((7, 7)), Some(63));
}

#[test]
fn test_map() {
    let mut map = Map::new(8, 8);
    map.set_bombs(16);
    assert!(
        map.map
            .iter()
            .filter(|x| matches!(x, TileKind::Danger(_)))
            .count()
            > 0
    );
}
