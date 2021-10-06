# Type Indexing into Rust Tuples

Provides a trait `TypeAt` which allow to query the n-th type of a Rust tuple at compile time.

## Example: Simple

```rust
use tuple_type::TypeAt;

let _: i8  = <(i8, i16, i32, i64) as TypeAt<0>>::Type::default();
let _: i16 = <(i8, i16, i32, i64) as TypeAt<1>>::Type::default();
let _: i32 = <(i8, i16, i32, i64) as TypeAt<2>>::Type::default();
let _: i64 = <(i8, i16, i32, i64) as TypeAt<3>>::Type::default();
```

## Example: Nested

```rust
use tuple_type::TypeAt;

let _: i64 = <<<<(i8, (i16, (i32, (i64,))))
    as TypeAt<1>>::Type // (i16, (i32, (i64,)))
    as TypeAt<1>>::Type // (i32, (i64,))
    as TypeAt<1>>::Type // (i64,)
    as TypeAt<0>>::Type::default();
```

## License

Licensed under either of

 * Apache license, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Dual licence: [![badge][license-mit-badge]](LICENSE-MIT) [![badge][license-apache-badge]](LICENSE-APACHE)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as below, without any
additional terms or conditions.
