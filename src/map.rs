use std::ops::Rem;

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
                let idx = y * self.width + x;
                if matches!(self.map[idx], TileKind::Boom) {
                    continue;
                }
                let num = self.bomb_count_at(idx);
                if num > 0 {
                    self.map[idx] = TileKind::Danger(num);
                }
            }
        }
    }

    fn bomb_count_at(&self, index: usize) -> u8 {
        let around: Vec<(isize, isize)> = (-1isize..1)
            .zip(-1isize..1)
            .filter(|x| *x != (0, 0))
            .collect();
        dbg!(&around);
        let (target_x, target_y) = (
            (index % self.width) as isize,
            index.rem(self.width) as isize,
        );

        let mut count = 0u8;

        for (offset_x, offset_y) in around {
            let x = match target_x.checked_add(offset_x) {
                None => continue,
                Some(x) if x as usize > self.width => continue,
                Some(x) => x as usize,
            };
            let y = match target_y.checked_add(offset_y) {
                None => continue,
                Some(y) if y as usize > self.height => continue,
                Some(y) => y as usize,
            };

            if matches!(self.map[x * self.width + y], TileKind::Boom) {
                count = count.checked_add(1).unwrap();
            }
        }

        count
    }
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
