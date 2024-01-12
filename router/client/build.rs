fn main() {
    tonic_build::configure()
        .build_client(true)
        .compile(&["../pb/router.proto"], &["../pb"])
        .unwrap();
}
        .build_server(false)
        .out_dir("src/pb")
        .include_file("mod.rs")
        .compile_with_config(config, &["../../proto/generate.proto"], &["../../proto"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {e}"));

    Ok(())
}
