# CLI Reference

---

## Commands Catalog

The Xaor command line interface (CLI) is a developer utility for hashing, checking config values, and testing system bottlenecks:

| Command | Arguments | Description | Example |
| :--- | :--- | :--- | :--- |
| **`hash`** | `<password>` | Hashes a plaintext password and outputs the PHC string | `xaor hash "pass"` |
| **`verify`** | `<password> <stored-hash>` | Verifies a password against a stored PHC hash string | `xaor verify "pass" '$xaor$...'` |
| **`bench`** | `[iterations]` | Benchmarks hash execution speeds (default: 10 runs) | `xaor bench 5` |
| **`config`** | *None* | Prints details of the active profile and hardware configuration | `xaor config` |
| **`errors`** | *None* | Lists all engine error codes and diagnostic descriptions | `xaor errors` |

---

## Dynamic Profiles (`--profile`)

You can specify the execution profile via the `--profile` flag to balance speed vs memory cost:

```bash
xaor --profile interactive hash "my-password"
```

| Profile | CPU Rounds | Memory Cost | Target Latency | Use Case |
| :--- | :---: | :---: | :---: | :--- |
| **`interactive`** | `12` | `64 MB` | ~100-200 ms | Fast login forms, client-side verifiers |
| **`standard`** | `16` | `256 MB` | ~300-500 ms | Standard user logins (Default) |
| **`server`** | `16` | `256 MB` | ~300-500 ms | Multi-threaded server logs (4 parallel lanes) |
| **`high-security`** | `24` | `512 MB` | ~1-2 sec | Administrative authentication, offline archives |

---

## Environment Overrides

The engine dynamically reads configuration settings from environment variables. If set, they override profile defaults:

* **`XAOR_PROFILE`**: Active profile preset (`interactive`, `standard`, `high-security`, `server`).
* **`XAOR_ROUNDS`**: Number of chaotic rounds.
* **`XAOR_MEMORY`**: Memory arena capacity in megabytes (MB).
* **`XAOR_NODES`**: Computational graph node count.
* **`XAOR_LANES`**: Parallel lanes for rayon execution.
* **`XAOR_PEPPER`**: Server-side secret pepper string (never stored inside the hash output).

---

## Shell Escaping Guides

Because Xaor hash strings contain `$` delimiters, command-line interpreters can mistake them for variables. Ensure you escape them correctly:

### PowerShell (Windows)
Always enclose the hash in **single quotes** (`'`) to disable variable expansion:
```powershell
xaor verify "my-password" '$xaor$v=3$m=256$r=16$n=32$l=1$o=std$fnBPCGtZqTL...$DIdrV...'
```

### Bash & Zsh (Linux / macOS)
Use **single quotes** (`'`) or backslash escaping (`\$`):
```bash
xaor verify "my-password" '$xaor$v=3$m=256$r=16$n=32$l=1$o=std$fnBPCGtZqTL...$DIdrV...'
```

### Command Prompt (Windows cmd.exe)
Cmd does not evaluate `$` as variables. Standard **double quotes** (`"`) work perfectly:
```cmd
xaor verify "my-password" "$xaor$v=3$m=256$r=16$n=32$l=1$o=std$fnBPCGtZqTL...$DIdrV..."
```
