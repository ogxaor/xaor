use std::env;
use std::process::ExitCode;
use std::time::Instant;

use xaor::{error_catalog, Xaor, XaorConfig, XaorMode};

// ── Version from Cargo.toml ────────────────────────────────────────────────────
const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

// ── ANSI colour helpers ────────────────────────────────────────────────────────
const RESET:  &str = "\x1b[0m";
const BOLD:   &str = "\x1b[1m";
const DIM:    &str = "\x1b[2m";
const CYAN:   &str = "\x1b[36m";
const GREEN:  &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED:    &str = "\x1b[31m";
const BLUE:   &str = "\x1b[34m";
const MAGENTA:&str = "\x1b[35m";

fn c(code: &str, text: &str) -> String {
    format!("{code}{text}{RESET}")
}

// ── Main help screen ───────────────────────────────────────────────────────────

fn print_help() {
    println!(
        "\n{logo}

{bold}{name}{reset} {dim}v{version}{reset}   {dim}{description}{reset}

{section}USAGE{reset}
  {cmd}xaor{reset} {arg}[OPTIONS]{reset} {arg}<command>{reset} {arg}[args]{reset}

{section}COMMANDS{reset}
  {cmd}hash{reset}     {arg}<password>{reset}                    Hash a password
  {cmd}verify{reset}   {arg}<password> <stored-hash>{reset}      Verify a password against its hash
  {cmd}bench{reset}    {arg}[iterations]{reset}                  Benchmark hash speed (default 10 runs)
  {cmd}config{reset}                                  Show resolved configuration
  {cmd}errors{reset}                                  List all error codes and descriptions
  {cmd}mode{reset}     {arg}<hash|encrypt>{reset}                Parse and confirm a mode name

{section}OPTIONS{reset}
  {flag}--profile{reset}  {arg}<interactive|standard|high-security|server>{reset}   Preset config profile
  {flag}-v{reset}, {flag}--version{reset}                                           Show version and exit
  {flag}-h{reset}, {flag}--help{reset}                                              Show this help screen

{section}PROFILES{reset}
  {hl}interactive{reset}   ~100-200 ms   Low memory (64 MB)   For login UIs
  {hl}standard{reset}      ~300-500 ms   256 MB memory        General purpose  {dim}(default){reset}
  {hl}high-security{reset} ~1-2 s        512 MB memory        Sensitive data
  {hl}server{reset}        ~300-500 ms   256 MB, 4 lanes      Concurrent servers

{section}ENVIRONMENT VARIABLES{reset}
  {env}XAOR_PROFILE{reset}       = interactive | standard | high-security | server
  {env}XAOR_ROUNDS{reset}        = <usize>   Number of mixing rounds
  {env}XAOR_MEMORY{reset}        = <usize>   Memory arena size in MB
  {env}XAOR_NODES{reset}         = <usize>   Topology node count
  {env}XAOR_LANES{reset}         = <usize>   Parallel execution lanes
  {env}XAOR_MODE{reset}          = hash | encrypt
  {env}XAOR_OUTPUT_MODE{reset}   = standard | quantum
  {env}XAOR_PEPPER{reset}        = <string>  Server-side secret (never stored in hash)

{section}EXAMPLES{reset}
  {dim}# Hash a password with the default profile{reset}
  {cmd}xaor{reset} hash {arg}\"my-password\"{reset}

  {dim}# Verify a stored hash{reset}
  {cmd}xaor{reset} verify {arg}\"my-password\" \"$xaor$...\"{reset}

  {dim}# Hash faster for interactive login{reset}
  {cmd}xaor{reset} {flag}--profile{reset} {arg}interactive{reset} hash {arg}\"password\"{reset}

  {dim}# Benchmark 5 iterations with high-security profile{reset}
  {cmd}xaor{reset} {flag}--profile{reset} {arg}high-security{reset} bench {arg}5{reset}

  {dim}# Show what config will be used{reset}
  {cmd}xaor{reset} config

{dim}Run '{cmd_dim}xaor help <command>{reset_dim}' for detailed help on a specific command.{reset}
",
        logo = format!(
            "{CYAN}{BOLD} ██╗  ██╗  █████╗   ██████╗  ██████╗ {RESET}\n\
             {CYAN}{BOLD} ╚██╗██╔╝ ██╔══██╗ ██╔═══██╗ ██╔══██╗{RESET}\n\
             {CYAN}{BOLD}  ╚███╔╝  ███████║ ██║   ██║ ██████╔╝{RESET}\n\
             {CYAN}{BOLD}  ██╔██╗  ██╔══██║ ██║   ██║ ██╔══██╗{RESET}\n\
             {CYAN}{BOLD} ██╔╝ ██╗ ██║  ██║ ╚██████╔╝ ██║  ██║{RESET}\n\
             {CYAN}{BOLD} ╚═╝  ╚═╝ ╚═╝  ╚═╝  ╚═════╝  ╚═╝  ╚═╝{RESET}"
        ),
        bold      = BOLD,
        reset     = RESET,
        dim       = DIM,
        name      = c(&format!("{BOLD}{CYAN}"), PKG_NAME),
        version   = VERSION,
        description = DESCRIPTION,
        section   = format!("{BOLD}{YELLOW}"),
        cmd       = format!("{BOLD}{GREEN}"),
        cmd_dim   = format!("{GREEN}"),
        arg       = format!("{CYAN}"),
        flag      = format!("{BOLD}{MAGENTA}"),
        env       = format!("{BOLD}{BLUE}"),
        hl        = format!("{BOLD}{GREEN}"),
        reset_dim = format!("{DIM}"),
    );
}

