use postgres_flatten::{flattened::ToFlattenedSql, ToFlattenedSql};

#[derive(ToFlattenedSql)]
struct Cat {
    name: String,
    age: u8,
    color: u8,
    friendliness: i8,
}

fn main() {}
