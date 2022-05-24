extern crate paste;

#[macro_export]
macro_rules! gen_lattice_contract {
    ($lattice:expr, $raise_level:expr, $deploy_level:expr) => {
            pub fn deploy_contract() {
                println!("Deploy if level  > {}", $deploy_level);
            }

            pub fn raise_level() {
                // Arguments for target level
                println!("Raise level of user if caller level > {}", $raise_level);
            }

    };
}