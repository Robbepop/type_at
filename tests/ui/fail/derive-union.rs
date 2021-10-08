use type_at::TypeAt;

#[derive(TypeAt)]
pub union Union {
    a: i32,
    b: u32,
}

fn main() {}
