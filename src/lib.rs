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

## rust_analyzer::cli::load_cargo::LoadCargoConfig.prefill_caches

不知道缓存存在哪(可能在内存)，分析同样的 cargo 项目加了缓存要 60 秒，不加缓存只要 0.2 秒(因为 ra 是 lazy 的不缓存的话啥也不会干)

由于第一次 load_workspace_at 加上缓存选项也不能加快第二次 load_workspace_at 所以我放弃了缓存
*/
#[test]
fn test_() -> anyhow::Result<()> {
    init_logger();
    let start = std::time::Instant::now();

    let manifest_path: paths::AbsPathBuf = "/home/w/temp/unused_pub_test_case/Cargo.toml".try_into().unwrap();
    let manifest = project_model::ProjectManifest::from_manifest_file(manifest_path)?;
    let workspace = project_model::ProjectWorkspace::load(manifest, &project_model::CargoConfig::default(), &|_| {})?;
    // traverse all cargo_package(members) in cargo_workspace
    for package in workspace.to_roots() {
        // skip std package `/home/w/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library`
        if !package.is_local {
            continue;
        }
        let package_path = package.include[0].clone();
        dbg!(package_path);
    }

    let crate_graph = workspace.to_crate_graph(&mut |_| Vec::new(), &mut {
        let mut counter = 0;
        move |_path| {
            counter += 1;
            Some(vfs::FileId(counter))
        }
    });
    // dbg!(crate_graph);
    let (analysis_host, vfs, _proc_macro_srv_opt) = rust_analyzer::cli::load_cargo::load_workspace(workspace, &rust_analyzer::cli::load_cargo::LoadCargoConfig {
        load_out_dirs_from_check: false,
        with_proc_macro: false,
        prefill_caches: false,
    })?;

    dbg!(start.elapsed());

    // find the excepted crate to analyze unused pub in workspace
    let db = analysis_host.raw_database();
    // for crate_ in hir::Crate::all(db) {
    //     let crate_name = crate_.display_name(db).unwrap();
    //     let crate_name: &str = std::ops::Deref::deref(&crate_name);
    //     dbg!(crate_name);
    // }

    let crate_ = hir::Crate::all(db)
        .into_iter()
        .find(|crate_| {
            let crate_name_ = crate_.display_name(db).unwrap();
            std::ops::Deref::deref(&crate_name_) == "pub_util"
        })
        .unwrap();
    dbg!(start.elapsed());

    

    // stack to simulate DFS recursive
    // LONG_TIME_WARN: crate_.root_module may cost a long time
    let mut modules = vec![crate_.root_module(db)];
    dbg!(start.elapsed());
    let a = analysis_host.analysis();
    // a.find_all_refs(position, search_scope);
    let b = 1;
    
    while let Some(module) = modules.pop() {
        // root crate has no name
        // dbg!(module.name(db));
        for define in module.declarations(db) {
            match define {
                hir::ModuleDef::Function(func) => {
                    if hir::HasVisibility::visibility(&func, db) != hir::Visibility::Public {
                        continue;        
                    }
                    dbg!(func);
                    // func.
                },
                _ => continue
            }
        }
        modules.extend(module.children(db));
    }

    dbg!(start.elapsed());
    Ok(())
}
