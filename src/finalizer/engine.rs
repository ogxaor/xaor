use blake3::Hasher;

pub struct FinalizerEngine;

impl FinalizerEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn finalize(state: &[u8]) -> Vec<u8> {
        let mut buffer = state.to_vec();

        // Diffusion layer
        for round in 0..4 {
            for i in 0..buffer.len() {
                let next = buffer[(i + 1) % buffer.len()];
                let prev = buffer[(i + buffer.len() - 1) % buffer.len()];

                buffer[i] ^= next.rotate_left((round + 1) as u32);
                buffer[i] = buffer[i].wrapping_add(prev);
                buffer[i] = buffer[i].rotate_left(3);
            }
        }

        // Cryptographic compression
        let mut hasher = Hasher::new();
        hasher.update(&buffer);

        let hash = hasher.finalize();

        let mut output = Vec::with_capacity(64);

        output.extend_from_slice(hash.as_bytes());
        output.extend_from_slice(hash.as_bytes());

        output
    }
}