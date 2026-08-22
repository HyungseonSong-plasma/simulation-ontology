use sol_external_conformance_consumer::run_external_project_observation;
use std::process;

fn main() {
    match run_external_project_observation() {
        Ok(observation) => {
            let json = serde_json::to_string_pretty(&observation)
                .expect("conformance observation is composed of JSON values");
            println!("{json}");
        }
        Err(error) => {
            eprintln!("external conformance example failed: {error}");
            process::exit(1);
        }
    }
}
