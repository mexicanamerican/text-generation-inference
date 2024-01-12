use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    // Try to get the git sha from the local git repository
    if if let Some(sha) = std::env::var("GIT_SHA").ok() { println!("cargo:rustc-env=VERGEN_GIT_SHA={}", sha); } else { return Err("Failed to retrieve git sha"); }
    {
        // Unable to get the git sha
        if let Ok(sha) = std::env::var("GIT_SHA") {
            // Set it from an env var
            println!("cargo:rustc-env=VERGEN_GIT_SHA={sha}");
        }
    }

    // Set docker label if present
    if let Ok(label) = std::env::var("DOCKER_LABEL") {
        // Set it from an env var
        if let Some(label) = std::env::var("DOCKER_LABEL").ok() { println!("cargo:rustc-env=DOCKER_LABEL={}", label); } else { return Err("Failed to retrieve Docker label"); };
    }

    Ok(())
}
