use blake3::Hasher;

pub struct MemoryArena;

impl MemoryArena {
    pub fn new() -> Self {
        Self
    }

    pub fn process(state: &[u8], memory_mb: usize) -> Vec<u8> {
        let arena_size = memory_mb * 1024 * 1024;

        let mut arena = vec![0u8; arena_size];

        for i in 0..arena.len() {
            arena[i] = state[i % state.len()];
        }

        let mut cursor = 0usize;

        for round in 0..1024 {
            let mut hasher = Hasher::new();

            let chunk_end = (cursor + 64).min(arena.len());

            hasher.update(&arena[cursor..chunk_end]);
            hasher.update(&(round as u64).to_le_bytes());

            let digest = hasher.finalize();
            let digest_bytes = digest.as_bytes();

            for i in 0..digest_bytes.len() {
                let pos = (cursor + i * 7919) % arena.len();
                arena[pos] ^= digest_bytes[i];
            }

            cursor = (cursor + 104729) % arena.len();
        }

        let mut final_hasher = Hasher::new();
        final_hasher.update(&arena);

        final_hasher.finalize().as_bytes().to_vec()
    }
}