extern crate paste;

type LatticeElement = String;

#[macro_export]
macro_rules! gen_lattice_contract {
    // Check at ting givet med er rigtige typer gennem match
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

// Lattice Contract er et struct med disse methods implemented på det.
// Struct indeholder mapping fra address til lattice level
// Der skal være en init function for at "finde ud af" hvem owner af lattice contract er. Eller måske give owner med i macro. Addressen skal sættes til top i mappet - owner.
// Lattice er en struct med list af elements (strings). Impl method på dette struct, som er vores partial order.