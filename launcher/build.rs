use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    // Emit cargo and rustc compile time values
    let result = EmitBuilder::builder().cargo().rustc().build();
    if let Err(err) = result {
        eprintln!("Error: {}", err);
        return Err(Box::new(err));
    }

    // Try to get the git sha from the local git repository
    let result = EmitBuilder::builder()
        .fail_on_error()
        .git_short_sha(true)
        .commit_date(true)
        .target_dir(true)
        .timezone(true)
        .to_branch(true)
        .to_hash(true)
        .to_commit_date(true)
        .enable_git2(false)
        .emit();
    if let Err(err) = result {
        eprintln!("Error: {}", err);
        return Err(Box::new(err));
    }

    // Unable to get the git sha
    if let Ok(sha) = std::env::var("GIT_SHA") {
        // Set it from an env var
        println!("cargo:rustc-env=VERGEN_GIT_SHA={sha}");
    }

    // Set docker label if present
    if let Ok(label) = std::env::var("DOCKER_LABEL") {
        // Set it from an env var
        println!("cargo:rustc-env=DOCKER_LABEL={label}");
    }

    Ok(())
}
