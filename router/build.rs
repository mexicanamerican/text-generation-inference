use axum::handler::get;
use axum::response::Html;
use axum::Router;

pub fn health_routes() -> Router {
    Router::new().route("/", get(|| async { Html("<h1>Healthy</h1>".to_string()) }))
}use axum::handler::get;
use axum::response::Html;
use axum::Router;

pub fn health_routes() -> Router {
    Router::new().route("/", get(|| async { Html("<h1>Healthy</h1>".to_string()) }))
}use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
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
