//! into_flattened.rs
//! Author: bradlet

pub trait ToFlattenedSql {
    fn into_flattened_row();
}

pub trait FromFlattenedSql {
    fn from_flattened_row(row: &postgres::Row) -> Self;
}

pub trait Flattenable {
    fn flatten() -> Vec<syn::Field>;
}
