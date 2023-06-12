use postgres_flatten::{
    flattened::FromFlattenedSql, flattened::ToFlattenedSql, FromFlattenedSql, ToFlattenedSql,
};

#[allow(dead_code)]
#[derive(ToFlattenedSql, FromFlattenedSql)]
struct Cat {
    name: String,
    age: u8,
}

fn main() {}
