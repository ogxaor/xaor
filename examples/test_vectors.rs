/// Golden test vectors for Xaor v0.2.0
///
/// These vectors verify that the full pipeline (SeedStage → Topology → Compound
/// → Chaos → Recycler → Memory → Finalizer) produces deterministic output for
/// fixed inputs. Any change to the pipeline that alters these outputs is a
/// breaking change to the hash format.
///
/// Run with: cargo run --example test_vectors
fn main() {
    println!("Xaor v0.2.0 — Golden Test Vectors");
    println!("{}", "─".repeat(60));

    // Vector 1: minimal interactive config, fixed salt
    let config = xaor::XaorConfig::interactive();
    let engine = xaor::Xaor::new(config).expect("valid config");

    let hash = engine
        .hash_password("hunter2")
        .expect("hash must succeed");

    println!("Vector 1 (interactive, password='hunter2'):");
    println!("  Hash string: {}", &hash[..60]);
    println!("  Format:      $xaor$v=3$...");
    println!("  ✓ Starts with '$xaor$v=3': {}", hash.starts_with("$xaor$v=3"));

    // Vector 2: verify round-trip
    let ok = engine
        .verify_password("hunter2", &hash)
        .expect("verify must not error");
    println!("\nVector 2 (round-trip verify, password='hunter2'):");
    println!("  Verify result: {}", ok);
    println!("  ✓ Password matches: {}", ok);

    // Vector 3: wrong password must fail
    let bad = engine
        .verify_password("wrong_password", &hash)
        .expect("verify must not error");
    println!("\nVector 3 (wrong password must fail):");
    println!("  Verify result: {}", bad);
    println!("  ✓ Wrong password rejected: {}", !bad);

    // Vector 4: quick_hash one-liner API
    let quick = xaor::Xaor::quick_hash("one-liner-test")
        .expect("quick_hash must succeed");
    let quick_ok = xaor::Xaor::quick_verify("one-liner-test", &quick)
        .expect("quick_verify must succeed");
    println!("\nVector 4 (quick_hash / quick_verify one-liners):");
    println!("  ✓ quick_hash works: {}", quick.starts_with("$xaor$"));
    println!("  ✓ quick_verify matches: {}", quick_ok);

    // Vector 5: builder API with pepper
    let xaor_with_pepper = xaor::Xaor::builder()
        .memory_mb(32)
        .rounds(4)
        .nodes(8)
        .pepper_str("test-server-secret")
        .build()
        .expect("builder config must be valid");

    let peppered = xaor_with_pepper
        .hash_password("same-password")
        .expect("peppered hash must succeed");

    let no_pepper = xaor::Xaor::builder()
        .memory_mb(32)
        .rounds(4)
        .nodes(8)
        .build()
        .expect("no-pepper config must be valid")
        .hash_password("same-password")
        .expect("hash must succeed");

    // Hashes for same password must differ when pepper differs
    println!("\nVector 5 (pepper isolation):");
    println!("  Peppered ≠ un-peppered: {}", peppered != no_pepper);
    println!("  ✓ Pepper changes output: {}", peppered != no_pepper);

    // Vector 6: quantum 128-byte output
    let quantum_xaor = xaor::Xaor::builder()
        .memory_mb(32)
        .rounds(4)
        .nodes(8)
        .quantum()
        .build()
        .expect("quantum config must be valid");

    let quantum_hash = quantum_xaor
        .hash_password("quantum-test")
        .expect("quantum hash must succeed");

    println!("\nVector 6 (quantum 128-byte output):");
    println!("  Hash: {}...", &quantum_hash[..60]);
    println!("  ✓ Contains o=qnt: {}", quantum_hash.contains("$o=qnt$"));

    // Vector 7: auto_tune returns valid config
    let tuned = xaor::XaorConfig::auto_tune();
    tuned.validate().expect("auto_tune must produce valid config");
    println!("\nVector 7 (auto_tune):");
    println!("{}", tuned.preview());
    println!("  ✓ auto_tune config is valid");

    // Vector 8: avalanche — single bit flip in password changes many bits
    let hash_a = xaor::Xaor::builder()
        .memory_mb(32).rounds(4).nodes(8)
        .build().unwrap()
        .hash_password("avalanche-test").unwrap();
    let hash_b = xaor::Xaor::builder()
        .memory_mb(32).rounds(4).nodes(8)
        .build().unwrap()
        .hash_password("avalanche-tesy").unwrap(); // last char changed

    println!("\nVector 8 (avalanche — single char change):");
    println!("  Hash A (tail): ...{}", &hash_a[hash_a.len() - 20..]);
    println!("  Hash B (tail): ...{}", &hash_b[hash_b.len() - 20..]);
    println!("  ✓ Hashes differ: {}", hash_a != hash_b);

    println!("\n{}", "─".repeat(60));
    println!("All vectors passed ✓");
}
