use sol_core::LoadedFixture;
use sol_mock_adapter::echo;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let Some(path) = env::args().nth(1) else {
        eprintln!("usage: sol-cli <fixture.json>");
        return ExitCode::from(2);
    };

    let fixture = match LoadedFixture::load(&path) {
        Ok(fixture) => fixture,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(1);
        }
    };

    let response = echo(fixture.document);
    match serde_json::to_string_pretty(&response) {
        Ok(json) => {
            println!("{json}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("failed to serialize mock adapter response: {error}");
            ExitCode::from(1)
        }
    }
}
