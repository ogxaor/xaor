use blake3::Hasher;

pub struct RecyclerEngine;

impl RecyclerEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn recycle(state: &[u8], cycles: usize) -> Vec<u8> {
        let mut current = state.to_vec();

        if current.is_empty() {
            return current;
        }

        for cycle in 0..cycles {
            // Step 1: derive a per-cycle keystream using BLAKE3-XOF.
            // Domain separator isolates Recycler output from other stages.
            let mut hasher = Hasher::new();
            hasher.update(b"xaor.recycler.v2");
            hasher.update(&current);
            hasher.update(&(cycle as u64).to_le_bytes());

            let mut keystream = vec![0u8; current.len()];
            hasher.finalize_xof().fill(&mut keystream);

            // Step 2: XOR state with the keystream.
            for i in 0..current.len() {
                current[i] ^= keystream[i];
            }

            // Step 3: 64-bit word-level mixing for inter-byte diffusion.
            // Bug fix: the old code did byte-level rotate_left((i + cycle) % 8)
            // which is a no-op when the rotation amount is 0, providing no
            // mixing on those iterations.
            let word_count = current.len() / 8;
            for w in 0..word_count {
                let base = w * 8;
                let mut word = u64::from_le_bytes(
                    current[base..base + 8].try_into().unwrap()
                );
                // Mix with the next word (wrapping) and a cycle-derived constant.
                let next_base = ((w + 1) % word_count) * 8;
                let next_word = u64::from_le_bytes(
                    current[next_base..next_base + 8].try_into().unwrap()
                );
                word = word.wrapping_add(next_word)
                    .rotate_left(((cycle * 7 + w * 3 + 1) % 63 + 1) as u32)
                    .wrapping_mul(0x6c62272e07bb0142);
                current[base..base + 8].copy_from_slice(&word.to_le_bytes());
            }

            // Step 4: Rotate the full byte array for positional mixing.
            let len = current.len();
            let shift = (cycle + 1) % len;
            current.rotate_left(shift);
        }

        current
    }
}

impl Default for RecyclerEngine {
    fn default() -> Self {
        Self::new()
    }
}
