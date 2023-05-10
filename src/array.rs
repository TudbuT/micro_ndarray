use std::{
    ops::{Index, IndexMut},
    slice,
};

use crate::iterator::Iter;

#[derive(Clone)]
pub struct Array<T, const D: usize> {
    pub(crate) size: [usize; D],
    pub(crate) data: Vec<T>,
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
        Self {
            size,
            data: vec![item; l],
        }
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

    pub fn new_by_enumeration<F: Fn(usize) -> T>(size: [usize; D], supplier: F) -> Self {
        let mut l = 1;
        for dim in size {
            l *= dim;
        }
        let mut r = Self {
            size,
            data: Vec::new(),
        };
        for i in 0..l {
            r.data.push(supplier(i));
        }
        r
    }

    pub fn size(&self) -> [usize; D] {
        self.size
    }

    pub fn get(&'a self, loc: [usize; D]) -> Option<&'a T> {
        self.internal_get(loc, false)
    }

    #[inline]
    fn internal_get(&'a self, loc: [usize; D], panic: bool) -> Option<&'a T> {
        let mut real_loc = 0;
        for (i, dim) in loc.iter().enumerate() {
            let mut dim = *dim;
            if dim >= self.size[i] {
                if panic {
                    panic!(
                        "Array index of dimension {} is out of bounds! 0..{}.contains({}) == false",
                        i + 1,
                        self.size[i],
                        dim
                    )
                } else {
                    return None;
                }
            }
            if i == 0 {
                real_loc += dim;
                continue;
            }
            for s in &self.size[0..i] {
                dim *= s;
            }
            real_loc += dim;
        }
        unsafe {
            // SAFETY this is checked in the previous lines
            Some(self.data.get_unchecked(real_loc))
        }
    }

    pub unsafe fn get_unchecked(&'a self, loc: [usize; D]) -> &'a T {
        let mut real_loc = 0;
        for (i, dim) in loc.iter().enumerate() {
            let mut dim = *dim;
            if i == 0 {
                real_loc += dim;
                continue;
            }
            for s in &self.size[0..i] {
                dim *= s;
            }
            real_loc += dim;
        }
        self.data.get_unchecked(real_loc)
    }

    pub fn get_mut(&'a mut self, loc: [usize; D]) -> Option<&'a mut T> {
        self.internal_get_mut(loc, false)
    }

    #[inline]
    fn internal_get_mut(&'a mut self, loc: [usize; D], panic: bool) -> Option<&'a mut T> {
        let mut real_loc = 0;
        for (i, dim) in loc.iter().enumerate() {
            let mut dim = *dim;
            if dim >= self.size[i] {
                if panic {
                    panic!(
                        "Array index of dimension {} is out of bounds! 0..{}.contains({}) == false",
                        i + 1,
                        self.size[i],
                        dim
                    )
                } else {
                    return None;
                }
            }
            if i == 0 {
                real_loc += dim;
                continue;
            }
            for s in &self.size[0..i] {
                dim *= s;
            }
            real_loc += dim;
        }
        unsafe {
            // SAFETY this is checked in the previous lines
            Some(self.data.get_unchecked_mut(real_loc))
        }
    }

    pub unsafe fn get_unchecked_mut(&'a mut self, loc: [usize; D]) -> &'a mut T {
        let mut real_loc = 0;
        for (i, dim) in loc.iter().enumerate() {
            let mut dim = *dim;
            if i == 0 {
                real_loc += dim;
                continue;
            }
            for s in &self.size[0..i] {
                dim *= s;
            }
            real_loc += dim;
        }
        self.data.get_unchecked_mut(real_loc)
    }

    pub fn iter(&mut self) -> Iter<slice::Iter<T>, D> {
        Iter::new(self)
    }

    pub fn iter_mut(&mut self) -> Iter<slice::IterMut<T>, D> {
        Iter::new_mut(self)
    }
}

impl<T, const D: usize> Index<[usize; D]> for Array<T, D> {
    type Output = T;

    fn index(&self, index: [usize; D]) -> &Self::Output {
        // SAFETY this unwrap can not panic due to panic:true in the args of internal_get
        self.internal_get(index, true).unwrap()
    }
}

impl<T, const D: usize> IndexMut<[usize; D]> for Array<T, D> {
    fn index_mut(&mut self, index: [usize; D]) -> &mut T {
        // SAFETY this unwrap can not panic due to panic:true in the args of internal_get_mut
        self.internal_get_mut(index, true).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::Array;

    #[test]
    fn iterator() {
        let mut array = Array::new_with([5, 4], 0);
        array
            .iter_mut()
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
            array.iter().map(|x| *x.1).collect::<Vec<_>>(),
            vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 0]
        )
    }
}
