use crate::vector::Vector2;

pub trait Grid<T>
where
    T: Clone + PartialEq + Eq + Default,
{
    fn set(&mut self, x: usize, y: usize, value: T);
    fn set_vec(&mut self, pos: &Vector2, value: T);
    fn get_vec(&self, pos: &Vector2) -> Option<&T>;
    fn get(&self, x: usize, y: usize) -> Option<&T>;
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T>;
}

#[derive(PartialEq, Eq, Clone, Hash)]
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
        self.grid.get_mut(x + y * self.width)
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
    type Item = ((usize, usize), &'a T);

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

            return Some(((x, y), v));
        } else {
            return None;
        }
    }
}
