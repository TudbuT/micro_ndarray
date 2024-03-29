use vec_split::{
    accessors::{Accessor, AccessorMut},
    RawVector, SizedVectorArray, Vector, VectorArray,
};

use crate::Array;

impl<T, const D: usize, V: Vector<T, D>> VectorArray<T, D, V, [usize; D]> for Array<V, D> {
    fn get<'a>(&'a self, index: [usize; D]) -> Option<&'a V> {
        Array::get(self, index)
    }

    fn get_mut<'a>(&'a mut self, index: [usize; D]) -> Option<&'a mut V> {
        Array::get_mut(self, index)
    }
}

impl<T, const D: usize, V: RawVector<T, D>> SizedVectorArray<T, D, V, [usize; D]> for Array<V, D> {
    fn ptr(&self) -> *const V {
        self.data.as_ptr()
    }

    fn ptr_mut(&mut self) -> *mut V {
        self.data.as_mut_ptr()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    fn convert_index(&self, index: [usize; D]) -> usize {
        let mut real_loc = 0;
        for (i, dim) in index.iter().enumerate() {
            let mut dim = *dim;
            if i == 0 {
                real_loc += dim;
                continue;
            }
            dim *= self.stride[i];
            real_loc += dim;
        }
        real_loc
    }
}

impl<'a, T, const D: usize> Accessor<T, [usize; D]> for Array<T, D> {
    fn get<'b>(&'b self, index: [usize; D]) -> Option<&'b T> {
        <Array<T, D>>::get(self, index)
    }
}
impl<'a, T, const D: usize> AccessorMut<T, [usize; D]> for Array<T, D> {
    fn get_mut<'b>(&'b mut self, index: [usize; D]) -> Option<&'b mut T> {
        <Array<T, D>>::get_mut(self, index)
    }
}
