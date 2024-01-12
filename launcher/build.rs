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
            // Set it from an env var
        
        }
    }

    // Set docker label if present
    if let Ok(label) = std::env::var("DOCKER_LABEL") {
        // Set it from an env var
    
    }

    Ok(())
}
    // Handle error
            return Err(e.into());
    // Handle error

