use std::{
    ops::{Index, IndexMut},
    slice,
};

use crate::iterator::Iter;

#[cfg(not(feature = "allocator"))]
mod no_alloc {
    use std::marker::PhantomData;

    pub trait Allocator {}
    pub struct Global;
    impl Allocator for Global {}

    #[derive(Clone)]
    pub struct Array<T, const D: usize, A: Allocator = Global> {
        pub(crate) size: [usize; D],
        pub(crate) stride: [usize; D],
        pub(crate) data: Vec<T>,
        phantom_a: PhantomData<A>,
    }

    impl<T: Default + Clone, const D: usize> Array<T, D> {
        pub fn new(size: [usize; D]) -> Self {
            Self::new_with(size, T::default())
        }
    }

    impl<T: Clone, const D: usize> Array<T, D> {
        pub fn new_with(size: [usize; D], item: T) -> Self {
            let mut l = 1;
            let mut stride = [0usize; D];
            for (i, dim) in size.into_iter().enumerate() {
                stride[i] = l;
                l *= dim;
            }
            Self {
                size,
                stride,
                data: vec![item; l],
                phantom_a: PhantomData,
            }
        }
    }
    impl<T, const D: usize> Array<T, D> {
        pub fn new_by<F: Fn() -> T>(size: [usize; D], supplier: F) -> Self {
            let mut l = 1;
            let mut stride = [0usize; D];
            for (i, dim) in size.into_iter().enumerate() {
                stride[i] = l;
                l *= dim;
            }
            let mut r = Self {
                size,
                stride,
                data: Vec::with_capacity(l),
                phantom_a: PhantomData,
            };
            for _ in 0..l {
                r.data.push(supplier());
            }
            r
        }

        pub fn new_by_enumeration<F: Fn(usize) -> T>(size: [usize; D], supplier: F) -> Self {
            let mut l = 1;
            let mut stride = [0usize; D];
            for (i, dim) in size.into_iter().enumerate() {
                stride[i] = l;
                l *= dim;
            }
            let mut r = Self {
                size,
                stride,
                data: Vec::with_capacity(l),
                phantom_a: PhantomData,
            };
            for i in 0..l {
                r.data.push(supplier(i));
            }
            r
        }

        /// Flattens the ND Array into a 1D Array with indexing `x + y * size_x + z * size_x * size_y` etc. This is a zero-cost operation.
        pub fn into_flattened(self) -> Vec<T> {
            self.data
        }

        /// Reinterprets a 1D array as an ND Array with indexing `x + y * size_x + z * size_x * size_y` etc. This is a zero-cost operation.
        pub fn from_flat(array: Vec<T>, size: [usize; D]) -> Option<Self> {
            let mut l = 1;
            let mut stride = [0usize; D];
            for (i, dim) in size.into_iter().enumerate() {
                stride[i] = l;
                l *= dim;
            }
            if l != array.len() {
                return None;
            }
            Some(Self {
                data: array,
                size,
                stride,
                phantom_a: PhantomData,
            })
        }
    }
}
#[cfg(not(feature = "allocator"))]
pub use no_alloc::*;
#[cfg(feature = "allocator")]
mod alloc {
    macro_rules! insert_functions {
        // matches a trait constraints declaration followed by a function definition with one argument
        // separated using ;. this argument is the allocator.
        (
            // type constraints for T
            $($t:ident),*:
            // declaration
            pub fn $name:tt
                // type args
                $( < $($targs_t:tt $(: $targs_constraint:tt)?),+ > )?
                // args
                ( $($arg_name:ident : $arg_type:ty),*; $alloc:ident: A )
                // return type
                -> $ret:tt
            // code
            $block:tt
        ) => {
            // implements for ANY allocator
            impl<T: $($t + )*, const D: usize, A: Allocator> Array<T, D, A> {
                // the function that was given as input. uses `replace!` to do $name_in, which isnt possible normally.
                ::ident_concat::replace!{p_in $name _in:
                    pub fn p_in $( < $($targs_t $(: $targs_constraint)?, )* > )?
                        ( $($arg_name: $arg_type, )* $alloc: A ) -> $ret
                    $block
                }
            }
            // implements for the global allocator
            impl<T: $($t + )*, const D: usize> Array<T, D, Global> {
                // the function that was given as input, but without the alloc argument. used to default to global allocator.
                // once again, `replace!` is used to use $name_in.
                pub fn $name $( < $($targs_t $(: $targs_constraint)?, )* > )?
                    ( $($arg_name: $arg_type, )* ) -> $ret
                {
                    ::ident_concat::replace!(p_in $name _in: Self::p_in)($($arg_name,)* Global)
                }
            }
        };
    }

    pub use std::alloc::Allocator;
    use std::{alloc::Global, vec};

    #[derive(Clone)]
    pub struct Array<T, const D: usize, A: Allocator = Global> {
        pub(crate) size: [usize; D],
        pub(crate) stride: [usize; D],
        pub(crate) data: Vec<T, A>,
    }

