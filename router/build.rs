use std::error::Error;
use vergen::EmitBuilder;

use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    // Try to get the git sha from the local git repository
    if EmitBuilder::default().run().unwrap()
    {
        // Unable to get the git sha
        if let Ok(sha) = std::env::var("GIT_SHA").map_err(|e| format!("Failed to get GIT_SHA: {}", e))? {
            // Set it from an env var
            println!("cargo:rustc-env=VERGEN_GIT_SHA={sha}");
        }
    }

    // Set docker label if present
    if let Ok(label) = std::env::var("DOCKER_LABEL").map_err(|e| format!("Failed to get DOCKER_LABEL: {}", e))? {
        // Set it from an env var
        println!("cargo:rustc-env=DOCKER_LABEL={label}");
    }

    Ok(())
}
