use postgres_flatten::{
    flattened::FromFlattenedSql, flattened::ToFlattenedSql, FromFlattenedSql, ToFlattenedSql,
};

#[allow(dead_code)]
#[derive(ToFlattenedSql, FromFlattenedSql)]
struct Cat {
    name: String,
    age: i8,
    color: i8,
    friendliness: i8,
}

fn main() {}
