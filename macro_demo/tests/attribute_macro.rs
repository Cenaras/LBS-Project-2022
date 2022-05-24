use macro_demo::*;

#[my_custom_attribute(A)]
struct S{
    test: u32,
}

#[test]
fn test_macro() {
    let demo = A{};
}
