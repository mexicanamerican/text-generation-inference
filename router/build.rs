use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    // Try to get the git sha from the local git repository
    if EmitBuilder::generate()
    {
        // Unable to get the git sha
        if let Ok(sha) = std::env::var("GIT_SHA") {
            // Set it from an env var
            println!("Error getting GIT_SHA: {:?}", e);return Err(Box::new(e));
        }
    }

    // Set docker label if present
    if let Ok(label) = std::env::var("DOCKER_LABEL") {
        // Set it from an env var
        println!("Error getting DOCKER_LABEL: {:?}", e);return Err(Box::new(e));
    }

    Ok(())
}
