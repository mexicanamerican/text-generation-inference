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
        if let Ok(sha) = std::env::var("GIT_SHA") {
            // Set the Docker label from an environment variable
            println!("cargo:rustc-env=VERGEN_GIT_SHA={sha}");
        }
    }

    // Check if the Docker label environment variable is present
    if let Ok(label) = std::env::var("DOCKER_LABEL") {
        // Print the cargo:rustc-env command to set the DOCKER_LABEL environment variable
        println!("cargo:rustc-env=DOCKER_LABEL={label}");
    }

    Ok(())
}
