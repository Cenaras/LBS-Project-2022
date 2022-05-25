use lattice_macro::*;




// This inserts a print...
#[level("allowed")]
fn test() {
    // if (S.le(stuff, stuff))
    println!("test");
}

#[test]
fn test_macro() {
    //let demo = H{};
    test();
}
/*
#[test]
fn test_struct() {
    State::print();
}
*/