    insert_functions!(Clone, Default: pub fn new(size: [usize; D]; alloc: A) -> Self {
        Self::new_with_in(size, T::default(), alloc)
    });

    insert_functions!(Clone: pub fn new_with(size: [usize; D], item: T; alloc: A) -> Self {
        let mut l = 1;
        let mut stride = [0usize; D];
        for (i, dim) in size.into_iter().enumerate() {
            stride[i] = l;
            l *= dim;
        }
        Self {
            size,
            stride,
            data: vec::from_elem_in(item, l, alloc),
        }
    });

    insert_functions!(: pub fn new_by<F: (Fn() -> T)>(size: [usize; D], supplier: F; alloc: A) -> Self {
        let mut l = 1;
        let mut stride = [0usize; D];
        for (i, dim) in size.into_iter().enumerate() {
            stride[i] = l;
            l *= dim;
        }
        let mut r = Self {
            size,
            stride,
            data: Vec::with_capacity_in(l, alloc),
        };
        for _ in 0..l {
            r.data.push(supplier());
        }
        r
    });

    insert_functions!(: pub fn new_by_enumeration<F: (Fn(usize) -> T)>(size: [usize; D], supplier: F; alloc: A) -> Self {
        let mut l = 1;
        let mut stride = [0usize; D];
        for (i, dim) in size.into_iter().enumerate() {
            stride[i] = l;
            l *= dim;
        }
        let mut r = Self {
            size,
            stride,
            data: Vec::with_capacity_in(l, alloc),
        };
        for i in 0..l {
            r.data.push(supplier(i));
        }
        r
    });

    impl<'a, T, const D: usize, A: Allocator> Array<T, D, A> {
        /// Flattens the ND Array into a 1D Array with indexing `x + y * size_x + z * size_x * size_y` etc. This is a zero-cost operation.
        pub fn into_flattened(self) -> Vec<T, A> {
            self.data
        }

        /// Reinterprets a 1D array as an ND Array with indexing `x + y * size_x + z * size_x * size_y` etc. This is a zero-cost operation.
        pub fn from_flat(array: Vec<T, A>, size: [usize; D]) -> Option<Self> {
            let mut l = 1;
            let mut stride = [0usize; D];
            for (i, dim) in size.into_iter().enumerate() {
                stride[i] = l;
                l *= dim;
            }
            if l != array.len() {
                return None;
            }
            Some(Self {
                data: array,
                size,
                stride,
            })
        }
    }
}
#[cfg(feature = "allocator")]
pub use alloc::*;

// "Allocator" in all future uses refers to the re-exported or the placeholder.

impl<'a, T, const D: usize, A: Allocator> Array<T, D, A> {
    pub fn size(&self) -> [usize; D] {
        self.size
    }

    pub fn get(&'a self, loc: [usize; D]) -> Option<&'a T> {
        self.internal_get(loc, false)
    }

    #[inline]
    fn internal_get(&'a self, loc: [usize; D], panic: bool) -> Option<&'a T> {
        let mut real_loc = 0;
        for (i, &(mut dim)) in loc.iter().enumerate() {
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
            dim *= self.stride[i];
            real_loc += dim;
        }
        unsafe {
            // SAFETY this is checked in the previous lines
            Some(self.data.get_unchecked(real_loc))
        }
    }

    pub unsafe fn get_unchecked(&'a self, loc: [usize; D]) -> &'a T {
        let mut real_loc = 0;
        for (i, &(mut dim)) in loc.iter().enumerate() {
            if i == 0 {
                real_loc += dim;
                continue;
            }
            dim *= self.stride[i];
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
        for (i, &(mut dim)) in loc.iter().enumerate() {
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
            dim *= self.stride[i];
            real_loc += dim;
        }
        unsafe {
            // SAFETY this is checked in the previous lines
            Some(self.data.get_unchecked_mut(real_loc))
        }
    }

    pub unsafe fn get_unchecked_mut(&'a mut self, loc: [usize; D]) -> &'a mut T {
        let mut real_loc = 0;
        for (i, &(mut dim)) in loc.iter().enumerate() {
            if i == 0 {
                real_loc += dim;
                continue;
            }
            dim *= self.stride[i];
            real_loc += dim;
        }
        self.data.get_unchecked_mut(real_loc)
    }

    pub fn iter(&self) -> Iter<slice::Iter<T>, D> {
        Iter::new(self)
    }

    pub fn iter_mut(&mut self) -> Iter<slice::IterMut<T>, D> {
        Iter::new_mut(self)
    }

    /// Flattens the ND Array into a 1D Array with indexing `x + y * size_x + z * size_x * size_y` etc. This is a zero-cost operation.
    pub fn as_flattened(&self) -> &[T] {
        self.data.as_slice()
    }

    /// Flattens the ND Array into a 1D Array with indexing `x + y * size_x + z * size_x * size_y` etc. This is a zero-cost operation.
    pub fn as_flattened_mut(&mut self) -> &mut [T] {
        self.data.as_mut_slice()
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
