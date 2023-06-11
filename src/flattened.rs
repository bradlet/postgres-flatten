//! into_flattened.rs
//! Author: bradlet

// use postgres::Row;

pub trait ToFlattenedSql {
    fn into_flattened_row();

    fn default() -> Self;
}

pub trait FromFlattenedSql {
    fn from_flattened_row() -> Self;
}
