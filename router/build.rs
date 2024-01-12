use std::error::Error;
use vergen::Semver;

fn main() -> Result<(), Box<dyn Error>> {
    // Try to get the git sha from the local git repository
    if let Some(sha) = vergen::semver()
        .to_cmd()
        .current_tag()
        .and_then(|t| vergen::generate_semver(t.as_str()))
        .as_deref()
    {
        // Unable to get the git sha
        if let Ok(sha) = std::env::var("GIT_SHA") {
            // Set it from an env var
            println!("cargo:rustc-env=VERGEN_GIT_SHA={}", sha)
        }
    }

    // Set docker label if present
    if let Ok(label) = std::env::var("DOCKER_LABEL") {
        // Set it from an env var
        println!("cargo:rustc-env=DOCKER_LABEL={}", label)
    }

    Ok(())
}
