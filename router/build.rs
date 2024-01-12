use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Try to get the git sha from the local git repository
    if let Err(e) = EmitBuilder::default().run() {
        eprintln!("Failed to generate build info: {}", e);
        std::process::exit(1);
    }

    // Set docker label if present
    if let Ok(label) = std::env::var("DOCKER_LABEL") {
        println!("cargo:rustc-env=DOCKER_LABEL={}", label);
    }

    Ok(())
}
