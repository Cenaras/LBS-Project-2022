use lattice_contract_macro::gen_lattice_contract;
use lattice_macro::lattice_address;
use lattice_macro::level;
// ### LATTICE CONTRACT ###

fn my_order(a1: &String, a2: &String) -> bool {
    match (a1.as_str(), a2.as_str()) {
        ("bot", _) => true,
        ("player", "bot") => false,
        ("dealer", "bot") => false,
        ("player", "dealer") => false,
        ("dealer", "player") => false,
        ("player", _) => true,
        ("dealer", _) => true,
        ("top", "contract") => false,
        (_, "contract") => true,
        (_, "top") => true,
        _ => panic!("Element not in lattice"),
    }
}

gen_lattice_contract!(
    vec![
        String::from("top"),
        String::from("contract"),
        String::from("player"),
        String::from("dealer"),
        String::from("bot"),
    ],
    my_order,
    &String::from("contract")
);

// ### LATTICE CONTRACT END ###

// Example of binding a contract to a lattice contract. Macro is applied to the state struct
#[lattice_address("0x01")]
pub struct EnterCasinoContract {
    pub test: u32,
}

impl EnterCasinoContract {
    #[level("bot")]
    pub fn enter_casino(&self, amount: u32) {
        assert!(amount > 1000);
        self.set_level(caller_address, &String::from("player"))
    }

    // Require that "contract" flows to caller level
    #[level("contract")]
    pub fn hire_person(&self, person_address: &String) {
        self.set_level(person_address, &String::from("dealer"))
    }
}

#[lattice_address("0x01")]
pub struct RouletteContract {
    pub bids: HashMap<Address, NumberBid>,
}

pub struct NumberBid {
    pub number: u8,
    pub amount: u32,
}

impl RouletteContract {
    #[level("player")]
    pub fn bid_on_number(&mut self, number: u8, amount: u32) {
        let bid = NumberBid { number, amount };
        // Insert (unused) bid - this is just placeholder code, the important parts are the annotations
        self.bids.insert(caller_address.to_string(), bid);
    }

    #[level("dealer")]
    pub fn spin_wheel(&mut self) {
        println!("Spinning wheel, paying winners!");
        // Bids cleared after each round
        self.bids.clear();
    }

    #[level("contract")]
    pub fn close_roulette(&self) {
        println!("Roulette was closed")
    }
}
#[lattice_address("0x01")]
pub struct BlackjackContract {}

impl BlackjackContract {
    #[level("player")]
    pub fn play_blackjack(&self) {
        println!("Playing blackjack and paying winners")
    }

    #[level("top")]
    pub fn terminate_blackjack_contract(&self) {
        println!("Only top can terminate me!");
    }
}
