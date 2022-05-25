use lattice_contract_macro::gen_lattice_contract;

pub fn my_order(a1: &String, a2: &String) -> bool {
    true
}

gen_lattice_contract!(
    MyLatticeContract,
    vec![
        String::from("top"),
        String::from("contract"),
        String::from("allowed"),
        String::from("bot"),
    ],
    my_order,
    &String::from("contract")
);

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
