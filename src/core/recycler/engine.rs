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
            let mut hasher = Hasher::new();

            hasher.update(&current);
            hasher.update(&(cycle as u64).to_le_bytes());

            let hash = hasher.finalize();
            let hash_bytes = hash.as_bytes();

            for i in 0..current.len() {
                current[i] ^= hash_bytes[i % hash_bytes.len()];
                current[i] =
                    current[i].rotate_left(((i + cycle) % 8) as u32);

                current[i] = current[i]
                    .wrapping_add((cycle as u8).wrapping_mul(13));
            }

            let len = current.len();
            let shift = (cycle + 1) % len;
            current.rotate_left(shift);
        }

        current
    }
}
