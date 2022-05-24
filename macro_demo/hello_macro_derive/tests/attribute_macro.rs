use hello_macro_derive::*;

/*
#[lattice_address(A)]
struct S{
    test: u32,
}*/

// This inserts a print...
#[trace_vars(Player)]
fn test() {
    // if (S.le(stuff, stuff))
    println!("test");
}

#[test]
fn test_macro() {
    //let demo = H{};
    test();
}

