use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    // Emit cargo and rustc compile time values
    EmitBuilder::builder().all_cargo().all_rustc().emit()?;

    // Try to get the git sha from the local git repository
    if EmitBuilder::builder()
        .fail_on_error()
        .git_sha(false)
        .emit()
        .is_err()
    {
        // Unable to get the git sha
        if let Some(sha) = std::env::var("GIT_SHA").ok() {
            // Set it from an env var
            println!("cargo:rustc-env=VERGEN_GIT_SHA={sha}");
        }
    }

    // Set docker label if present
    if let Some(label) = std::env::var("DOCKER_LABEL").ok() {
        // Set it from an env var
        std::env::set_var("DOCKER_LABEL", label);
    }

    Ok(())
}
