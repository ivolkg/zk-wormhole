#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


use risc0_zkvm::guest::env;
use tiny_keccak::{Keccak, Hasher};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // TODO: Implement your guest code here

    // read the input
    let nullifier: [u8; 32] =env::read();
    env::commit(&nullifier);
    let secret: [u8; 32] = env::read();
    let mut hasher = Keccak::v256();
    hasher.update(&nullifier);
    hasher.update(&secret);
    let mut unspendable_addr = [0_u8; 32];
    hasher.finalize(&mut unspendable_addr);
    let mut hasher = Keccak::v256();
    hasher.update(&unspendable_addr);
    let mut unspendable_addr = [0_u8; 32];
    hasher.finalize(&mut unspendable_addr);
    let amount: u64 = env::read();
    env::commit(&amount);
    let balance: u64 = env::read();
    assert!(balance > amount);
    let state_root: [u8; 32] = env::read();
    env::commit(&state_root);
    let sender_addr: [u8; 20] = env::read();
    env::commit(&sender_addr);
}
