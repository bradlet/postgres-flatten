use postgres_flatten::{flattened::ToFlattenedSql, ToFlattenedSql};

#[allow(dead_code)]
struct Health {
    age: u8,
}

#[allow(dead_code)]
#[derive(ToFlattenedSql)]
struct Dog {
    name: String,
    health: Health,
}

fn main() {}
