# LBS-Project-2022
This repository contains the codebase for the Language Based Security 2022 Project. 

The project itself consists of extending Rust using macros, to allow explicit control over method invocations in a blockchain environment. Authenticiy levels are specified by a lattice contract, where methods can be restricted, requiring the caller to have a specific level. The syntax is:
```
#[level(Player)]
pub fn only_player_level_can_call_this() {
    // Specific code here
}
```
The extension is built, such that support for authenticity in a inter-contract manner, meaning that several contracts all can subscribe to the same lattice and share the mappings from a blockchain address to a lattice level, using the following syntax:
```
#[lattice_address(0x01)]
struct MyContractStateStruct {
    // state variables
}
```
Note that in the tests, some additional code is required to simulate the blockchain environment - such as satisfying the type checker on invocations (rather than rpc calls on the chain) - this is simply here to exemplify the extensions, and would not be required in a real blockchain environment.

For further explanation, see the project report.

### File Structure
- The *"lattice_contract_macro"* folder contains the declarative macro for generating a lattice contract.  

- The *"lattice_macro"* folder contains the procedual macro used to indicate required levels and subscribing to lattice contracts  

- The *"src/lib.rs"* file contains the working example of the project, the CasinoContract implementation.  

- The *"tests* folder contains several tests for the working CasinoContract example implementation.