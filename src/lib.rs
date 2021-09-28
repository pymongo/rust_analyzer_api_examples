#![doc = include_str!("../README.md")]
#![feature(rustc_private)]
// extern crate rustc_graphviz;
extern crate rustc_lexer;

#[cfg(test)]
mod common;
#[cfg(test)]
mod graphviz_render_crate_graph;
#[cfg(test)]
mod misc_code_snippets;
#[cfg(test)]
mod print_all_crates_in_workspace;
#[cfg(test)]
mod unused_pub_analysis_annotations;
#[cfg(test)]
mod unused_pub_find_all_refs;
#[cfg(test)]
mod unused_pub_ra_lsp_client;
