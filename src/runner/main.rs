// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
fn main() -> Result<asimov_module::SysexitsError, Box<dyn std::error::Error>> {
    use asimov_module::SysexitsError::*;
    use jq::JsonFilter;
    use std::io::{Read, stdin, stdout};

    // Load environment variables from `.env`:
    asimov_module::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = asimov_module::args_os()?;
    if args.len() < 2 {
        return Ok(EX_USAGE);
    }

    // Configure logging & tracing:
    #[cfg(feature = "tracing")]
    asimov_module::tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(asimov_module::tracing_subscriber::filter::LevelFilter::WARN)
        .init();

    // Load and compile the filter:
    let filter_path = args.iter().skip(1).next().expect("filter path");
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

#[cfg(not(feature = "std"))]
fn main() {
    unimplemented!("asimov-jq-runner requires the 'std' feature")
}
