


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Emit cargo and rustc compile time values
// Calculate build-time variables
let time = chrono::offset::Utc::now();
    

    // Try to get the git sha from the local git repository





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
