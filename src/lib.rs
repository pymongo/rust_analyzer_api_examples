#![doc = include_str!("../README.md")]
#![feature(rustc_private)]
extern crate rustc_lexer;

#[cfg(test)]
mod misc_code_snippets;

/**
salsa crate: kv store, for source code incremental computation
*/
#[cfg(test)]
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

#[cfg(test)]
#[cfg(not)]
/// easy to debug which env not set
fn env(key: &str) -> String {
    match std::env::var(key) {
        Ok(val) => val,
        Err(_) => panic!("env {} not set!", key),
    }
}

/*
todo: where is prefill_caches storage? is prefill_caches working?
*/
#[test]
fn test_() -> anyhow::Result<()> {
    init_logger();
    let start = std::time::Instant::now();

    // load_workspace_at may cost 369s
    let cargo_workspace = "/home/w/temp/unused_pub_test_case/Cargo.toml";
    // let crate_name = env("CRATE");
    // let crate_name = "pub_util";
    let (analysis_host, _vfs, _proc_macro_server_opt) =
        rust_analyzer::cli::load_cargo::load_workspace_at(
            std::path::Path::new(&cargo_workspace),
            &project_model::CargoConfig::default(),
            &rust_analyzer::cli::load_cargo::LoadCargoConfig {
                load_out_dirs_from_check: false,
                with_proc_macro: false,
                prefill_caches: true,
            },
            &|_| {},
        )?;
    dbg!(start.elapsed());

    // find the excepted crate to analyze unused pub in workspace
    // let db = analysis_host.raw_database();
    // let crate_ = hir::Crate::all(db)
    //     .into_iter()
    //     .find(|crate_| {
    //         let crate_name_ = crate_.display_name(db).unwrap();
    //         std::ops::Deref::deref(&crate_name_) == crate_name
    //     })
    //     .unwrap();
    // dbg!(crate_);
    // dbg!(start.elapsed());

    Ok(())
}
