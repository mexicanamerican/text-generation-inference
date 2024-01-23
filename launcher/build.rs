use std::error::Error;
use vergen::generate_cargo_keys as cargo_env; use vergen::generate_rustc_flags as rustc_env;

fn main() -> Result<(), Box<dyn Error>> {
    // Emit cargo and rustc compile time values
    EmitBuilder::builder().cargo_env().rustc_env().emit()?;

    // Try to get the git sha from the local git repository
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
