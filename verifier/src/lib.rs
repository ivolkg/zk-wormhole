extern crate libc;

use libc::size_t;
use std::slice;
use risc0_zkvm::{Receipt, SegmentReceipts, SegmentReceipt, InnerReceipt};
use methods::WORMHOLE_ID;
use host::wormhole;
use host::PubInputs;

#[no_mangle]
pub extern "C" fn prove_wormhole() -> Receipt {
    wormhole([0_u8; 32], [1_u8; 32], 100_u64, 1000_u64, [2_u8; 32], [3_u8; 20])
}

#[no_mangle]
pub extern "C" fn get_public_inputs(receipt: &Receipt) -> PubInputs {
    let journal = &receipt.journal.bytes;
    PubInputs::new(&journal)
}

fn as_u32_be(a: u8, b: u8, c: u8, d: u8) -> u32 {
    ((a as u32) << 24) +
    ((b as u32) << 16) +
    ((c as u32) <<  8) +
    ((d as u32) <<  0)
}

fn decode_seal(encoded: &[u8]) -> Vec<u32> {
     let mut seal = Vec::<u32>::new();
     for i in (0..encoded.len()).step_by(4) {
        seal.push(as_u32_be(encoded[i], encoded[i + 1], encoded[i + 2], encoded[i + 3]));
     }
     seal
}

#[no_mangle]
pub extern "C" fn verify(proof_ptr: *const u8, proof_len: size_t,
                         nullifier_ptr: *const u8, nullifier_len: size_t,
                         value_ptr: *const u8,value_len: size_t,
                         sender_ptr: *const u8, sender_len: size_t,
                         state_root_ptr: *const u8, state_root_len: size_t) -> bool {
    let nullifier = unsafe {
        slice::from_raw_parts(nullifier_ptr, nullifier_len as usize)
    };
    let value = unsafe {
        slice::from_raw_parts(value_ptr, value_len as usize)
    };
    let sender = unsafe {
        slice::from_raw_parts(sender_ptr, sender_len as usize)
    };
    let state_root = unsafe {
        slice::from_raw_parts(state_root_ptr, state_root_len as usize)
    };
    let proof = unsafe {
        slice::from_raw_parts(proof_ptr, proof_len as usize)
    };
    
    let seal = decode_seal(&proof);
   
    let journal = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0];
    
    let receipt = Receipt::new(
        InnerReceipt::Flat(SegmentReceipts(vec![SegmentReceipt {
            seal: seal,
            index: 0,
            hashfn: "poseidon".to_string(),
        }])),
        journal,
    );
    verify2(receipt)
}

fn verify2(receipt: Receipt) -> bool {
    match receipt.verify(WORMHOLE_ID) {
        Ok(_) => return true,
        Err(_) => return false,
    }
}
