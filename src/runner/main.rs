// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "std"))]
compile_error!("asimov-jq-runner requires the 'std' feature");

use asimov_module::SysexitsError::{self, *};
use clap::Parser;
use clientele::StandardOptions;
use jq::JsonFilter;
use std::{
    error::Error,
    io::{Read, stdin, stdout},
    path::PathBuf,
};

/// asimov-jq-runner
#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// The `.jq` filter file to execute.
    filter: PathBuf,
}

pub fn main() -> Result<SysexitsError, Box<dyn Error>> {
    // Load environment variables from `.env`:
    asimov_module::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = asimov_module::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    // Handle the `--version` flag:
    if options.flags.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(EX_OK);
    }

    // Handle the `--license` flag:
    if options.flags.license {
        print!("{}", include_str!("../../UNLICENSE"));
        return Ok(EX_OK);
    }

    // Configure logging & tracing:
    #[cfg(feature = "tracing")]
    asimov_module::init_tracing_subscriber(&options.flags).expect("failed to initialize logging");

    // Load and compile the filter:
    let filter_path = options.filter;
    let filter_json = std::fs::read_to_string(filter_path)?;
    let filter: JsonFilter = filter_json.parse()?;

    // Read the input JSON from stdin:
    let mut input_json = String::new();
    stdin().read_to_string(&mut input_json)?;

    // Write the output JSON to stdout:
    let output_json = filter.filter_json_str(input_json)?;
    if cfg!(feature = "pretty") {
        colored_json::write_colored_json(&output_json, &mut stdout())?;
        println!();
    } else {
        println!("{}", serde_json::to_string(&output_json).unwrap());
    }

    Ok(EX_OK)
}
