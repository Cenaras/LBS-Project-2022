use blockchain_demo::*;
use std::collections::HashMap;

/*
    0x01 is the lattice contract
    0x02 is deployer of all contracts
    0x03 is address of EnterCasinoContract
    0x04 is address of RouletteContract
    0x05 is address of BlackjackContract
    0x06 is normal user, raised to "player"
    0x07 is normal user, raised to "dealer"

*/

#[test]
fn test_casino() {
    // This map is simply used to simulate the blockchain having lattice_contracts at addresses
    // replaces the rpc call needed on the blockchain to interact with the lattice contract
    let mut map: HashMap<String, LatticeContract> = HashMap::new();

    // Deploy lattice contract. It is put in a map to simulate blockchain addresses
    let lattice_contract = LatticeContract::new(String::from("0x02"));
    map.insert(String::from("0x01"), lattice_contract);

    // Deploy contracts
    let enter_casino_contract = EnterCasinoContract {
        address: String::from("0x03"),
        test: 14,
    };

    let mut roulette_contract = RouletteContract {
        address: String::from("0x04"),
        bids: HashMap::new(),
    };

    let blackjack_contract = BlackjackContract {
        address: String::from("0x05"),
    };

    // Raise contracts to contract level - only owner (0x02) of the lattice can do this
    map.get_mut(&String::from("0x01")).unwrap().set_level(
        &String::from("0x02"),
        &String::from("0x03"),
        &String::from("contract"),
    );
    map.get_mut(&String::from("0x01")).unwrap().set_level(
        &String::from("0x02"),
        &String::from("0x04"),
        &String::from("contract"),
    );
    map.get_mut(&String::from("0x01")).unwrap().set_level(
        &String::from("0x02"),
        &String::from("0x05"),
        &String::from("contract"),
    );

    // Owner 0x02 hires 0x06 to be dealer
    enter_casino_contract.hire_person(&mut map, &String::from("0x02"), &String::from("0x07"));
    // User 0x05 enters the casino with 1234 tokens, and is raised from "bot" to "player"
    enter_casino_contract.enter_casino(&mut map, &String::from("0x06"), 1234);

    // 0x05 now plays roulette/blackjack, which is allowed since he has payed the entry fee and is assigned "player"
    roulette_contract.bid_on_number(&mut map, &String::from("0x06"), 42, 4321);
    blackjack_contract.play_blackjack(&mut map, &String::from("0x06"));

    // 0x06 spins the wheel
    roulette_contract.spin_wheel(&mut map, &String::from("0x07"));

    // The owner closes the contract
    roulette_contract.close_roulette(&mut map, &String::from("0x02"));
}

#[test]
#[should_panic]
fn player_cannot_spin_wheel() {
    let mut map: HashMap<String, LatticeContract> = HashMap::new();

    // Deploy lattice contract. It is put in a map to simulate blockchain addresses
    let lattice_contract = LatticeContract::new(String::from("0x02"));
    map.insert(String::from("0x01"), lattice_contract);

    // Deploy contracts
    let enter_casino_contract = EnterCasinoContract {
        address: String::from("0x03"),
        test: 14,
    };

    let mut roulette_contract = RouletteContract {
        address: String::from("0x04"),
        bids: HashMap::new(),
    };

    map.get_mut(&String::from("0x01")).unwrap().set_level(
        &String::from("0x02"),
        &String::from("0x03"),
        &String::from("contract"),
    );
    map.get_mut(&String::from("0x01")).unwrap().set_level(
        &String::from("0x02"),
        &String::from("0x04"),
        &String::from("contract"),
    );

    enter_casino_contract.hire_person(&mut map, &String::from("0x02"), &String::from("0x07"));
    // User 0x05 enters the casino with 1234 tokens, and is raised from "bot" to "player"
    enter_casino_contract.enter_casino(&mut map, &String::from("0x06"), 1234);

    // 0x05 is "player" and cannot spin the wheel since "dealer" does not flow to "player"
    roulette_contract.spin_wheel(&mut map, &String::from("0x06"));
}

#[test]
#[should_panic]
fn can_decrease_level() {
    let mut map: HashMap<String, LatticeContract> = HashMap::new();

    // Deploy lattice contract. It is put in a map to simulate blockchain addresses
    let lattice_contract = LatticeContract::new(String::from("0x02"));
    map.insert(String::from("0x01"), lattice_contract);

    // Deploy contracts
    let enter_casino_contract = EnterCasinoContract {
        address: String::from("0x03"),
        test: 14,
    };

    let mut roulette_contract = RouletteContract {
        address: String::from("0x04"),
        bids: HashMap::new(),
    };

    map.get_mut(&String::from("0x01")).unwrap().set_level(
        &String::from("0x02"),
        &String::from("0x03"),
        &String::from("contract"),
    );
    map.get_mut(&String::from("0x01")).unwrap().set_level(
        &String::from("0x02"),
        &String::from("0x04"),
        &String::from("contract"),
    );

    enter_casino_contract.enter_casino(&mut map, &String::from("0x06"), 1234);

    roulette_contract.bid_on_number(&mut map, &String::from("0x06"), 42, 4321);

    // 0x06 does something suspicious and we dont trust him anymore.
    map.get_mut(&String::from("0x01")).unwrap().set_level(
        &String::from("0x02"),
        &String::from("0x06"),
        &String::from("bot"),
    );
    // The untrusted party can no longer play.
    roulette_contract.bid_on_number(&mut map, &String::from("0x06"), 2, 12);
}

#[test]
fn it_works() {
    let mut map: HashMap<String, LatticeContract> = HashMap::new();

    let lattice_contract = LatticeContract::new(String::from("0x02"));
    map.insert(String::from("0x01"), lattice_contract);
    let contract = EnterCasinoContract {
        address: String::from("0x03"),
        test: 14,
    };

    // simulating the blockchain getting the correct lattice contract
    map.get_mut(&String::from("0x01")).unwrap().set_level(
        &String::from("0x02"), // This is inferred in the context on the blockchain, and not given as an argument
        &String::from("0x03"),
        &String::from("contract"),
    );
    map.get_mut(&String::from("0x01")).unwrap().set_level(
        &String::from("0x02"), // This is inferred in the context on the blockchain, and not given as an argument
        &String::from("0x06"),
        &String::from("player"),
    );
    contract.enter_casino(&mut map, &String::from("0x04"), 1500);
}

#[test]
#[should_panic]
fn should_fail() {
    let mut map: HashMap<String, LatticeContract> = HashMap::new();

    let lattice_contract = LatticeContract::new(String::from("0x02"));
    map.insert(String::from("0x01"), lattice_contract);
    let contract = EnterCasinoContract {
        address: String::from("0x03"),
        test: 14,
    };

    map.get_mut(&String::from("0x01")).unwrap().set_level(
        &String::from("0x02"),
        &String::from("0x03"),
        &String::from("contract"),
    );
    contract.enter_casino(&mut map, &String::from("0x06"), 1500);
    contract.hire_person(&mut map, &String::from("0x06"), &String::from("0x06"));
}
