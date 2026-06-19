use xcrypt::{ProofConfig, ProofEngine};

fn main() {
    let engine = ProofEngine::new(ProofConfig::low()).unwrap();
    let challenge = engine.challenge("signup").unwrap();
    let solution = engine.solve(&challenge).unwrap();

    println!("Challenge: {}", challenge.serialize());
    println!("Solution nonce: {}", solution.nonce);
    println!("Leading zero bits: {}", solution.leading_zero_bits);
    println!("Verified: {}", engine.verify(&challenge, &solution).unwrap());
}
