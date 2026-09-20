use std::{env, fs, path::PathBuf};
use sol_external_runtime_v02_consumer::{adapter_from_env, fixture_from_env, run_consumer_v02, run_external_v02_flow};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let evidence = match args.as_slice() {
        [command, adapter, request] if command == "run" => {
            let input = fs::read_to_string(request).map_err(|e| e.to_string())?;
            let request = serde_json::from_str(&input).map_err(|e| e.to_string())?;
            run_consumer_v02(&PathBuf::from(adapter), &request)?
        }
        [] => {
            // Backward-compatible M0.9 evidence mode. New consumers should use `run`.
            let adapter = adapter_from_env("SOL_RUNTIME_V02_ADAPTER")?;
            let fixture = fixture_from_env()?;
            run_external_v02_flow(&adapter, &fixture)?
        }
        _ => return Err("usage: sol-external-runtime-v02-consumer run <adapter-command> <consumer-request.json>".to_owned()),
    };
    println!("{}", serde_json::to_string(&evidence).map_err(|e| e.to_string())?);
    Ok(())
}
