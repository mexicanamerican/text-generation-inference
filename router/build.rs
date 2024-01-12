use std::error::Error;
use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    // Try to get the git sha from the local git repository
    if let Err(e) = EmitBuilder::builder()
        .fail_on_error()
        .git_sha(false)
        .emit() {
        Err(format!("Failed to emit git info: {}", e))?;
    }
    {
        // Unable to get the git sha
        if let Ok(sha) = std::env::var("GIT_SHA").unwrap_or_else(|_| panic!("GIT_SHA environment variable not set")) {
            // Set it from an env var
            println!("cargo:rustc-env=VERGEN_GIT_SHA={sha}");
        }
    }

    // Set docker label if present
    if let Ok(label) = std::env::var("DOCKER_LABEL").unwrap_or_else(|_| panic!("DOCKER_LABEL environment variable not set")) {
        // Set it from an env var
        println!("cargo:rustc-env=DOCKER_LABEL={label}");
    }

    Ok(())
}
