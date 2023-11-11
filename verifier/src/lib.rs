use risc0_zkvm::Receipt;
use methods::WORMHOLE_ID;

fn verify(receipt: Receipt) -> bool {
    match receipt.verify(WORMHOLE_ID) {
        Ok(_) => return true,
        Err(_) => return false,
    }
}