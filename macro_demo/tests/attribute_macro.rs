use macro_demo::*;

#[my_custom_attribute(B)]
struct S{}

#[test]
fn test_macro() {
    let demo = H{};
}