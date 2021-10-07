use type_at::TypeAt;

fn main() {
    let _: _  = <(i8,) as TypeAt<1>>::Type::default();
}
