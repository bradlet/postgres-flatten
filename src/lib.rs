// Copyright © 2023 Bradley Thompson
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! # postgres-flatten
//! A proc macro lib crate which provides the `ToFlattenedSql` and `FromFlattenedSql` traits,
//! as well as a derive macro to implement these traits on any type.

pub mod flattened;

pub use postgres_flatten_impl::*;
