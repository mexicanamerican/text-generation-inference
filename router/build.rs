use std::error::Error;
use std::fmt;
use std::io;
use vergen::Error as VergenError;
use failure::Fail;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    // Try to get the git sha from the local git repository
    if EmitBuilder::new()
        .cargo_features(true)
        .cargo_metadata_feature(true)
        .build("FEATURES")
        .context(file!())
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
