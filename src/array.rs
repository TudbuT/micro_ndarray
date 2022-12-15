use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Array<T, const D: usize> {
    size: [usize; D],
    data: Vec<T>,
}

impl<T: Default + Clone, const D: usize> Array<T, D> {
    pub fn new(size: [usize; D]) -> Self {
        let mut l = 1;
        for dim in size {
            l *= dim;
        }
        Self {
            size,
            data: vec![T::default(); l],
        }
    }
}

impl<T: Default + Clone, const D: usize> Array<T, D> {
    pub fn new_with(size: [usize; D], item: T) -> Self {
        let mut l = 1;
        for dim in size {
            l *= dim;
        }
        let mut r = Self {
            size,
            data: Vec::new(),
        };
        for _ in 0..l {
            r.data.push(item.clone());
        }
        r
    }
}

impl<'a, T, const D: usize> Array<T, D> {
    pub fn new_by<F: Fn() -> T>(size: [usize; D], supplier: F) -> Self {
        let mut l = 1;
        for dim in size {
            l *= dim;
        }
        let mut r = Self {
            size,
            data: Vec::new(),
        };
        for _ in 0..l {
            r.data.push(supplier());
        }
        r
    }

    pub fn size(&self) -> [usize; D] {
        self.size
    }

    pub fn get_mut(&'a mut self, loc: [usize; D]) -> Option<&'a mut T> {
        let mut real_loc = 0;
        for (i, dim) in loc.iter().enumerate() {
            if i == 0 {
                real_loc += dim;
                continue;
            }
            real_loc += dim * self.size[i - 1];
        }
        self.data.get_mut(real_loc)
    }

    pub fn get(&'a self, loc: [usize; D]) -> Option<&'a T> {
        let mut real_loc = 0;
        for (i, dim) in loc.iter().enumerate() {
            if i == 0 {
                real_loc += dim;
                continue;
            }
            real_loc += dim * self.size[i - 1];
        }
        self.data.get(real_loc)
    }

    pub fn iterable(&self) -> Vec<([usize; D], &T)> {
        self.data
            .iter()
            .enumerate()
            .map(|(mut idx, x)| {
                let mut loc = [0usize; D];
                for (i, dim) in self.size.iter().enumerate() {
                    loc[i] = idx % dim;
                    idx /= dim;
                }
                (loc, x)
            })
            .collect::<Vec<_>>()
    }

    pub fn iterable_mut(&mut self) -> Vec<([usize; D], &mut T)> {
        self.data
            .iter_mut()
            .enumerate()
            .map(|(mut idx, x)| {
                let mut loc = [0usize; D];
                for (i, dim) in self.size.iter().enumerate() {
                    loc[i] = idx % dim;
                    idx /= dim;
                }
                (loc, x)
            })
            .collect::<Vec<_>>()
    }
}

impl<T, const D: usize> Index<[usize; D]> for Array<T, D> {
    type Output = T;

    fn index(&self, index: [usize; D]) -> &Self::Output {
        for (i, dim) in index.iter().enumerate() {
            if *dim >= self.size[i] {
                panic!(
                    "Array index of dimension {} is out of bounds! 0..{}.contains({}) == false",
                    i + 1,
                    self.size[i],
                    dim
                )
            }
        }
        self.get(index).unwrap() // SAFETY this is checked in the previous lines
    }
}

impl<T, const D: usize> IndexMut<[usize; D]> for Array<T, D> {
    fn index_mut(&mut self, index: [usize; D]) -> &mut T {
        for (i, dim) in index.iter().enumerate() {
            if *dim >= self.size[i] {
                panic!(
                    "Array index of dimension {} is out of bounds! 0..{}.contains({}) == false",
                    i + 1,
                    self.size[i],
                    dim
                )
            }
        }
        self.get_mut(index).unwrap() // SAFETY this is checked in the previous lines
    }
}

#[cfg(test)]
mod test {
    use crate::Array;

    #[test]
    fn iterator() {
        let mut array = Array::new_with([5, 4], 0);
        array
            .iterable_mut()
            .into_iter()
            .filter(|(loc, _)| loc[0] == 0)
            .for_each(|x| {
                println!("{x:?}");
                *x.1 += x.0[1];
            });
        for y in 0..4 {
            for x in 0..5 {
                print!("{}", array[[x, y]]);
            }
            println!();
        }
        assert_eq!(
            array
                .iterable()
                .into_iter()
                .map(|x| *x.1)
                .collect::<Vec<_>>(),
            vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 0]
        )
    }
}
