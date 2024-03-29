use std::slice;

use crate::Array;
// trickery!!
use crate::array::Allocator;

pub struct Iter<I: Iterator, const D: usize> {
    pub(crate) size: [usize; D],
    pub(crate) ptr: [usize; D],
    pub(crate) internal_iter: I,
}

impl<'a, T, const D: usize> Iter<slice::Iter<'a, T>, D> {
    pub(crate) fn new<A: Allocator>(array: &'a Array<T, D, A>) -> Self {
        Self {
            size: array.size,
            ptr: [0; D],
            internal_iter: array.data.iter(),
        }
    }
}

impl<'a, T, const D: usize> Iter<slice::IterMut<'a, T>, D> {
    pub(crate) fn new_mut<A: Allocator>(array: &'a mut Array<T, D, A>) -> Self {
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
        // match common lengths
        match D {
            0 => panic!("invalid array dimensions: 0"),
            1 => self.ptr[0] += 1,
            2 => {
                self.ptr[0] += 1;
                if self.ptr[0] == self.size[0] {
                    self.ptr[0] = 0;
                    self.ptr[1] += 1;
                }
            }
            3 => {
                self.ptr[0] += 1;
                if self.ptr[0] != self.size[0] {
                    return;
                }
                self.ptr[0] = 0;
                self.ptr[1] += 1;
                if self.ptr[1] != self.size[1] {
                    return;
                }
                self.ptr[1] = 0;
                self.ptr[2] += 1;
            }
            _ => {
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
