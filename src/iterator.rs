use std::slice;

use crate::Array;

pub struct Iter<I: Iterator, const D: usize> {
    pub(crate) size: [usize; D],
    pub(crate) ptr: [usize; D],
    pub(crate) internal_iter: I,
}

impl<'a, T, const D: usize> Iter<slice::Iter<'a, T>, D> {
    pub(crate) fn new(array: &'a Array<T, D>) -> Self {
        Self {
            size: array.size,
            ptr: [0; D],
            internal_iter: array.data.iter(),
        }
    }
}

impl<'a, T, const D: usize> Iter<slice::IterMut<'a, T>, D> {
    pub(crate) fn new_mut(array: &'a mut Array<T, D>) -> Self {
        Self {
            size: array.size,
            ptr: [0; D],
            internal_iter: array.data.iter_mut(),
        }
    }
}

impl<I: Iterator, const D: usize> Iter<I, D> {
    #[inline]
    fn increment_ptr(&mut self) {
        // propagate change
        #[allow(clippy::needless_range_loop)] // clippy bug
        for n in 0..D {
            self.ptr[n] += 1;
            if self.ptr[n] == self.size[n] {
                self.ptr[n] = 0;
            } else {
                break;
            }
        }
    }
}

impl<I: Iterator, const D: usize> Iterator for Iter<I, D> {
    type Item = ([usize; D], I::Item);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.internal_iter
            .next()
            .map(|x| {
                let r = Some((self.ptr, x));
                self.increment_ptr();
                r
            })
            .unwrap_or(None)
    }
}
