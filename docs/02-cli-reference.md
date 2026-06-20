# CLI Reference

---

## Installation & Environment Setup

You can install the Xaor CLI either by downloading precompiled binaries directly or building it from source.

### 1. Direct Binary Installation (No Rust Needed)
Use the following terminal commands to download the precompiled binary:

#### PowerShell (Windows)
```powershell
# Create installation directory and download the binary
New-Item -ItemType Directory -Force -Path "$Home\.xaor\bin"
Invoke-WebRequest -Uri "https://github.com/ogxaor/xaor/releases/latest/download/xaor.exe" -OutFile "$Home\.xaor\bin\xaor.exe"
```

#### Command Prompt (cmd.exe)
```cmd
:: Create installation directory and download using curl
mkdir "%USERPROFILE%\.xaor\bin"
curl -L "https://github.com/ogxaor/xaor/releases/latest/download/xaor.exe" -o "%USERPROFILE%\.xaor\bin\xaor.exe"
```

### 2. Alternative: Build from Source via Cargo
If you have the Rust toolchain installed, run the compile script to install globally:
```bash
cargo install --path . --bin xaor
```

### 3. Configure System PATH Environment Variables
Ensure the binary path is in your system's `PATH` variable so you can run the `xaor` command globally:

#### Windows (PowerShell)
To permanently add the binary path to your User environment variable block:
```powershell
# For Direct installs (Recommended):
[System.Environment]::SetEnvironmentVariable("Path", [System.Environment]::GetEnvironmentVariable("Path", "User") + ";$Home\.xaor\bin", "User")

# For Cargo source builds:
[System.Environment]::SetEnvironmentVariable("Path", [System.Environment]::GetEnvironmentVariable("Path", "User") + ";$Home\.cargo\bin", "User")
```
*Note: Restart your terminal window for the changes to apply.*

#### Linux & macOS (Bash/Zsh)
Add the path to your `~/.bashrc`, `~/.zshrc`, or `~/.profile`:
```bash
# For Direct installs:
export PATH="$HOME/.xaor/bin:$PATH"

# For Cargo source builds:
export PATH="$HOME/.cargo/bin:$PATH"
```

### 4. Verify Installation
Verify that the `xaor` binary has been successfully added to your environment path:
```bash
xaor --version
```

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
