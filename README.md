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

## Project

This library was implemented for CS510 - Programming in Rust, taught by Bart Massey at
Portland State University (Spring 2023).

[See the write-up below for more details](#write-up)

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

## Write Up

### What worked?

This project was a good introduction into developing procedural macros. I was able to learn
how to structure the project such that the procedural macros could share a name with the
derivable trait. This is only sort-of 'hard' when you want it to live within the same
repository; accomplished using a decently common pattern in procedural macro development,
using workspaces.

Testing the code out went well. Outside of compilation tests using [trybuild](https://docs.rs/trybuild/latest/trybuild/),
I included an integration test that uses a locally running postgres docker container.
This test will be ignored by default because the connection setup is dependent on the
database connection existing, as well as providing an updated connection string. I was able
to first play around with the code in my [personal practice repo](https://github.com/bradlet/bradlet-rust-practice),
then eventually moved onto creating `axum_macro_example` which gave me exposure into Rust
web service development.

Setting up `FromFlattenedSql` went pretty well early on. I was able to get it to the point
that I was parsing `Row` objects easily so long as no flattening needed to occur. The
"flattening" piece is where I ran into problems.

### What did not work?

When it came to the "flattening" I quickly ran into issues and began to understand that
this was a pretty hard project. Bart Massey gave me the advice to focus on deriving a
schema for the data, but the `postgres` library that I used doesn't seem to have any
concept like that. There are ORM's written ontop of it that have that, but the goal
of my project was to essentially provide a light-weight way to declare the structure of
query results and parse them succinctly. I didn't want to get into developing an ORM.
There was no way (obvious to me) to create a schema that could then be mapped,
either at compile or run-time, to the nested-structure schema.

Because the problem is essentially needing a way to recursively search through the
struct tokens, exploding struct fields into multiple fields, and then `get`ing the
values from the `Row`, before constructing an object, I began to work on `Flattenable`.
The goal of this trait and derive macro pair was to provide the aforementioned
"explosion" mechanism. This ate up a _lot_ of development time, as there were many
pitfalls. For one, in the other langauges (which have runtime reflection) that I've
implemented this type of functionality, there were ways to easily tell if a feild was
a primitive type, or some complex type. Perhaps that could still be possible with
a better `TokenStream` parsing API in Rust; however, using `Syn` all field types
were simply `Path` objects. So I had to do a lot of research before happening upon
the [Little Book of Rust Macros](https://veykril.github.io/tlborm/) which mentioned
derive macro "helper attributes". I decided that clients of the library could
target the fields which needed to be flattened with the `#[flattenable]` helper
attribute. With that, I was able to figure out how to at least create a "flatten"
method implementation which would return a vector of `Field` objects describing
the struct. The helper attribute would not even be that bad because a desired end
goal for the project would be to make this work with protobuf messages, and protoc
basically be instructed to include the attribute when building out types for the
messages -- including the attribute for any message fields. I finished the
implementation that is currently in `impl_flattenable.rs` when I realized that it
required runtime execution to build out the result vector, which means it probably
would not work, and I also realized that, because `flatten` needs to be called at
runtime, there isn't really a way to use the vector of fields in my other macros
(`FromFlattenedSql`, namely).

At this point, I was pretty stuck and had already spent a lot of time on this project.
I realize now that there probably needs to be a pretty fundamental change in approach
to implement the desired functionality.

### How satisfied are you with the result?

I am satisfied in the fact that I learned a lot more about procedural macros.
I'm also happy that I got more comfortable with Rust through the complexity of
this project and thinking through what the code would look like, and need to do,
at compile time. I'm also happy that I was able to at least create a library that
could be paired with a web service library to cleanup CRUD API development a bit.

I am not so satisfied with the fact that I had to significantly cut the scope of
the MVP, and could not deliver the desired functionality. My past experience was
misleading; I figured it would be more difficult, but I did not expect it to be
as archaeic to work with `TokenStream`s. On top of that, there are almost no
resources online to really help with procedural macro development, so a lot of it
just comes from trial and error, and a few general resources without good specific
examples.

In the future, I'll probably avoid choosing a final project on a very complex
language feature that is not even really on the lesson plan for the class I'm
taking (lol).

### What would you like to improve in the future?

I'm definitely going to keep working on this library. I'm more convinced now that
the functionality doesn't exist in the ecosystem yet. You can get more 'fancy' if
you use some ORM library, but ORM's are a lot of hassle to setup and maintain. I've
had several projects where I just want to use a lightweight sql client, but I also
don't want to have to hand implement a mapping-method for each query that I write.
I think it would be nice to provide this library, in full form, for projects in the
early stages of their development cycle. More advanced data modeling can come with
time, but there's a lot of benefit to simply saying "screw it" and just rolling with
a flat table structure when you are trying to just bust out some feature, and want
to worry about refactoring towards some better model in the future.

I'll likely need to look into implementing some generic implementations for mapping
flat rust structs to nested rust structs, and then look into making the procedural
macro create the flat rust struct from the `Row` result. The rest of the library can
then provide method implementations that hide the flat intermediary representation
and simply go from `Row` -> some nested rust struct.

To be honest, if I wasn't so busy with work now, I'd probably just continously be
working on this until it was right. It was a lot of fun to learn and work on. But
I already spent so much time on it just to get it to this point. The end goal is 
to have both `ToFlattenedSql` and `FromFlattenedSql` fully implemented, and I will
keep working on it until that is working. I really want to stick to it b/c I think 
it would be a fun project to have on my Github (lol).
