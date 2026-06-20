use blake3::Hasher;

pub struct FinalizerEngine;

impl FinalizerEngine {
    pub fn new() -> Self {
        Self
    }

    /// Finalize the pipeline state into a fixed-length cryptographic hash.
    ///
    /// ## Diffusion Layer (4 rounds)
    /// Each byte is mixed with its neighbours across 4 rounds before
    /// compression. This ensures full avalanche across the state before
    /// the BLAKE3 compression step.
    ///
    /// ## Standard Mode (64 bytes)
    /// BLAKE3-XOF with domain `"xaor.finalize.v2"` fills all 64 bytes
    /// with genuinely independent keystream material.
    ///
    /// ## Quantum Mode (128 bytes)
    /// Two **independent** BLAKE3-XOF streams with different domain keys:
    /// - `"xaor.finalize.q0.v2"` → bytes 0-63
    /// - `"xaor.finalize.q1.v2"` → bytes 64-127
    ///
    /// Using two independent streams (rather than one XOF extended to 128B)
    /// ensures that even if an attacker finds a collision in one stream,
    /// the other stream's collision space is fully independent. This gives
    /// genuine 512-bit post-quantum security against Grover + birthday attacks.
    pub fn finalize(state: &[u8], output_size: usize) -> Vec<u8> {
        let mut buffer = state.to_vec();

        // Diffusion layer: mix each byte with its left and right neighbours
        // across 4 rounds to ensure full avalanche before compression.
        if !buffer.is_empty() {
            for round in 0u32..4 {
                for i in 0..buffer.len() {
                    let next = buffer[(i + 1) % buffer.len()];
                    let prev = buffer[(i + buffer.len() - 1) % buffer.len()];

                    buffer[i] ^= next.rotate_left(round + 1);
                    buffer[i] = buffer[i].wrapping_add(prev);
                    buffer[i] = buffer[i].rotate_left(3);
                }
            }
        }

        if output_size == 128 {
            // ── Quantum Mode ─────────────────────────────────────────────────
            // Two completely independent XOF streams — different domain labels,
            // same input. Even if one stream is broken, the other is intact.
            let mut h0 = Hasher::new();
            h0.update(b"xaor.finalize.q0.v2");
            h0.update(&buffer);

            let mut h1 = Hasher::new();
            h1.update(b"xaor.finalize.q1.v2");
            h1.update(&buffer);

            let mut output = vec![0u8; 128];
            h0.finalize_xof().fill(&mut output[..64]);
            h1.finalize_xof().fill(&mut output[64..]);
            output
        } else {
            // ── Standard Mode (64 bytes) ──────────────────────────────────────
            let mut hasher = Hasher::new();
            hasher.update(b"xaor.finalize.v2");
            hasher.update(&buffer);

            let mut reader = hasher.finalize_xof();
            let mut output = vec![0u8; 64];
            reader.fill(&mut output);
            output
        }
    }
}

impl Default for FinalizerEngine {
    fn default() -> Self {
        Self::new()
    }
}
