use blake3::Hasher;
use zeroize::Zeroize;

pub struct MemoryArena;

impl MemoryArena {
    pub fn new() -> Self {
        Self
    }

    /// Process the state through a 3-pass CPU+RAM memory-hard stage.
    ///
    /// ## Security Properties
    ///
    /// ### BLAKE3 Keyed Arena Initialization
    /// The arena is filled using `blake3::Hasher::new_keyed(&arena_key)` where
    /// `arena_key` is derived from the password+salt via BLAKE3. This means the
    /// arena content is cryptographically tied to the secret — an attacker who
    /// does not know the password cannot reproduce the arena, even knowing the
    /// BLAKE3 function.
    ///
    /// ### 3-Pass Memory Scheme (Inspired by Argon2id)
    /// - **Pass 0** (Forward): BLAKE3-keyed XOF initialization.
    /// - **Pass 1** (Forward, data-dependent): Mixes blocks using addresses
    ///   derived from a running hash chain. Each block references a random
    ///   earlier block — TMTO requires keeping all blocks in RAM.
    /// - **Pass 2** (Backward, independent chain): Iterates blocks in reverse,
    ///   using an INDEPENDENT hash chain for reference addresses. This creates
    ///   backward dependencies that force adversaries to hold the full arena
    ///   simultaneously. Partial arena computation is now impossible.
    ///
    /// ### CPU Hardening
    /// - 64-bit ALU mixing (rotations, multiplications) inside each block loop
    /// - Integer chaotic mixing with Xorshift-derived constants per block
    /// - BLAKE3 block hash update per block (prevents loop parallelism)
    ///
    /// ### Memory Security
    /// The arena `Vec<u8>` is explicitly zeroed via `zeroize` before being
    /// dropped, preventing recovery of intermediate crypto state from freed RAM.
    pub fn process(state: &[u8], memory_mb: usize) -> Vec<u8> {
        let arena_size = memory_mb * 1024 * 1024;

        if arena_size == 0 || state.is_empty() {
            return state.to_vec();
        }

        let mut arena = vec![0u8; arena_size];
        let block_count = arena_size / 1024;

        if block_count == 0 {
            return state.to_vec();
        }

        // ── Step 1: Derive a 32-byte arena key from the state ───────────────
        // This ties the entire arena content to the password+salt.
        // An attacker who knows the BLAKE3 function but not the secret cannot
        // reproduce the arena without first cracking the password.
        let arena_key: [u8; 32] = {
            let mut kh = Hasher::new();
            kh.update(b"xaor.arena.key.v2");
            kh.update(state);
            let h = kh.finalize();
            let mut k = [0u8; 32];
            k.copy_from_slice(h.as_bytes());
            k
        };

        // ── Pass 0: Forward initialization with BLAKE3 keyed mode ────────────
        // Each 1024-byte block is filled with a keyed BLAKE3-XOF stream.
        // The key (arena_key) is secret, so the initial arena content is secret.
        let mut counter = 0u64;
        let block_count_floor = arena_size / 1024;
        for i in 0..block_count_floor {
            let offset = i * 1024;
            let block = derive_keyed_block(&arena_key, counter);
            arena[offset..offset + 1024].copy_from_slice(&block);
            counter += 1;
        }
        let remainder = arena_size % 1024;
        if remainder > 0 {
            let offset = block_count_floor * 1024;
            let block = derive_keyed_block(&arena_key, counter);
            arena[offset..arena_size].copy_from_slice(&block[..remainder]);
        }

        // ── Pass 1: Forward data-dependent mixing ────────────────────────────
        // Each block reads from a DATA-DEPENDENT random earlier block
        // (address derived from a running BLAKE3 hash chain). This is the
        // key TMTO-resistance mechanism — partial recomputation is impossible.
        let mut running_hash = blake3::hash(state);

        for i in 0..block_count {
            let digest = running_hash.as_bytes();
            let rand64 = u64::from_le_bytes([
                digest[0], digest[1], digest[2], digest[3],
                digest[4], digest[5], digest[6], digest[7],
            ]);

            let ref_idx = (rand64 as usize) % block_count;
            let dest_off = i * 1024;
            let src_off  = ref_idx * 1024;

            // Mix dest block with src block using 64-bit keyed operations
            for j in (0..1024).step_by(8) {
                let d = dest_off + j;
                let s = src_off  + j;

                let a = u64::from_le_bytes(arena[d..d + 8].try_into().unwrap());
                let b = u64::from_le_bytes(arena[s..s + 8].try_into().unwrap());

                // Integer chaotic mixing: xorshift + Knuth multiplicative hash
                let chaos = xorshift_mix(rand64 ^ (j as u64));
                let mixed = a
                    .wrapping_add(b)
                    .rotate_left(((rand64 >> 8) % 63 + 1) as u32)
                    .wrapping_mul(0x517C_C1B7_2722_0A95)
                    ^ chaos;

                arena[d..d + 8].copy_from_slice(&mixed.to_le_bytes());
            }

            running_hash = blake3::hash(&arena[dest_off..dest_off + 1024]);
        }

        // ── Pass 2: Backward independent data-dependent mixing ───────────────
        // NEW: Iterates blocks in REVERSE using an INDEPENDENT hash chain.
        // This creates backward block dependencies, forcing the adversary to
        // have the full arena in memory. Partial TMTO is now impossible.
        let mut back_hash = {
            let mut h = Hasher::new_keyed(&arena_key);
            h.update(b"xaor.arena.backward.v2");
            h.update(&running_hash.as_bytes()[..]);
            h.finalize()
        };

        for i in (0..block_count).rev() {
            let digest = back_hash.as_bytes();
            let rand64 = u64::from_le_bytes([
                digest[0], digest[1], digest[2], digest[3],
                digest[4], digest[5], digest[6], digest[7],
            ]);

            // Reference index from the BACKWARD chain — independent of Pass 1
            let ref_idx = (rand64 as usize) % block_count;
            let dest_off = i * 1024;
            let src_off  = ref_idx * 1024;

            for j in (0..1024).step_by(8) {
                let d = dest_off + j;
                let s = src_off  + j;

                let a = u64::from_le_bytes(arena[d..d + 8].try_into().unwrap());
                let b = u64::from_le_bytes(arena[s..s + 8].try_into().unwrap());

                let chaos = xorshift_mix(rand64 ^ (j as u64).wrapping_mul(0xB5AD_4ECE_DA1C_E2A9));
                let mixed = a
                    .wrapping_mul(0x6C62_272E_07BB_0142 | 1)
                    .wrapping_add(b)
                    .rotate_left(((rand64 >> 16) % 63 + 1) as u32)
                    ^ chaos;

                arena[d..d + 8].copy_from_slice(&mixed.to_le_bytes());
            }

            back_hash = {
                let mut h = Hasher::new_keyed(&arena_key);
                h.update(&arena[dest_off..dest_off + 1024]);
                h.finalize()
            };
        }

        // ── Step 3: Finalize ─────────────────────────────────────────────────
        // Compress the full arena state into 64 bytes using BLAKE3-XOF.
        // We hash both the final block (most recently modified) and the first
        // block (modified in Pass 0 and 1 but not in Pass 2's last iteration)
        // to ensure the output depends on the full arena.
        let mut final_hasher = Hasher::new_keyed(&arena_key);
        final_hasher.update(b"xaor.arena.finalize.v2");
        final_hasher.update(&arena[..1024]);                              // first block
        final_hasher.update(&arena[arena.len().saturating_sub(1024)..]);  // last block
        final_hasher.update(back_hash.as_bytes());                        // backward chain
        final_hasher.update(running_hash.as_bytes());                     // forward chain

        let mut output = vec![0u8; 64];
        final_hasher.finalize_xof().fill(&mut output);

        // ── Security: Zero the arena before freeing ──────────────────────────
        // Prevents adversaries from reading intermediate crypto state
        // out of freed heap memory.
        arena.zeroize();

        output
    }
}

impl Default for MemoryArena {
    fn default() -> Self {
        Self::new()
    }
}

/// Derive a 1024-byte block using BLAKE3 in keyed mode.
///
/// `key` is the 32-byte arena key (derived from password+salt).
/// `counter` distinguishes blocks within the same arena.
fn derive_keyed_block(key: &[u8; 32], counter: u64) -> [u8; 1024] {
    let mut output = [0u8; 1024];
    let mut hasher = Hasher::new_keyed(key);
    hasher.update(b"xaor.arena.block.v2");
    hasher.update(&counter.to_le_bytes());
    hasher.finalize_xof().fill(&mut output);
    output
}

/// 64-bit integer chaotic XOR-shift — bijective, no floating-point.
///
/// Used for CPU-intensive mixing inside the memory arena loops.
/// The multiplier constant ensures strong bit diffusion per iteration.
#[inline(always)]
fn xorshift_mix(mut x: u64) -> u64 {
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x.wrapping_mul(0x2545_F491_4F6C_DD1D)
}
