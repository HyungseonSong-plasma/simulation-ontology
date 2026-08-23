use sol_external_runtime_v02_consumer::{adapter_from_env, fixture_from_env, run_external_v02_flow};

fn main() {
    if let Err(error) = run() {
        eprintln!("external runtime v0.2 evidence failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let adapter = adapter_from_env("SOL_RUNTIME_V02_ADAPTER")?;
    let fixture = fixture_from_env()?;
    let evidence = run_external_v02_flow(&adapter, &fixture)?;
    println!(
        "{}",
        serde_json::to_string(&evidence).map_err(|error| error.to_string())?
    );
    Ok(())
}
