#![doc = include_str!("../README.md")]
#![feature(rustc_private)]
extern crate rustc_lexer;

#[cfg(test)]
mod toy_calculator;

/*
todo: where is prefill_caches storage? is prefill_caches working?
*/
#[test]
fn test_() -> anyhow::Result<()> {
    let start = std::time::Instant::now();

    // load_workspace_at may cost 369s
    let cargo_workspace = "/home/w/repos/atlas/atlas";
    let crate_name = "common";
    let (analysis_host, _vfs, _proc_macro_server_opt) =
        rust_analyzer::cli::load_cargo::load_workspace_at(
            std::path::Path::new(cargo_workspace),
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
    let db = analysis_host.raw_database();
    let crate_ = hir::Crate::all(db).into_iter().find(|crate_| {
        let crate_name_ = crate_.display_name(db).unwrap();
        std::ops::Deref::deref(&crate_name_) == crate_name
    }).unwrap();
    dbg!(crate_);
    dbg!(start.elapsed());
        
    Ok(())
}
