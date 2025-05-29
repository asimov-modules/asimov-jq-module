// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure tracing & logging:
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing_subscriber::filter::LevelFilter::WARN)
        .init();

    println!("asimov-jq-runner"); // TODO

    Ok(())
}

#[cfg(not(feature = "std"))]
fn main() {
    unimplemented!("asimov-jq-runner requires the 'std' feature")
}
