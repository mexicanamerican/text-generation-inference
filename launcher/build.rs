if let Ok(label) = std::env::var("DOCKER_LABEL") {
    println!("cargo:rustc-env=DOCKER_LABEL={}", label);
}
