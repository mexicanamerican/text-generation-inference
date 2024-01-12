fn main() -> Result<(), Box<dyn std::error::Error>> { // Add error handling
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
            println!("cargo:rustc-env=VERGEN_GIT_SHA={sha}");
        } else {
            // Log the error
            eprintln!("Failed to get git sha");
        }
    }

    // Set docker label if present
    if let Ok(label) = std::env::var("DOCKER_LABEL") {
        // Set it from an env var
        println!("cargo:rustc-env=DOCKER_LABEL={label}");
    }

    Ok(())
}
    // Log the error
    eprintln!("Error during build process: {:?}", e);
    return Err(e.into());
    // Log the error
    eprintln!("Error during build process: {:?}", e);
    return Err(e.into());
