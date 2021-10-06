use tuple_type::TypeAt;

#[rustfmt::skip]
fn main() {
    type TestTuple = (i8, (i16, (i32, (i64,))));

    let _: i8 = <TestTuple as TypeAt<0>>::Type::default();
    let _: (i16, (i32, (i64,))) = <TestTuple as TypeAt<1>>::Type::default();
    let _: i16 = <<TestTuple
        as TypeAt<1>>::Type
        as TypeAt<0>>::Type::default();
    let _: (i32, (i64,)) = <<TestTuple
        as TypeAt<1>>::Type
        as TypeAt<1>>::Type::default();
    let _: i32 = <<<TestTuple
        as TypeAt<1>>::Type
        as TypeAt<1>>::Type
        as TypeAt<0>>::Type::default();
    let _: (i64,) = <<<TestTuple
        as TypeAt<1>>::Type
        as TypeAt<1>>::Type
        as TypeAt<1>>::Type::default();
    let _: i64 = <<<<TestTuple
        as TypeAt<1>>::Type
        as TypeAt<1>>::Type
        as TypeAt<1>>::Type
        as TypeAt<0>>::Type::default();
}
