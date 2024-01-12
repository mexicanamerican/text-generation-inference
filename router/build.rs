use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    // Try to get the git sha from the local git repository
if let Some(sha) = vergen::current_branch_commit_hash() { // Get the git sha from the local git repository
    std::env::set_var("VERGEN_GIT_SHA", sha); // Set `VERGEN_GIT_SHA` environment variable
} else if let Ok(sha) = std::env::var("GIT_SHA") { // Check if `GIT_SHA` environment variable is set
    std::env::set_var("VERGEN_GIT_SHA", sha); // Set `VERGEN_GIT_SHA` environment variable from `GIT_SHA`
}
if let Some(sha) = vergen::current_branch_commit_hash() { // Get the git sha from the local git repository
    std::env::set_var("VERGEN_GIT_SHA", sha); // Set `VERGEN_GIT_SHA` environment variable
} else if let Ok(sha) = std::env::var("GIT_SHA") { // Check if `GIT_SHA` environment variable is set
    std::env::set_var("VERGEN_GIT_SHA", sha); // Set `VERGEN_GIT_SHA` environment variable from `GIT_SHA`
}
    if EmitBuilder::builder()
        .fail_on_error()
        .git_sha(false)
        .emit()
        .is_err()
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
        println!("cargo:rustc-env=DOCKER_LABEL={label}");
    }

    Ok(())
}
