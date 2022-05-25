extern crate paste;

#[macro_export]
macro_rules! gen_lattice_contract {
    // Check at ting givet med er rigtige typer gennem match
    ($lattice_name:ident, $lattice_elements:expr, $lattice_order:expr, $raise_level:expr, $deploy_level:expr) => {
        use std::collections::HashMap;
        type LatticeElement = String;
        type Address = String;

        struct $lattice_name {
            map: HashMap<Address, LatticeElement>,
        }

        impl $lattice_name {
            pub fn new(owner: Address) -> Self {
                let mut map = HashMap::new();
                map.insert(owner, String::from("top"));
                Self { map }
            }

            pub fn deploy_contract(&mut self) {
                println!("Deploy if level  > {}", $deploy_level);
            }

            pub fn raise_level(&mut self) {
                let elements = $lattice_elements;
                // Arguments for target level
                println!("Raise level of user if caller level > {}", $raise_level);
            }

            pub fn le(&mut self, address1: Address, address2: Address) -> bool {
                $lattice_order(address1, address2)
            }

        }
    };
}

// Lattice Contract er et struct med disse methods implemented på det.
// Struct indeholder mapping fra address til lattice level
// Der skal være en init function for at "finde ud af" hvem owner af lattice contract er. Eller måske give owner med i macro. Addressen skal sættes til top i mappet - owner.
// Lattice er en struct med list af elements (strings). Impl method på dette struct, som er vores partial order.
