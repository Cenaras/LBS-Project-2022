use lattice_contract_macro::*;

#[test]
fn macro_test() {
    fn order(a1: String, a2: String) -> bool {
        match (a1.as_str(), a2.as_str()) {
            ("bot", _) => true,
            ("allowed", "bot") => false,
            ("allowed", _) => true,
            ("top", "contract") => false,
            (_, "contract") => true,
            (_, "top") => true,
            _ => false
        }
    }

    gen_lattice_contract!(MyLatticeContract,
        vec![String::from("top"), String::from("contract"), String::from("allowed"), String::from("bot")],
        order,
        String::from("contract"), String::from("top"));
    
    let mut lattice_contract = MyLatticeContract::new(String::from("0x02"));
    lattice_contract.deploy_contract();
    lattice_contract.raise_level();
    assert!(lattice_contract.le(String::from("contract"), String::from("contract")));
    
}
