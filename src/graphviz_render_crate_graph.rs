#[test]
fn run() {
    let path = "/home/w/temp/unused_pub_test_case/Cargo.toml";
    let path = "/home/w/repos/atlas/atlas";
    let path = "/home/w/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_driver/Cargo.toml";
    let path = "/home/w/repos/clone_repos/axum";
    let (analysis_host, _vfs, _proc_macro_srv_opt) =
        rust_analyzer::cli::load_cargo::load_workspace_at(
            std::path::Path::new(path),
            &project_model::CargoConfig::default(),
            &rust_analyzer::cli::load_cargo::LoadCargoConfig {
                // WARN: load_out_dirs_from_check may cost 10G+ memory
                load_out_dirs_from_check: false,
                with_proc_macro: false,
                prefill_caches: false,
            },
            &|_| {},
        )
        .unwrap();
    let analysis = analysis_host.analysis();
    let is_include_std_and_dependencies_crate = false;
    let dot = analysis
        .view_crate_graph(is_include_std_and_dependencies_crate)
        .unwrap()
        .unwrap();
    // KGraphViewer only recognize *.gv graphviz script
    let file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/target/graph.gv");
    dbg!(&file_path);
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(concat!(env!("CARGO_MANIFEST_DIR"), "/target/graph.gv"))
        .unwrap();
    std::io::Write::write_all(&mut f, dot.as_bytes()).unwrap();
    drop(f);
    let is_success = std::process::Command::new("xdot")
        .arg(file_path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success();
    assert!(is_success);
    // let db = analysis_host.raw_database();
    // let crate_graph = base_db::SourceDatabase::crate_graph(db);
}
