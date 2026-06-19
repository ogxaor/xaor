use crate::chaos::ChaosConfig;

pub fn mix_seed(seed: &[u8], scale: u64) -> u64 {
    if seed.is_empty() {
        return 1;
    }

    let mut value = 0u64;

    for (i, byte) in seed.iter().enumerate() {
        let shift = ((i % 8) * 8) as u32;
        value ^= (*byte as u64) << shift;
        value = value.rotate_left(5) ^ ((*byte as u64).wrapping_mul(0x9E37));
    }

    (value % (scale.saturating_sub(1)).max(1)) + 1
}

pub fn next_state(x: u64, config: ChaosConfig) -> u64 {
    let scale = config.scale as u128;
    let r = config.r as u128;
    let x = x as u128;
    let remaining = scale.saturating_sub(x);
    let denominator = scale.saturating_mul(scale).max(1);

    let next = (r
        .saturating_mul(x)
        .saturating_mul(remaining))
        / denominator;

    let next = next % scale.max(1);
    next as u64
}

pub fn chaos_stream(seed: &[u8], len: usize, config: ChaosConfig) -> Vec<u8> {
    if len == 0 {
        return Vec::new();
    }

    let mut x = mix_seed(seed, config.scale);
    let mut output = Vec::with_capacity(len);

    for _ in 0..len {
        x = next_state(x, config);
        if x == 0 {
            x = 1;
        }

        output.push((x % 256) as u8);
    }

    output
}
