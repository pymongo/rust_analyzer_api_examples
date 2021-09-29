/*!
1. project_workspace.to_crate_graph
2. hir::Crate::all(db)
3. base_db::SourceDatabase::crate_graph(db)
*/
use super::common::ra_load_manifest_path;

#[test]
fn list_crates_by_db() {
    let manifest_path = "/home/w/repos/clone_repos/rust-analyzer/Cargo.toml";
    let host = ra_load_manifest_path(manifest_path);
    let db = host.raw_database();
    let crate_graph = base_db::SourceDatabase::crate_graph(db);
    traverse_and_print_crate_graph(crate_graph);
}

fn traverse_and_print_crate_graph(graph: std::sync::Arc<base_db::CrateGraph>) {
    for key in graph.iter() {
        let crate_ = &graph[key];
        println!("{}", crate_.display_name.clone().unwrap());
        for dep in &crate_.dependencies {
            println!("  {}", dep.name);
        }
    }
}
