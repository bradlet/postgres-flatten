# postgres-flatten

Copyright © 2023 Bradley Thompson — Version 0.1.0

~~A proc macro lib crate which provides the `ToFlattenedRow` and `FromFlattenedRow` traits,
as well as the `ToFlattenedSql` and `FromFlattenedSql` derive macros which will provide
default implementations for these traits.~~

A proc macro lib crate which provides the `FromFlattenedRow` trait and derive macro, which
doesn't actually flatten, but rather makes it easy to pull rows with flat schemas into
strongly-typed Rust struct objects.

Also includes the pieces of the (currently) failed attempt create a recursive
procedural macro.

## License

This program is licensed under the "MIT License". Please see the file LICENSE in the source
distribution of this software for license terms.

## Usage

See the [example](/examples/example.rs) to see how you can pull the derive macro into scope
as well as the derived trait.

Once derived, you'll be able to parse a row into a rust struct:

```rust
let parsed = SomeStruct::from_flattened_row(&row);
```

Outside of this example, a more in-depth example is provided
which shows how to use this macro to derive the `from_flattened_row` method for any simple
struct: [axum_macro_example](https://gitlab.cecs.pdx.edu/bradlet2/axum_macro_example)

Note, again, that at this time only `FromFlattenedSql` is implemented / "ready for use".

## Limitations

Though I'll be working on this project over time to get it working as originally intended,
its functionality is currently limited. `FromFlattenedSql` will parse a `tokio_postgres::Row`
into a rust struct so long as the field naming matches the column naming used in the postgres
table being queried. The typing for the struct must also match the supported type mappings
provided by the `FromSql` trait. If there is a type mismatch, a runtime error will call
out the type mismatch.
