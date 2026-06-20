# Hash Anatomy

---

## The PHC String Format

Xaor uses the standard **PHC (Password Hashing Competition) String Format** to serialize hash outputs. This format is fully self-describing, meaning all memory capacities, round counts, parallel lane configurations, and salt details are embedded directly inside the string.

### Anatomy Diagram

```
$xaor$v=3$m=256$r=16$n=32$l=1$o=std$fnBPCGtZq...$DIdrV5qNZ...
 ──┬── ─┬─ ──┬── ─┬─ ─┬─ ─┬─ ─┬── ──┬── ───┬───
   │    │    │    │   │   │   │     │      └─ Base64 BLAKE3-XOF Finalized Hash
   │    │    │    │   │   │   │     └─────── Base64 Random Salt (32 bytes)
   │    │    │    │   │   │   └───────────── Output Mode (std / qnt)
   │    │    │    │   │   └───────────────── Parallel lanes (Rayon threads)
   │    │    │    │   └───────────────────── Graph Nodes count
   │    │    │    └───────────────────────── Chaotic rounds count
   │    │    └────────────────────────────── Memory capacity parameter (MB)
   │    └─────────────────────────────────── Xaor protocol version
   └──────────────────────────────────────── Algorithm identifier prefix (xaor)
```

---

## Detailed Parameter Breakdown

* **Algorithm Identifier (`$xaor`)**: Identifies that the hash was generated using the Xaor Cryptographic Engine. Used by parsers and database middleware to route verification queries.
* **Protocol Version (`v=3`)**: The core version of the Xaor state layout. As improvements are rolled out, this signals the verifier which computation graph layout to construct.
* **Memory Capacity (`m=256`)**: The size of the memory arena in megabytes (MB) that is allocated for the dual-chain memory passes.
* **Chaotic Rounds (`r=16`)**: The number of chaotic mixing rounds applied per node in the Compound and Chaos stages.
* **Graph Nodes (`n=32`)**: The size of the directed topology graph. Higher node counts mean a more complex dependency path for each hash sequence.
* **Parallel Lanes (`l=1`)**: Rayon lane branching. If higher than 1, indicates that the graph passes are evaluated simultaneously across independent parallel thread contexts.
* **Output Mode (`o=std` or `o=qnt`)**: 
  * `std`: Standard mode returning a 64-byte (512-bit) BLAKE3 hash.
  * `qnt`: Post-quantum resistant mode returning a 128-byte (1024-bit) dual-chain BLAKE3-XOF output.
* **Random Salt**: A cryptographically secure 32-byte salt, encoded as a URL-safe Base64 string.
* **Final Hash**: The computed final digest output, encoded as a URL-safe Base64 string.
