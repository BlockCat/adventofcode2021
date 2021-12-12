use std::collections::HashMap;

use crate::vector::Vector2;

pub trait Grid<T>
where
    T: Clone + PartialEq + Eq + Default,
{
    fn set(&mut self, x: usize, y: usize, value: T);
    fn set_vec(&mut self, pos: &Vector2, value: T) {
        self.set(pos[0] as usize, pos[1] as usize, value);
    }
    fn get_vec(&self, pos: &Vector2) -> Option<&T> {
        self.get(pos[0] as usize, pos[1] as usize)
    }
    fn get(&self, x: usize, y: usize) -> Option<&T>;

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T>;
    fn get_mut_vec(&mut self, pos: &Vector2) -> Option<&mut T>;
    fn get_insert_vec_mut(&mut self, pos: &Vector2, default: T) -> &mut T;
}

#[derive(Default, PartialEq, Eq, Clone)]
pub struct DynamicGrid<T>
where
    T: Clone + PartialEq + Eq + Default,
{
    pub map: HashMap<(usize, usize), T>,
}

impl<T> Grid<T> for DynamicGrid<T>
where
    T: Clone + PartialEq + Eq + Default,
{
    fn set(&mut self, x: usize, y: usize, value: T) {
        self.map.insert((x, y), value);
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.map.get(&(x, y))
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.map.get_mut(&(x, y))
    }

    fn get_mut_vec(&mut self, pos: &Vector2) -> Option<&mut T> {
        unimplemented!()
    }

    fn get_insert_vec_mut(&mut self, pos: &Vector2, default: T) -> &mut T {
        self.map
            .entry((pos[0] as usize, pos[1] as usize))
            .or_insert(default)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct StaticGrid<T>
where
    T: Clone + PartialEq + Eq + Default,
{
    pub grid: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> for StaticGrid<T>
where
    T: Clone + PartialEq + Eq + Default,
{
    fn set(&mut self, x: usize, y: usize, value: T) {
        self.grid[x + y * self.width] = value;
    }

    fn set_vec(&mut self, pos: &Vector2, value: T) {
        self.set(pos[0] as usize, pos[1] as usize, value);
    }

    fn get_vec(&self, pos: &Vector2) -> Option<&T> {
        self.get(pos[0] as usize, pos[1] as usize)
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            self.grid.get(x + y * self.width)
        } else {
            None
        }
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            self.grid.get_mut(x + y * self.width)
        } else {
            None
        }
    }
    fn get_mut_vec(&mut self, pos: &Vector2) -> Option<&mut T> {
       self.get_mut(pos[0] as usize, pos[1] as usize)
    }

    fn get_insert_vec_mut(&mut self, pos: &Vector2, _: T) -> &mut T {
        self.get_mut(pos[0] as usize, pos[1] as usize).unwrap()
    }
}

impl<T> StaticGrid<T>
where
    T: Clone + PartialEq + Eq + Default,
{
    pub fn new(width: usize, height: usize) -> StaticGrid<T> {
        StaticGrid {
            grid: vec![Default::default(); width * height],
            width,
            height,
        }
    }

    pub fn from_slice(width: usize, height: usize, slice: Vec<T>) -> StaticGrid<T> {
        StaticGrid {
            width,
            height,
            grid: slice,
        }
    }

    pub fn from_vec(grid: Vec<Vec<T>>) -> StaticGrid<T> {
        let width = grid[0].len();
        let height = grid.len();
        StaticGrid {
            grid: grid.into_iter().flatten().collect(),
            width,
            height,
        }
    }

    pub fn to_vec(&self) -> Vec<Vec<T>> {
        self.grid.chunks(self.width).map(|x| Vec::from(x)).collect()
    }

    pub fn iter<'a>(&'a self) -> GridIterator<'a, T> {
        GridIterator {
            grid: &self,
            x: 0,
            y: 0,
        }
    }
}

pub struct GridIterator<'a, T>
where
    T: Clone + PartialEq + Eq + Default,
{
    grid: &'a StaticGrid<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iterator for GridIterator<'a, T>
where
    T: Clone + PartialEq + Eq + Default,
{
    type Item = (Vector2, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.grid.get(self.x, self.y);

        if let Some(v) = val {
            let x = self.x;
            let y = self.y;

            self.x += 1;
            if self.x >= self.grid.width {
                self.x = 0;
                self.y += 1;
            }

            return Some((Vector2::new([x as isize, y as isize]), v));
        } else {
            return None;
        }
    }
}