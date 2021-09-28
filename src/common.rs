fn init_logger() {
    // 用了 EnvFilter 就不影响用 with_max_level 以免日志过滤规则被互相覆盖掉
    let filter = tracing_subscriber::filter::EnvFilter::default()
        .add_directive(tracing::Level::TRACE.into())
        // salsa crate: kv store, for source code incremental computation
        .add_directive("salsa=warn".parse().unwrap())
        // ignore crate graph debug output
        .add_directive("rust_analyzer::cli::load_cargo=info".parse().unwrap())
        // vfs 类似 git 用于记录文件改动，vfs_notify 类似于 syscall inotify
        .add_directive("vfs_notify=info".parse().unwrap())
        .add_directive("hir_expand::db=info".parse().unwrap())
        // ignore `Dependency { crate_id`
        .add_directive("ide_db::apply_change=warn".parse().unwrap())
        .add_directive("hir_def=info".parse().unwrap());
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

pub(crate) fn load_cargo_toml_to_workspace(manifest_path: &str) -> project_model::ProjectWorkspace {
    init_logger();
    let manifest_path: paths::AbsPathBuf = manifest_path.try_into().unwrap();
    let manifest = project_model::ProjectManifest::from_manifest_file(manifest_path).unwrap();
    let workspace = project_model::ProjectWorkspace::load(
        manifest,
        &project_model::CargoConfig::default(),
        &|_| {},
    )
    .unwrap();

    // traverse all cargo_package(members) in cargo_workspace
    for package in workspace.to_roots() {
        // skip std package `/home/w/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library`
        if !package.is_local {
            continue;
        }
        let package_path: &std::path::Path = package.include[0].as_ref();
        println!("found package {}", package_path.to_str().unwrap());
    }
    workspace
}

/**
## get CrateGraph Example 1: ProjectWorkspace -> CrateGraph
```no_run
let crate_graph = workspace.to_crate_graph(&mut |_| Vec::new(), &mut {
    let mut counter = 0;
    move |_path| {
        counter += 1;
        Some(vfs::FileId(counter))
    }
});
```

## get CrateGraph Example 2
```no_run
let crate_graph = base_db::SourceDatabase::crate_graph(db);
```
*/
pub(crate) fn ra_load_manifest_path(path: &str) -> ide::AnalysisHost {
    let workspace = load_cargo_toml_to_workspace(path);
    let (analysis_host, _vfs, _proc_macro_srv_opt) =
        rust_analyzer::cli::load_cargo::load_workspace(
            workspace,
            &rust_analyzer::cli::load_cargo::LoadCargoConfig {
                load_out_dirs_from_check: false,
                with_proc_macro: false,
                prefill_caches: false,
            },
        )
        .unwrap();
    analysis_host
}
