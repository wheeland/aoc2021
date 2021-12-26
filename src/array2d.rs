use core::fmt::Debug;
use std::slice::Iter;
use std::slice::IterMut;

#[derive(Clone)]
pub struct Array2D<T: Default + Clone> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Default + Clone> Array2D<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![T::default(); width * height];
        Self {
            width,
            height,
            data,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn fill(&mut self, value: T) {
        for v in &mut self.data {
            *v = value.clone();
        }
    }

    pub fn at(&self, pos: (usize, usize)) -> &T {
        debug_assert!(pos.0 < self.width);
        debug_assert!(pos.1 < self.height);
        &self.data[pos.0 + self.width * pos.1]
    }

    pub fn at_mut(&mut self, pos: (usize, usize)) -> &mut T {
        debug_assert!(pos.0 < self.width);
        debug_assert!(pos.1 < self.height);
        &mut self.data[pos.0 + self.width * pos.1]
    }

    pub fn set(&mut self, pos: (usize, usize), value: T) {
        *self.at_mut(pos) = value;
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.data.iter_mut()
    }
}

impl<T: Default + Debug + Clone> Debug for Array2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        for y in 0..self.height {
            let c1 = y * self.width;
            let c2 = (y + 1) * self.width;
            let dbg: Vec<&T> = self.data[c1..c2].iter().collect();
            list.entry(&dbg);
        }
        list.finish()
    }
}
