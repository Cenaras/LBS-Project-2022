use lattice_macro::*;

#[lattice_address(0x01)]
struct State {
    test: u32,
}

// This inserts a print...
#[level(Player)]
fn test() {
    // if (S.le(stuff, stuff))
    println!("test");
}

#[test]
fn test_macro() {
    //let demo = H{};
    test();
}

#[test]
fn test_struct() {
    State::print();
}
