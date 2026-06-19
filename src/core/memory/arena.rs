use blake3::Hasher;

pub struct MemoryArena;

impl MemoryArena {
    pub fn new() -> Self {
        Self
    }

    /// Process the state through a CPU and RAM memory-hard stage.
    ///
    /// Implements data-dependent random addressing (preventing TMTO attacks)
    /// and floating-point chaotic map mixing to maximize CPU execution pipeline cost.
    pub fn process(state: &[u8], memory_mb: usize) -> Vec<u8> {
        let arena_size = memory_mb * 1024 * 1024;

        if arena_size == 0 || state.is_empty() {
            return state.to_vec();
        }

        let mut arena = vec![0u8; arena_size];

        // 1. Initialize memory arena using BLAKE3 XOF keystream
        let block_count = arena_size / 1024;
        let mut counter = 0u64;

        for i in 0..block_count {
            let offset = i * 1024;
            let keystream = derive_keystream(state, counter);
            arena[offset..offset + 1024].copy_from_slice(&keystream);
            counter += 1;
        }

        // Handle remainder if arena size is not a multiple of 1024
        let remainder = arena_size % 1024;
        if remainder > 0 {
            let offset = block_count * 1024;
            let keystream = derive_keystream(state, counter);
            arena[offset..arena_size].copy_from_slice(&keystream[..remainder]);
        }

        // 2. Data-Dependent Random Memory-Hard mixing loop
        let mut current_hash = blake3::hash(state);
        let mut seed_val = 0.5f64; // Logistic map variable

        // Double pass over the memory arena
        for _pass in 0..2 {
            for i in 0..block_count {
                let digest_bytes = current_hash.as_bytes();
                let rand_val = u64::from_le_bytes([
                    digest_bytes[0], digest_bytes[1], digest_bytes[2], digest_bytes[3],
                    digest_bytes[4], digest_bytes[5], digest_bytes[6], digest_bytes[7]
                ]);

                // Read from a data-dependent random block index
                let ref_block_idx = (rand_val as usize) % block_count;

                let dest_offset = i * 1024;
                let src_offset = ref_block_idx * 1024;

                // CPU-intensive mixing loop operating on the 1024-byte block
                for j in (0..1024).step_by(8) {
                    let d_idx = dest_offset + j;
                    let s_idx = src_offset + j;

                    let mut a_bytes = [0u8; 8];
                    let mut b_bytes = [0u8; 8];
                    a_bytes.copy_from_slice(&arena[d_idx..d_idx + 8]);
                    b_bytes.copy_from_slice(&arena[s_idx..s_idx + 8]);

                    let a = u64::from_le_bytes(a_bytes);
                    let b = u64::from_le_bytes(b_bytes);

                    // Chaotic logistic map iteration to consume CPU ALU / FPU cycles
                    seed_val = 3.99 * seed_val * (1.0 - seed_val);
                    let chaos_bits = (seed_val * 1e15) as u64;

                    // Multiplications, additions, and rotations to stress CPU pipeline
                    let mixed = a.wrapping_add(b)
                        .rotate_left(13)
                        .wrapping_mul(0x517cc1b727220a95)
                        ^ chaos_bits;

                    arena[d_idx..d_idx + 8].copy_from_slice(&mixed.to_le_bytes());
                }

                // Update running hash state
                current_hash = blake3::hash(&arena[dest_offset..dest_offset + 1024]);
            }
        }

        // 3. Finalize
        let mut final_hasher = Hasher::new();
        final_hasher.update(current_hash.as_bytes());
        // Verify final block state integrity
        final_hasher.update(&arena[arena.len().saturating_sub(1024)..]);

        final_hasher.finalize().as_bytes().to_vec()
    }
}

fn derive_keystream(state: &[u8], counter: u64) -> [u8; 1024] {
    let mut output = [0u8; 1024];
    let mut hasher = Hasher::new();
    hasher.update(b"xaor.memory.init.v1");
    hasher.update(state);
    hasher.update(&counter.to_le_bytes());

    let mut reader = hasher.finalize_xof();
    reader.fill(&mut output);
    output
}
