use lattice_contract_macro::gen_lattice_contract;
use lattice_macro::lattice_address;

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

struct Blockchain {
    lattice_contract: MyLatticeContract
}

impl Blockchain {
    pub fn get_lattice_contract(&self, address: &Address) -> &mut MyLatticeContract {
        self.lattice_contract.as_mut()
    }
}

#[lattice_address("0x01")]
struct State {
    blockchain: Blockchain,
    address: String,
    test: u32,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
