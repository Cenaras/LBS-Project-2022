extern crate paste;

#[macro_export]
macro_rules! gen_lattice_contract {
    // Check at ting givet med er rigtige typer gennem match
    ($lattice_name:ident, $lattice_elements:expr, $lattice_order:expr, $raise_level:expr) => {
        use std::collections::HashMap;
        type LatticeElement = String;
        type Address = String;

        #[derive(Clone)]
        struct $lattice_name {
            map: HashMap<Address, LatticeElement>,
        }

        impl $lattice_name {
            pub fn new(caller_address: Address) -> Self {
                let mut map = HashMap::new();
                map.insert(caller_address, String::from("top"));
                Self { map }
            }

            // Raise the level of an address. Use this to deploy contracts, by raising contract level to desired.
            pub fn raise_level(
                &mut self,
                caller_address: &Address,
                address: &Address,
                target_level: &LatticeElement,
            ) {
                let elements = $lattice_elements;
                let bot = String::from("bot");
                let caller_level = self.map.get(caller_address).unwrap_or(&bot);

                // Ensure that caller has appropiate level to perform raise_level
                assert!(self.le($raise_level, caller_level));
                // Ensure that you cannot raise up to your own level
                assert!(self.le(target_level, caller_level) && target_level.ne(caller_level));

                self.map
                    .insert(address.to_string(), target_level.to_string());

                // Arguments for target level
                println!("Raise level of user if caller level > {}", $raise_level);
            }

            pub fn le(&self, address1: &Address, address2: &Address) -> bool {
                $lattice_order(address1, address2)
            }
        }
    };
}

// Lattice Contract er et struct med disse methods implemented på det.
// Struct indeholder mapping fra address til lattice level
// Der skal være en init function for at "finde ud af" hvem owner af lattice contract er. Eller måske give owner med i macro. Addressen skal sættes til top i mappet - owner.
// Lattice er en struct med list af elements (strings). Impl method på dette struct, som er vores partial order.
