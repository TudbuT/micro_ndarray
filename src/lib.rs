#![cfg_attr(feature = "allocator", feature(allocator_api))]

mod array;
mod iterator;
#[cfg(feature = "vec_split")]
/// micro_ndarray has been compiled with support for vec_split.
/// This means you can use vec_split's tools on this crate's
/// [`Array`] struct. vec_split has been re-exported.
mod vec_split_impl;
#[cfg(feature = "vec_split")]
pub use vec_split;

pub use array::Array;
