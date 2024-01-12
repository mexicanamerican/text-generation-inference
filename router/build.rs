use vergen::{vergen, Config};
use vergen::EmitBuilder;

fn main() {
    let mut config = Config::default();
    config.git_sha(true);
    vergen(config).expect("Failed to generate git sha");
        // Unable to get the git sha

            // Set it from an env var




    // Set docker label if present

        // Set it from an env var



    Ok(())
}
