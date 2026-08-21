use std::{env, fs, process};

use sol_cli::{
    plan_document, plan_public_contract_document, validate_document,
    validate_public_contract_document,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("ERROR: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.as_slice() {
        [command, path] if command == "validate" => {
            let input = fs::read_to_string(path).map_err(|error| error.to_string())?;
            println!("{}", validate_document(&input)?);
            Ok(())
        }
        [command, path, format] if command == "validate" && format == "--json" => {
            let input = fs::read_to_string(path).map_err(|error| error.to_string())?;
            println!("{}", validate_public_contract_document(&input)?);
            Ok(())
        }
        [command, path, flag, target] if command == "plan" && flag == "--target" => {
            let input = fs::read_to_string(path).map_err(|error| error.to_string())?;
            println!("{}", plan_document(&input, target)?);
            Ok(())
        }
        [command, path, flag, target, format]
            if command == "plan" && flag == "--target" && format == "--json" =>
        {
            let input = fs::read_to_string(path).map_err(|error| error.to_string())?;
            println!("{}", plan_public_contract_document(&input, target)?);
            Ok(())
        }
        _ => Err(
            "usage: sol-cli validate <simulation.json> [--json] | sol-cli plan <simulation.json> --target mock [--json]"
                .to_owned(),
        ),
    }
}
