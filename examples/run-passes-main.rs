use postgres_flatten::{
    flattened::FromFlattenedSql, flattened::ToFlattenedSql, FromFlattenedSql, ToFlattenedSql,
};

#[derive(ToFlattenedSql, FromFlattenedSql)]
struct Cat {
    name: String,
    // age: u8,
    // color: u8,
    // friendliness: i8,
}

fn main() {}
