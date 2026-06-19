use std::env;
use std::process::ExitCode;
use std::time::Instant;

use xaor::{error_catalog, Xaor, XaorConfig, XaorMode};

fn print_usage() {
    eprintln!(
        "Usage:\n  xaor [--profile <interactive|standard|high-security|server>] <command>\n\nCommands:\n  hash <password>\n  verify <password> <stored-hash>\n  config\n  errors\n  bench [iterations]\n  mode <hash|encrypt>\n\nEnvironment:\n  XAOR_PROFILE=interactive|standard|high-security|server\n  XAOR_ROUNDS=<usize>\n  XAOR_MEMORY=<usize>\n  XAOR_NODES=<usize>\n  XAOR_OUTPUT_SIZE=<usize>\n  XAOR_MODE=hash|encrypt"
    );
}

fn parse_args() -> (Option<String>, String, Vec<String>) {
    let mut args = env::args().skip(1);
    let mut profile = None;

    while let Some(arg) = args.next() {
        if arg == "--profile" {
            profile = args.next();
            continue;
        }

        let command = arg;
        let rest = args.collect::<Vec<_>>();
        return (profile, command, rest);
    }

    (profile, String::new(), Vec::new())
}

fn load_xaor(profile_override: Option<&str>) -> Result<Xaor, String> {
    let config = XaorConfig::from_env_with_profile(profile_override).map_err(|err| err.to_string())?;
    Xaor::new(config).map_err(|err| format!("{} ({})", err, err.code()))
}

fn main() -> ExitCode {
    let (profile, command, args) = parse_args();

    if command.is_empty() {
        print_usage();
        return ExitCode::from(2);
    }

    if profile.is_some() && command == "--profile" {
        eprintln!("--profile must be followed by a command");
        return ExitCode::from(2);
    }

    let mut args = args.into_iter();

    match command.as_str() {
        "hash" => {
            let Some(password) = args.next() else {
                print_usage();
                return ExitCode::from(2);
            };

            match load_xaor(profile.as_deref())
                .and_then(|x| x.hash_password(&password).map_err(|e| format!("{} ({})", e, e.code())))
            {
                Ok(hash) => {
                    println!("{hash}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("{err}");
                    ExitCode::from(1)
                }
            }
        }
        "verify" => {
            let Some(password) = args.next() else {
                print_usage();
                return ExitCode::from(2);
            };
            let Some(stored) = args.next() else {
                print_usage();
                return ExitCode::from(2);
            };

            match load_xaor(profile.as_deref())
                .and_then(|x| x.verify_password(&password, &stored).map_err(|e| format!("{} ({})", e, e.code())))
            {
                Ok(true) => {
                    println!("VALID");
                    ExitCode::SUCCESS
                }
                Ok(false) => {
                    println!("INVALID");
                    ExitCode::from(3)
                }
                Err(err) => {
                    eprintln!("{err}");
                    ExitCode::from(1)
                }
            }
        }
        "config" => match XaorConfig::from_env_with_profile(profile.as_deref()) {
            Ok(config) => {
                println!("profile config:");
                println!("  mode: {:?}", config.mode);
                println!("  rounds: {}", config.rounds);
                println!("  memory_size_mb: {}", config.memory_size_mb);
                println!("  node_count: {}", config.node_count);
                println!("  output_size: {}", config.output_size);
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("{err}");
                ExitCode::from(1)
            }
        },
        "errors" => {
            for (code, description) in error_catalog() {
                println!("{code}: {description}");
            }
            ExitCode::SUCCESS
        }
        "bench" => {
            let iterations = match args.next() {
                Some(value) => match value.parse::<usize>() {
                    Ok(value) if value > 0 => value,
                    _ => {
                        eprintln!("iterations must be a positive integer");
                        return ExitCode::from(2);
                    }
                },
                None => 10,
            };

            let engine = match load_xaor(profile.as_deref()) {
                Ok(engine) => engine,
                Err(err) => {
                    eprintln!("{err}");
                    return ExitCode::from(1);
                }
            };

            let sample = "benchmark-sample-password";
            let mut total_hash_nanos = 0u128;

            for _ in 0..iterations {
                let start = Instant::now();
                match engine.hash_password(sample) {
                    Ok(_) => {
                        total_hash_nanos += start.elapsed().as_nanos();
                    }
                    Err(err) => {
                        eprintln!("{err} ({})", err.code());
                        return ExitCode::from(1);
                    }
                }
            }

            let average_ms = (total_hash_nanos as f64 / iterations as f64) / 1_000_000.0;

            println!("benchmark profile:");
            println!("  iterations: {}", iterations);
            println!("  avg_hash_time_ms: {:.2}", average_ms);
            println!("  sample_password_len: {}", sample.len());
            ExitCode::SUCCESS
        }
        "mode" => {
            let Some(mode) = args.next() else {
                print_usage();
                return ExitCode::from(2);
            };

            match XaorMode::parse(&mode) {
                Some(parsed) => {
                    println!("{parsed:?}");
                    ExitCode::SUCCESS
                }
                None => {
                    eprintln!("unknown mode: {mode}");
                    ExitCode::from(2)
                }
            }
        }
        "--profile" => {
            eprintln!("--profile must appear before the command");
            ExitCode::from(2)
        }
        _ => {
            print_usage();
            ExitCode::from(2)
        }
    }
}
