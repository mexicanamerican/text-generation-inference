use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    // Try to get the git sha from the local git repository
    if EmitBuilder::builder()
        .fail_on_error()
        .git_sha(false)
        .emit()
        .unwrap_or_else(|_| {std::env::set_var("VERGEN_GIT_SHA", "")})
    {
        // Unable to get the git sha
        if let Ok(sha) = std::env::var("GIT_SHA") {
            // Set it from an env var
            std::env::set_var("VERGEN_GIT_SHA", sha)
        }
    }

    // Set docker label if present
    if let Ok(label) = std::env::var("DOCKER_LABEL") {
        // Set it from an env var
        std::env::set_var("DOCKER_LABEL", label)
    }

    Ok(())
}
