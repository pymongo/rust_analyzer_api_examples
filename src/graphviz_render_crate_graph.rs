use super::common::ra_load_manifest_path;

#[test]
fn run() {
    // let manifest_path = "/home/w/temp/unused_pub_test_case/Cargo.toml";
    // let manifest_path = "/home/w/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/rustc-src/rust/compiler/rustc_driver/Cargo.toml";
    let manifest_path = "/home/w/repos/clone_repos/rust-clippy/Cargo.toml";
    let host = ra_load_manifest_path(manifest_path);
    let analysis = host.analysis();

    let is_include_std_and_dependencies_crate = false;
    let dot = analysis
        .view_crate_graph(is_include_std_and_dependencies_crate)
        .unwrap()
        .unwrap();
    // KGraphViewer only recognize *.gv graphviz script
    let file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/target/graph.gv");
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
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
}
