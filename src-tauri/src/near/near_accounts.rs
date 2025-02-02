use near_api::Signer;
use std::sync::Arc;

pub fn create_singer(seed: String)  -> Arc<Signer>{
    // Create a signer from a seed phrase
    let seed_phrase = Signer::from_seed_phrase(&seed, None).unwrap(); // "royal success river ..."
    let signer = Signer::new(seed_phrase).unwrap(); // Create the signer
    signer
}

