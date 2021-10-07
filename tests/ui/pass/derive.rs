use type_at::TypeAt;

#[rustfmt::skip]
fn main() {
    #[derive(TypeAt)]
    pub struct TupleStruct(
        i8, i16, i32, i64, i128, u8, u16, u32, u64, u128
    );

    let _: i8   = <TupleStruct as TypeAt<0>>::Type::default();
    let _: i16  = <TupleStruct as TypeAt<1>>::Type::default();
    let _: i32  = <TupleStruct as TypeAt<2>>::Type::default();
    let _: i64  = <TupleStruct as TypeAt<3>>::Type::default();
    let _: i128 = <TupleStruct as TypeAt<4>>::Type::default();
    let _: u8   = <TupleStruct as TypeAt<5>>::Type::default();
    let _: u16  = <TupleStruct as TypeAt<6>>::Type::default();
    let _: u32  = <TupleStruct as TypeAt<7>>::Type::default();
    let _: u64  = <TupleStruct as TypeAt<8>>::Type::default();
    let _: u128 = <TupleStruct as TypeAt<9>>::Type::default();

    #[derive(TypeAt)]
    pub struct NamedStruct {
        _1: i8,
        _2: i16,
        _3: i32,
        _4: i64,
        _5: i128,
        _6: u8,
        _7: u16,
        _8: u32,
        _9: u64,
        _10: u128
    }

    let _: i8   = <NamedStruct as TypeAt<0>>::Type::default();
    let _: i16  = <NamedStruct as TypeAt<1>>::Type::default();
    let _: i32  = <NamedStruct as TypeAt<2>>::Type::default();
    let _: i64  = <NamedStruct as TypeAt<3>>::Type::default();
    let _: i128 = <NamedStruct as TypeAt<4>>::Type::default();
    let _: u8   = <NamedStruct as TypeAt<5>>::Type::default();
    let _: u16  = <NamedStruct as TypeAt<6>>::Type::default();
    let _: u32  = <NamedStruct as TypeAt<7>>::Type::default();
    let _: u64  = <NamedStruct as TypeAt<8>>::Type::default();
    let _: u128 = <NamedStruct as TypeAt<9>>::Type::default();
}
