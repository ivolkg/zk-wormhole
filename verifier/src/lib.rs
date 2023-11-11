extern crate libc;

use libc::size_t;
use std::slice;
use risc0_zkvm::Receipt;
use methods::WORMHOLE_ID;


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
    true
}

fn verify2(receipt: Receipt) -> bool {
    match receipt.verify(WORMHOLE_ID) {
        Ok(_) => return true,
        Err(_) => return false,
    }
}
