/// 64-bit integer chaotic map using xorshift + Knuth multiplicative hashing.
///
/// This replaces the old f64 logistic map which had data-dependent timing on
/// CPU denormal values — a potential side-channel timing oracle.
///
/// This version is:
/// - Purely integer — no floating-point, no denormal risk
/// - A bijection (invertible) — every 64-bit input maps to a unique output
/// - Chaotic — small seed differences cause full avalanche within ~4 steps
/// - Fast — three XOR-shifts + one multiply, fits in a single CPU pipeline stage
pub fn xorshift_mix(x: u64) -> u64 {
    // Xorshift64 — full-period 64-bit chaotic scramble
    let x = x ^ (x << 13);
    let x = x ^ (x >> 7);
    let x = x ^ (x << 17);
    // Knuth multiplicative hash — bijective, maximises bit diffusion
    x.wrapping_mul(0x2545F4914F6CDD1D)
}

/// Seed a 64-bit chaotic state from a byte slice.
///
/// Uses the FNV-like structure seeded from all input bytes to produce
/// a non-trivially-predictable initial state.
pub fn mix_seed(seed: &[u8]) -> u64 {
    let mut state = 0x6E37_1B5A_C3F2_9D81u64; // arbitrary non-zero start
    for (i, byte) in seed.iter().enumerate() {
        state ^= (*byte as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        state = state.rotate_left(((i % 47) + 1) as u32);
        state = xorshift_mix(state);
    }
    if state == 0 {
        state = 0xDEAD_BEEF_CAFE_BABE;
    }
    state
}

/// Generate a chaotic byte stream of `len` bytes seeded from `seed`.
///
/// Each byte is derived from a successive application of `xorshift_mix`,
/// ensuring every output byte depends on all previous chaotic state.
pub fn chaos_stream(seed: &[u8], len: usize) -> Vec<u8> {
    if len == 0 {
        return Vec::new();
    }

    let mut state = mix_seed(seed);
    let mut output = Vec::with_capacity(len);

    for _ in 0..len {
        state = xorshift_mix(state);
        output.push((state >> 56) as u8); // top byte — best bit distribution
    }

    output
}
