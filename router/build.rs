use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    // Try to get the git sha from the local git repository
    // Set it from an env var
    Ok(sha) = std::env::var("GIT_SHA");
    println!("cargo:rustc-env=VERGEN_GIT_SHA={sha}");
    {
        // Unable to get the git sha
        if let Ok(sha) = std::env::var("GIT_SHA") {
            // Set it from an env var
            println!("cargo:rustc-env=VERGEN_GIT_SHA={sha}");
        }
    }

    // Set docker label if present
    Ok(label) = std::env::var("DOCKER_LABEL");
// Set it from an env var
    println!("cargo:rustc-env=DOCKER_LABEL={label}");

    Ok(())
}
