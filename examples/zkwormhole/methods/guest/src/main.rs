#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


use risc0_zkvm::guest::env;
use tiny_keccak::{Keccak, Hasher};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // TODO: Implement your guest code here

    // read the input
    let nullifier: [u8; 32] = env::read();
    env::commit(&nullifier);
    let secret: [u8; 32] = env::read();
    let mut hasher = Keccak::v256();
    let mut addr = [0_u8;32];
    hasher.update(&nullifier);
    hasher.update(&secret);
    hasher.finalize(&mut addr);

    let value: u64 = env::read();
    let balance: u64 = env::read();
    env::commit(&value);
    assert!(balance > value);
    let state_root: [u8; 32] = env::read();
    env::commit(&state_root);
    let sender_addr: [u8; 20] = env::read();
    env::commit(&sender_addr);
}
