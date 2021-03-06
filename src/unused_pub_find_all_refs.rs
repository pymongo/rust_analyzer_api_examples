//! find unused pub in workspace by ra find_all_refs API

/*
todo: where is prefill_caches storage? is prefill_caches working?

## rust_analyzer::cli::load_cargo::LoadCargoConfig.prefill_caches

不知道缓存存在哪(可能在内存)，分析同样的 cargo 项目加了缓存要 60 秒，不加缓存只要 0.2 秒(因为 ra 是 lazy 的不缓存的话啥也不会干)

由于第一次 load_workspace_at 加上缓存选项也不能加快第二次 load_workspace_at 所以我放弃了缓存

为啥 find_all_refs 的返回值跟 vscode 里面操作不一样?
*/
#[test]
fn run() {
    use super::common::ra_load_manifest_path;
    let path = "/home/w/temp/unused_pub_test_case/Cargo.toml";
    let host = ra_load_manifest_path(path);
    let db = host.raw_database();
    let analysis = host.analysis();
    let sem = hir::Semantics::new(db);

    // find the excepted crate to analyze unused pub in workspace
    for crate_ in hir::Crate::all(db) {
        let file = crate_.root_file(db);
        dbg!(file);
        let crate_name = crate_.display_name(db).unwrap();
        let crate_name: &str = std::ops::Deref::deref(&crate_name);
        dbg!(crate_name);
    }

    let crate_ = hir::Crate::all(db)
        .into_iter()
        .find(|crate_| {
            let crate_name_ = crate_.display_name(db).unwrap();
            std::ops::Deref::deref(&crate_name_) == "pub_util"
        })
        .unwrap();
    let crate_id: base_db::CrateId = unsafe { std::mem::transmute(crate_) };
    dbg!(&crate_id);
    let crate_graph = base_db::SourceDatabase::crate_graph(db);
    let mut file_ids = vec![];
    for crate_id_ in crate_graph.iter() {
        dbg!(crate_id_);
        // search_scope skip pub fn inside crate
        // if crate_id_ == crate_id {
        //     continue;
        // }
        let file_id = crate_graph[crate_id].root_file_id;
        file_ids.push(file_id);
    }
    let search_scope = Some(ide_db::search::SearchScope::files(&file_ids));
    dbg!(&search_scope);

    // stack to simulate DFS recursive
    // LONG_TIME_WARN: crate_.root_module may cost a long time
    let mut modules = vec![crate_.root_module(db)];
    // analysis.file

    // 方案 2 用 analysis.annotations(fileID) 去扫描每个文件的 method reference count
    // source_file.syntax().descendants().filter_map(|it| method_range(it, file_id)).collect()
    while let Some(module) = modules.pop() {
        // root crate has no name
        for define in module.declarations(db) {
            match define {
                hir::ModuleDef::Function(func) => {
                    if hir::HasVisibility::visibility(&func, db) != hir::Visibility::Public {
                        continue;
                    }
                    let _fn_in_module = hir::Function::module(func, db);
                    let fn_in_file = hir::HasSource::source(func, db).unwrap();
                    let file_id = fn_in_file.file_id.original_file(db);
                    let fn_ = fn_in_file.value;
                    let fn_name = syntax::ast::HasName::name(&fn_).unwrap();

                    let token = fn_.fn_token().unwrap();
                    let def = ide_db::defs::Definition::from_token(&sem, &token);
                    dbg!(def);
                    let offset = token.text_range().start();
                    // find_all_refs 可能无法分析过程宏生成的代码，除非启用 ra 的 proc_macro_server ?
                    println!("scanning pub fn `{}`, find_all_refs...", fn_name);
                    // find_all_refs 的 search_scope 如果不是 None 就会将默认的搜索结果跟 search_scope 入参 intersect 一下
                    //analysis.call_hierarchy(position)
                    // analysis.resolve_annotation(annotation)
                    let find_usage = analysis
                        .find_all_refs(base_db::FilePosition { file_id, offset }, None)
                        .unwrap();

                    // #[tokio::main] may has None references?
                    if let Some(usage) = find_usage {
                        if usage.is_empty() {
                            println!("WARN: Find unused pub fn `{}` in workspace", fn_name);
                            // dbg!(fn_in_module, fn_name);
                        }
                    }
                }
                _ => continue,
            }
        }
        modules.extend(module.children(db));
    }
}
