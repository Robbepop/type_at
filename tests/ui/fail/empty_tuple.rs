use type_at::TypeAt;

fn main() {
    let _: _  = <() as TypeAt<0>>::Type::default();
}
