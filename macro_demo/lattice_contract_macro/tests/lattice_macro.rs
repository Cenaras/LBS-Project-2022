use lattice_contract_macro::*;




#[test]
fn macro_test() {
    gen_lattice_contract!(1, 2, 3);
    deploy_contract();
    raise_level();

}


