extern crate paste;

// Macro for generating the lattice contract and needed methods for it.
#[macro_export]
macro_rules! gen_lattice_contract {
    // TODO: Match types of params
    ($lattice_elements:expr, $lattice_order:expr, $set_level:expr) => {
        use std::collections::HashMap;
        type LatticeElement = String;
        type Address = String;

        #[derive(Clone)]
        pub struct LatticeContract {
            map: HashMap<Address, LatticeElement>,
        }

        impl LatticeContract {
            pub fn new(caller_address: Address) -> Self {
                let mut map = HashMap::new();
                map.insert(caller_address, String::from("top"));
                Self { map }
            }

            // Raise the level of an address. Use this to deploy contracts, by raising contract level to desired.
            pub fn set_level(
                &mut self,
                caller_address: &Address,
                address: &Address,
                target_level: &LatticeElement,
            ) {
                let elements: Vec<String> = $lattice_elements;
                let caller_level = &self.get_level(caller_address);

                // Only correct lattice elements can be used in map
                assert!(elements.contains(target_level));

                // Ensure that caller has appropiate level to perform set_level
                assert!(self.flows_to($set_level, caller_level));
                // Ensure that you can only change person with level under yours
                let prev_level = &self.get_level(address);
                assert!(self.flows_to(prev_level, caller_level) && prev_level.ne(caller_level));
                // Ensure that you can only raise up to a level under your own level
                assert!(self.flows_to(target_level, caller_level) && target_level.ne(caller_level));

                self.map
                    .insert(address.to_string(), target_level.to_string());
            }

            pub fn flows_to(&self, level1: &LatticeElement, level2: &LatticeElement) -> bool {
                $lattice_order(level1, level2)
            }

            pub fn get_level(&self, address: &Address) -> LatticeElement {
                self.map
                    .get(address)
                    .unwrap_or(&String::from("bot"))
                    .to_string()
            }
        }
    };
}
