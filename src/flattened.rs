//! into_flattened.rs
//! Author: bradlet

pub trait ToFlattenedSql {
    fn into_flattened_row();

    fn default() -> Self;
}
