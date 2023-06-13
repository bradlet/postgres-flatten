use postgres_flatten::{
    flattened::{Flattenable, ToFlattenedSql},
    Flattenable, ToFlattenedSql,
};

#[allow(dead_code)]
#[derive(Flattenable)]
struct Health {
    age: u8,
}

#[allow(dead_code)]
#[derive(Flattenable, ToFlattenedSql)]
struct Dog {
    name: String,
    #[flattenable]
    health: Health,
}

fn main() {}