// ── Per-command help ───────────────────────────────────────────────────────────

fn print_command_help(cmd: &str) {
    match cmd {
        "hash" => println!(
            "\n{bold}{cyan}xaor hash{reset} {arg}<password>{reset}\n\n\
             Hash a password using the memory-hard Xaor pipeline.\n\n\
             {section}ARGUMENTS{reset}\n  {arg}<password>{reset}   The password to hash\n\n\
             {section}OUTPUT{reset}\n  A portable {cyan}$xaor$…{reset} stored-hash string written to stdout.\n\n\
             {section}EXAMPLES{reset}\n\
             {dim}  xaor hash \"hunter2\"\n\
               xaor --profile interactive hash \"mypass\"{reset}\n",
            bold = BOLD, cyan = CYAN, reset = RESET,
            arg = CYAN, section = format!("{BOLD}{YELLOW}"), dim = DIM,
        ),
        "verify" => println!(
            "\n{bold}{cyan}xaor verify{reset} {arg}<password> <stored-hash>{reset}\n\n\
             Verify a plaintext password against a stored hash.\n\n\
             {section}ARGUMENTS{reset}\n\
             {dim}  <password>     The plaintext password to check\n\
               <stored-hash>  The hash string produced by `xaor hash`{reset}\n\n\
             {section}EXIT CODES{reset}\n\
             {dim}  0  — VALID (password matches)\n\
               1  — Internal error\n\
               3  — INVALID (password does not match){reset}\n\n\
             {section}EXAMPLES{reset}\n\
             {dim}  xaor verify \"hunter2\" \"$xaor$...\"{reset}\n",
            bold = BOLD, cyan = CYAN, reset = RESET,
            arg = CYAN, section = format!("{BOLD}{YELLOW}"), dim = DIM,
        ),
        "bench" => println!(
            "\n{bold}{cyan}xaor bench{reset} {arg}[iterations]{reset}\n\n\
             Benchmark hash speed on the current machine.\n\
             Prints average ms per hash over N iterations.\n\n\
             {section}ARGUMENTS{reset}\n  {arg}[iterations]{reset}   Number of hashes to run (default: 10)\n\n\
             {section}EXAMPLES{reset}\n{dim}  xaor bench\n\
               xaor bench 5\n\
               xaor --profile interactive bench 20{reset}\n",
            bold = BOLD, cyan = CYAN, reset = RESET,
            arg = CYAN, section = format!("{BOLD}{YELLOW}"), dim = DIM,
        ),
        "config" => println!(
            "\n{bold}{cyan}xaor config{reset}\n\n\
             Print the resolved configuration that will be used.\n\
             Reads environment variables and the selected profile.\n\n\
             {section}EXAMPLES{reset}\n{dim}  xaor config\n\
               XAOR_MEMORY=512 xaor --profile server config{reset}\n",
            bold = BOLD, cyan = CYAN, reset = RESET,
            section = format!("{BOLD}{YELLOW}"), dim = DIM,
        ),
        "errors" => println!(
            "\n{bold}{cyan}xaor errors{reset}\n\n\
             Print all error codes and their descriptions.\n\
             Useful for integrating with external tooling or debugging exit codes.\n",
            bold = BOLD, cyan = CYAN, reset = RESET,
        ),
        _ => {
            eprintln!("{RED}error:{RESET} unknown command '{cmd}'\nRun 'xaor --help' to see available commands.");
        }
    }
}

