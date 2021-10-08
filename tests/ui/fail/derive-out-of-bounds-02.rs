use type_at::TypeAt;

#[derive(TypeAt)]
pub struct Struct { a: i8, b: i16 }

fn main() {
    let _: _  = <Struct as TypeAt<2>>::Type::default();
}
