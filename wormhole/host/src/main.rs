use host::*;
use methods::{
    WORMHOLE_ID,
    WORMHOLE_ELF,
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use risc0_zkvm::{Receipt, InnerReceipt, SegmentReceipts, Journal, SegmentReceipt};

fn wormhole() -> Receipt {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    env_logger::init();

    // An executor environment describes the configurations for the zkVM
    // including program inputs.
    // An default ExecutorEnv can be created like so:
    // `let env = ExecutorEnv::builder().build().unwrap();`
    // However, this `env` does not have any inputs.
    //
    // To add add guest input to the executor environment, use
    // ExecutorEnvBuilder::write().
    // To access this method, you'll need to use ExecutorEnv::builder(), which
    // creates an ExecutorEnvBuilder. When you're done adding input, call
    // ExecutorEnvBuilder::build().

    let nullifier = [0_u8; 32];
    let secret = [1_u8; 32];
    let amount = 100_u64;
    let balance = 1000_u64;
    let state_root = [2_u8; 32];
    let sender_addr = [3_u8; 20];
    let env = ExecutorEnv::builder()
    .write(&nullifier)
    .unwrap()
    .write(&secret)
    .unwrap()
    .write(&amount)
    .unwrap()
    .write(&balance)
    .unwrap()
    .write(&state_root)
    .unwrap()
    .write(&sender_addr)
    .unwrap()
    .build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, WORMHOLE_ELF).unwrap();
    receipt
}

fn encode_seal(seal: &Vec<u32>) -> Vec<u8> {
    let mut res = Vec::<u8>::new();
    for i in seal {
        res.extend_from_slice(&i.to_be_bytes());
    }
    res
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

fn main() {

    // TODO: Implement code for retrieving receipt journal here.

    let receipt = wormhole();
    expose_pi(&receipt);
    let seal = receipt.inner.flat().unwrap()[0].seal.clone();
    let journal = receipt.journal.bytes;
    
    let seal_encoded = encode_seal(&seal);
    let seal_decoded = decode_seal(&seal_encoded);
    
    println!("{:?}", journal);
    
    let receipt2 = Receipt::new(
        InnerReceipt::Flat(SegmentReceipts(vec![SegmentReceipt {
            seal: seal_decoded,
            index: 0,
            hashfn: "poseidon".to_string(),
        }])),
        journal,
    );

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt2.verify(WORMHOLE_ID).unwrap();
}

#[derive(Debug)]
pub struct PubInputs {
    nullifier: [u8; 32],
    amount: u64,
    balance: u64,
    state_root: [u8; 32],
    sender_addr: [u8; 20],
}
impl PubInputs {
    pub fn new(input_array: &[u8]) -> PubInputs {
        let nullifier = vec_to_array32(extract_bytes(&input_array[0..128], 32)).unwrap();
        let amount = u32::from_le_bytes(vec_to_array4(input_array[128..132].to_vec()).unwrap()) as u64;
        let balance = u32::from_le_bytes(vec_to_array4(input_array[132..136].to_vec()).unwrap()) as u64;
        let state_root = vec_to_array32(extract_bytes(&input_array[136..264], 32)).unwrap();
        let sender_addr = vec_to_array20(extract_bytes(&input_array[264..],20)).unwrap();

        PubInputs {
            nullifier,
            amount,
            balance,
            state_root,
            sender_addr,
        }
    }
}
fn vec_to_array4(vec: Vec<u8>) -> Result<[u8; 4], String> {
    if vec.len() == 4 {
        let mut arr = [0u8; 4];
        arr.copy_from_slice(&vec);
        Ok(arr)
    } else {
        Err(format!("expected a vec<u8> of length 4, got {}", vec.len()))
    }
}
fn vec_to_array8(vec: Vec<u8>) -> Result<[u8; 1], String> {
    if vec.len() == 1 {
        let mut arr = [0u8; 1];
        arr.copy_from_slice(&vec);
        Ok(arr)
    } else {
        Err(format!("expected a vec<u8> of length 1, got {}", vec.len()))
    }
}
fn vec_to_array20(vec: Vec<u8>) -> Result<[u8; 20], String> {
    if vec.len() == 20 {
        let mut arr = [0u8; 20];
        arr.copy_from_slice(&vec);
        Ok(arr)
    } else {
        Err(format!("Expected a Vec<u8> of length 20, got {}", vec.len()))
    }
}
fn vec_to_array32(vec: Vec<u8>) -> Result<[u8; 32], String> {
    if vec.len() == 32 {
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&vec);
        Ok(arr)
    } else {
        Err(format!("Expected a Vec<u8> of length 32, got {}", vec.len()))
    }
}
fn extract_bytes(encoded_slice: &[u8], num_bytes: usize) -> Vec<u8> {
    encoded_slice.chunks(4).map(|chunk| chunk[0]).take(num_bytes).collect()
}