// ── Argument parser ────────────────────────────────────────────────────────────

enum CliAction {
    Help,
    CommandHelp(String),
    Version,
    Run { profile: Option<String>, command: String, args: Vec<String> },
}

fn parse_args() -> CliAction {
    let argv: Vec<String> = env::args().skip(1).collect();

    if argv.is_empty() {
        return CliAction::Help;
    }

    // Flags that trigger early-exit
    match argv[0].as_str() {
        "-v" | "--version"           => return CliAction::Version,
        "-h" | "--help"              => return CliAction::Help,
        "help" if argv.len() == 1    => return CliAction::Help,
        "help"                       => return CliAction::CommandHelp(argv[1].clone()),
        _ => {}
    }

    // Parse optional --profile flag
    let mut iter = argv.into_iter().peekable();
    let mut profile: Option<String> = None;

    if iter.peek().map(|s| s.as_str()) == Some("--profile") {
        iter.next(); // consume "--profile"
        profile = iter.next();
    }

    let command = match iter.next() {
        Some(c) => c,
        None => return CliAction::Help,
    };

    let args: Vec<String> = iter.collect();
    CliAction::Run { profile, command, args }
}

// ── Config helpers ─────────────────────────────────────────────────────────────

fn load_xaor(profile_override: Option<&str>) -> Result<Xaor, String> {
    let config = XaorConfig::from_env_with_profile(profile_override)
        .map_err(|e| e.to_string())?;
    Xaor::new(config).map_err(|e| format!("{e} ({})", e.code()))
}

// ── main ───────────────────────────────────────────────────────────────────────

