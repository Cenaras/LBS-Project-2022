use lattice_contract_macro::gen_lattice_contract;
use lattice_macro::lattice_address;
use lattice_macro::level;

fn my_order(a1: &String, a2: &String) -> bool {
    match (a1.as_str(), a2.as_str()) {
        ("bot", _) => true,
        ("allowed", "bot") => false,
        ("allowed", _) => true,
        ("top", "contract") => false,
        (_, "contract") => true,
        (_, "top") => true,
        _ => false,
    }
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

//#[lattice_address("0x01")]
struct Contract {
    address: String,
    test: u32,
}

impl Contract {
    #[level("allowed")]
    pub fn test(&self, lattice_contracts: &mut HashMap<String, MyLatticeContract>, caller_address: &String) {
        println!("Was allowed to call me")
    }
}

impl Contract {
    fn raise_level(&self, lattice_contracts: &mut HashMap<String, MyLatticeContract>, address: &String, level: &String) {
        // blockchain_call simulates a blockchain transaction to the entity present at its argument
        let addr = &String::from("0x01");
        lattice_contracts.get_mut(addr).unwrap().raise_level(&self.address, address, level);
    }

    fn le(&self, lattice_contracts: &mut HashMap<String, MyLatticeContract>, level1: &String, level2: &String) -> bool {
        let addr = &String::from("0x01");
        lattice_contracts.get_mut(addr).unwrap().le(level1, level2)
    }

    fn get_level(&self, lattice_contracts: &mut HashMap<String, MyLatticeContract>, address: &String) -> String {
        let addr = &String::from("0x01");
        lattice_contracts.get_mut(addr).unwrap().get_level(address)
    }
}

struct Blockchain {
    lattice_contracts: HashMap<String, MyLatticeContract>,
    contract: Contract,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut map: HashMap<String, MyLatticeContract> = HashMap::new();

        let lattice_contract = MyLatticeContract::new(String::from("0x02"));
        map.insert(String::from("0x01"), lattice_contract);
        let contract = Contract {address: String::from("0x03"), test: 14 };
        
        map.get_mut(&String::from("0x01")).unwrap().raise_level(&String::from("0x02"),  &String::from("0x03"), &String::from("contract"));
        contract.raise_level(&mut map, &String::from("0x04"), &String::from("allowed"));
        contract.test(&mut map, &String::from("0x04"));
        contract.test(&mut map, &String::from("0x06"));

    }
}
