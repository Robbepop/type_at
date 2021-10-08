use type_at::TypeAt;

#[derive(TypeAt)]
pub struct TupleStruct(i8, i16);

fn main() {
    let _: _  = <TupleStruct as TypeAt<2>>::Type::default();
}
