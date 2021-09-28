//! LSP 的原理是 通过 stdin/stdout(管道 uri: todo!(), name: todo!()  uri: todo!(), name: todo!() ) 跟 rust-analyzer executable 文件进行通信
//! 当然 LSP 用 socket 通信也行，只不过 LSP 用 pipe 通信在客户端和服务端都在单机上性能会好于 socket
//! 改下 ra 源码 Log 每一个 request 和 response 的 json 方便学习

use serde_json::json;
use std::io::{BufRead, Write};

#[link(name = "c")]
extern "C" {
    fn getpid() -> i32;
}

/**

*/
#[test]
fn ra_lsp_client() {
    let pid = unsafe { getpid() };
    let mut ra_child_process = std::process::Command::new("rust-analyzer")
        .env("RA_LOG", "trace")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let ra_stout = ra_child_process.stdout.take().unwrap();
    // let (tx, rx) = std::sync::mpsc::sync_channel(1);
    let ra_resp_reader = std::io::BufReader::new(ra_stout);
    let _join_handle = std::thread::spawn(move || {
        for line_res in ra_resp_reader.lines() {
            let line = line_res.unwrap();
            dbg!(line);
        }
    });

    let mut req_helper = ReqHelper {
        req_id: 1,
        ra_req_sender: ra_child_process.stdin.take().unwrap(),
    };
    let path = "/home/w/temp/unused_pub_test_case";
    let project_root = lsp_types::Url::parse(&format!("file://{}", path)).unwrap();
    #[allow(deprecated)]
    let init_params = lsp_types::InitializeParams {
        process_id: Some(pid as u32),
        root_path: None,
        root_uri: Some(project_root.clone()),
        /* RA source
        let mut config = Config::new(root_path, initialize_params.capabilities);
        if let Some(json) = initialize_params.initialization_options {
            config.update(json);
        }
        */
        initialization_options: None,
        capabilities: lsp_types::ClientCapabilities {
            workspace: Some(lsp_types::WorkspaceClientCapabilities {
                apply_edit: Some(false),
                configuration: Some(false),
                ..Default::default()
            }),
            text_document: Some(lsp_types::TextDocumentClientCapabilities {
                references: Some(lsp_types::DynamicRegistrationClientCapabilities {
                    dynamic_registration: Some(true),
                }),
                ..Default::default()
            }),
            window: None,
            general: None,
            offset_encoding: None,
            experimental: Some(json!({
                "commands": {
                    "commands": [
                        "rust-analyzer.showReferences",
                    ]
                }
            })),
        },
        trace: Some(lsp_types::TraceOption::Off),
        workspace_folders: Some(vec![lsp_types::WorkspaceFolder {
            uri: project_root,
            name: "unused_pub_test_case".to_string(),
        }]),
        client_info: Some(lsp_types::ClientInfo {
            name: "lsp client".to_string(),
            version: Some("0.1.0".to_string()),
        }),
        locale: Some("en-us".to_string()),
    };
    req_helper.send_req("initialize", serde_json::to_value(init_params).unwrap());
    req_helper.send_req("initialized", json!({}));

    // FIXME RA server not handle codeLens/resolve req

    // pub fn used_pub() {}
    // range 是函数名字/ident 的字符偏移范围
    req_helper.send_req(
        "codeLens/resolve",
        json!({
            "range": {
                "start": {
                    "line": 0,
                    "character": 7
                },
                "end": {
                    "line": 0,
                    "character": 15
                }
            },
            "data": {
                "references": {
                    "textDocument": {
                        "uri": "file:///home/w/temp/unused_pub_test_case/crates/pub_util/src/lib.rs"
                    },
                    "position": {
                        "line": 1,
                        "character": 7
                    }
                }
            }
        }),
    );
    // pub fn unused_pub() {}
    req_helper.send_req(
        "codeLens/resolve",
        json!({
            "range": {
                "start": {
                    "line": 1,
                    "character": 7
                },
                "end": {
                    "line": 1,
                    "character": 17
                }
            },
            "data": {
                "references": {
                    "textDocument": {
                        "uri": "file:///home/w/temp/unused_pub_test_case/crates/pub_util/src/lib.rs"
                    },
                    "position": {
                        "line": 1,
                        "character": 7
                    }
                }
            }
        }),
    );

    let _exit_code = ra_child_process.wait().unwrap();
    // assert!(exit_code.success());
}

struct ReqHelper {
    /// similar to TCP sequence num
    req_id: u32,
    ra_req_sender: std::process::ChildStdin,
}

impl ReqHelper {
    /// send req or notify
    fn send_req(&mut self, method: &str, params: serde_json::Value) {
        let req = json!({
            // "jsonrpc": "2.0",
            "id": self.req_id,
            "method": method,
            "params": params
        });
        self.ra_req_sender
            .write_all(serde_json::to_string(&req).unwrap().as_bytes())
            .unwrap();
        self.req_id += 1;
        println!("send req to server success");
    }
}
