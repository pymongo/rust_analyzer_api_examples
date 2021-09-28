#[test]
fn run() {
    use super::common::load_cargo_toml_to_workspace;
    let start = std::time::Instant::now();
    let path = "/home/w/temp/unused_pub_test_case/crates/pub_util/Cargo.toml";
    let workspace = load_cargo_toml_to_workspace(path);
    let (host, vfs, _proc_macro_srv_opt) = rust_analyzer::cli::load_cargo::load_workspace(
        workspace,
        &rust_analyzer::cli::load_cargo::LoadCargoConfig {
            load_out_dirs_from_check: false,
            with_proc_macro: false,
            prefill_caches: false,
        },
    )
    .unwrap();
    let db = host.raw_database();
    let analysis = host.analysis();
    let sem = hir::Semantics::new(db);
    dbg!(start.elapsed());

    let annotations_config = ide::AnnotationConfig {
        binary_target: false,
        annotate_runnables: false,
        annotate_impls: false,
        annotate_references: false,
        annotate_method_references: true,
    };
    let std_lib_path_prefix: paths::AbsPathBuf = "/home/w/.rustup".try_into().unwrap();
    let std_lib_path_prefix = vfs::VfsPath::from(std_lib_path_prefix);
    for (file_id, path) in vfs.iter() {
        if path.starts_with(&std_lib_path_prefix) {
            continue;
        }
        dbg!(path);
        dbg!(start.elapsed());
        let annotations = analysis.annotations(&annotations_config, file_id).unwrap();
        // rustconf demo 提到的 bottom_up 解析方法，从 file+offset -> token, 与之相反的是从 crates->token_in_file
        let file = sem.parse(file_id);
        let ast_node = syntax::AstNode::syntax(&file);
        dbg!(start.elapsed());
        for annotation in annotations {
            let (position, refs_count) = match annotation.kind {
                ide::AnnotationKind::HasReferences { position, data } => {
                    // FIXME refs_count 总是为 0
                    let refs_count = data.unwrap_or_default().len();
                    (position, refs_count)
                }
                _ => continue,
            };
            // dbg!(start.elapsed());
            let offset = annotation.range.start();
            let token1 = ast_node.token_at_offset(offset);
            dbg!(token1);
            let token2 = ast_node.token_at_offset(position.offset);
            dbg!(token2);
            dbg!(refs_count);

            // let a = syntax::ast::NameOwner::name(token);
            // dbg!(annotation);
            dbg!(start.elapsed());
        }
    }
}
