#![no_implicit_prelude]

use ::core::default::Default;

fn main() {
    #[derive(::type_at::TypeAt)]
    pub struct TupleStruct(
        ::core::primitive::i8,
        ::core::primitive::i16,
        ::core::primitive::i32,
        ::core::primitive::i64,
    );

    let _: ::core::primitive::i8 =
        <TupleStruct as ::type_at::TypeAt<0usize>>::Type::default();
    let _: ::core::primitive::i16 =
        <TupleStruct as ::type_at::TypeAt<1usize>>::Type::default();
    let _: ::core::primitive::i32 =
        <TupleStruct as ::type_at::TypeAt<2usize>>::Type::default();
    let _: ::core::primitive::i64 =
        <TupleStruct as ::type_at::TypeAt<3usize>>::Type::default();

    #[derive(::type_at::TypeAt)]
    pub struct NamedStruct {
        _1: ::core::primitive::i8,
        _2: ::core::primitive::i16,
        _3: ::core::primitive::i32,
        _4: ::core::primitive::i64,
    }

    let _: ::core::primitive::i8 =
        <NamedStruct as ::type_at::TypeAt<0usize>>::Type::default();
    let _: ::core::primitive::i16 =
        <NamedStruct as ::type_at::TypeAt<1usize>>::Type::default();
    let _: ::core::primitive::i32 =
        <NamedStruct as ::type_at::TypeAt<2usize>>::Type::default();
    let _: ::core::primitive::i64 =
        <NamedStruct as ::type_at::TypeAt<3usize>>::Type::default();
}