fn main() -> ExitCode {
    match parse_args() {

        CliAction::Version => {
            println!("{PKG_NAME} {VERSION}");
            return ExitCode::SUCCESS;
        }

        CliAction::Help => {
            print_help();
            return ExitCode::SUCCESS;
        }

        CliAction::CommandHelp(cmd) => {
            print_command_help(&cmd);
            return ExitCode::SUCCESS;
        }

        CliAction::Run { profile, command, args } => {
            let mut args = args.into_iter();

            match command.as_str() {

                // ── hash ─────────────────────────────────────────────────────────
                "hash" => {
                    let Some(password) = args.next() else {
                        eprintln!("{RED}error:{RESET} missing argument <password>\n\nRun '{BOLD}xaor help hash{RESET}' for usage.");
                        return ExitCode::from(2);
                    };
                    match load_xaor(profile.as_deref())
                        .and_then(|x| x.hash_password(&password).map_err(|e| format!("{e} ({})", e.code())))
                    {
                        Ok(hash) => { println!("{hash}"); ExitCode::SUCCESS }
                        Err(err) => { eprintln!("{RED}error:{RESET} {err}"); ExitCode::from(1) }
                    }
                }

                // ── verify ───────────────────────────────────────────────────────
                "verify" => {
                    let Some(password) = args.next() else {
                        eprintln!("{RED}error:{RESET} missing argument <password>\n\nRun '{BOLD}xaor help verify{RESET}' for usage.");
                        return ExitCode::from(2);
                    };
                    let Some(stored) = args.next() else {
                        eprintln!("{RED}error:{RESET} missing argument <stored-hash>\n\nRun '{BOLD}xaor help verify{RESET}' for usage.");
                        return ExitCode::from(2);
                    };
                    match load_xaor(profile.as_deref())
                        .and_then(|x| x.verify_password(&password, &stored).map_err(|e| format!("{e} ({})", e.code())))
                    {
                        Ok(true)  => { println!("{GREEN}VALID{RESET}");   ExitCode::SUCCESS }
                        Ok(false) => { println!("{RED}INVALID{RESET}");   ExitCode::from(3) }
                        Err(err)  => { eprintln!("{RED}error:{RESET} {err}"); ExitCode::from(1) }
                    }
                }

                // ── config ───────────────────────────────────────────────────────
                "config" => match XaorConfig::from_env_with_profile(profile.as_deref()) {
                    Ok(cfg) => {
                        println!("\n{BOLD}{YELLOW}Xaor Configuration{RESET}");
                        println!("  {BOLD}mode{RESET}          {:?}", cfg.mode);
                        println!("  {BOLD}rounds{RESET}        {}", cfg.rounds);
                        println!("  {BOLD}memory{RESET}        {} MB", cfg.memory_size_mb);
                        println!("  {BOLD}nodes{RESET}         {}", cfg.node_count);
                        println!("  {BOLD}lanes{RESET}         {}", cfg.lanes);
                        println!("  {BOLD}output_mode{RESET}   {:?}", cfg.output_mode);
                        println!("  {BOLD}pepper{RESET}        {}", if cfg.pepper.is_some() { "set" } else { "none" });
                        println!();
                        ExitCode::SUCCESS
                    }
                    Err(err) => { eprintln!("{RED}error:{RESET} {err}"); ExitCode::from(1) }
                },

                // ── errors ───────────────────────────────────────────────────────
                "errors" => {
                    println!("\n{BOLD}{YELLOW}Xaor Error Codes{RESET}\n");
                    for (code, description) in error_catalog() {
                        println!("  {BOLD}{CYAN}{code}{RESET}   {description}");
                    }
                    println!();
                    ExitCode::SUCCESS
                }

                // ── bench ────────────────────────────────────────────────────────
                "bench" => {
                    let iterations = match args.next() {
                        Some(v) => match v.parse::<usize>() {
                            Ok(n) if n > 0 => n,
                            _ => {
                                eprintln!("{RED}error:{RESET} iterations must be a positive integer");
                                return ExitCode::from(2);
                            }
                        },
                        None => 10,
                    };

                    let engine = match load_xaor(profile.as_deref()) {
                        Ok(e) => e,
                        Err(err) => { eprintln!("{RED}error:{RESET} {err}"); return ExitCode::from(1); }
                    };

                    println!("\n{BOLD}{YELLOW}Xaor Benchmark{RESET}");
                    println!("  {DIM}Running {iterations} iterations…{RESET}");

                    let sample = "benchmark-sample-password";
                    let mut total_nanos = 0u128;

                    for i in 0..iterations {
                        let start = Instant::now();
                        match engine.hash_password(sample) {
                            Ok(_) => { total_nanos += start.elapsed().as_nanos(); }
                            Err(err) => {
                                eprintln!("{RED}error:{RESET} {err} ({})", err.code());
                                return ExitCode::from(1);
                            }
                        }
                        // Print simple progress bar
                        let done = (i + 1) * 20 / iterations;
                        let bar: String = "█".repeat(done) + &"░".repeat(20 - done);
                        eprint!("\r  [{bar}] {}/{iterations}", i + 1);
                    }
                    eprintln!(); // newline after progress

                    let avg_ms = (total_nanos as f64 / iterations as f64) / 1_000_000.0;
                    let total_ms = total_nanos as f64 / 1_000_000.0;

                    println!();
                    println!("  {BOLD}iterations{RESET}    {iterations}");
                    println!("  {BOLD}avg / hash{RESET}    {GREEN}{avg_ms:.2} ms{RESET}");
                    println!("  {BOLD}total{RESET}         {total_ms:.0} ms");
                    println!("  {BOLD}input len{RESET}     {} bytes", sample.len());
                    println!();
                    ExitCode::SUCCESS
                }

                // ── mode ─────────────────────────────────────────────────────────
                "mode" => {
                    let Some(mode) = args.next() else {
                        eprintln!("{RED}error:{RESET} missing argument <hash|encrypt>");
                        return ExitCode::from(2);
                    };
                    match XaorMode::parse(&mode) {
                        Some(m) => { println!("{m:?}"); ExitCode::SUCCESS }
                        None    => { eprintln!("{RED}error:{RESET} unknown mode '{mode}' — expected 'hash' or 'encrypt'"); ExitCode::from(2) }
                    }
                }

                // ── unknown ──────────────────────────────────────────────────────
                _ => {
                    eprintln!("{RED}error:{RESET} unknown command '{command}'\n\nRun '{BOLD}xaor --help{RESET}' to see available commands.");
                    ExitCode::from(2)
                }
            }
        }
    }
}
