use type_at::TypeAt;

#[rustfmt::skip]
fn main() {
    type TestTuple = (i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

    let _: i8   = <TestTuple as TypeAt<0>>::Type::default();
    let _: i16  = <TestTuple as TypeAt<1>>::Type::default();
    let _: i32  = <TestTuple as TypeAt<2>>::Type::default();
    let _: i64  = <TestTuple as TypeAt<3>>::Type::default();
    let _: i128 = <TestTuple as TypeAt<4>>::Type::default();
    let _: u8   = <TestTuple as TypeAt<5>>::Type::default();
    let _: u16  = <TestTuple as TypeAt<6>>::Type::default();
    let _: u32  = <TestTuple as TypeAt<7>>::Type::default();
    let _: u64  = <TestTuple as TypeAt<8>>::Type::default();
    let _: u128 = <TestTuple as TypeAt<9>>::Type::default();
}
