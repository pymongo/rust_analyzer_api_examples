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
        ack_seq_id: 1,
        ra_req_sender: ra_child_process.stdin.take().unwrap(),
    };
    let path = "/home/w/temp/unused_pub_test_case";
    let project_root = lsp_types::Url::parse(&format!("file://{}", path)).unwrap();
    #[allow(deprecated)]
    let init_params = lsp_types::InitializeParams {
        process_id: Some(pid as u32),
        root_path: None,
        root_uri: Some(project_root),
        initialization_options: todo!(),
        capabilities: todo!(),
        trace: Some(lsp_types::TraceOption::Off),
        workspace_folders: Some(vec![lsp_types::WorkspaceFolder {
            uri: project_root,
            name: "unused_pub_test_case".to_string(),
        }]),
        client_info: Some(lsp_types::ClientInfo {
            name: "rust_analyzer_api_examples".to_string(),
            version: Some("0.1.0".to_string()),
        }),
        locale: Some("en-us".to_string()),
    };
    req_helper.send_req("initialize", serde_json::to_value(init_params).unwrap());

    let _exit_code = ra_child_process.wait().unwrap();
    // assert!(exit_code.success());
}

struct ReqHelper {
    /// similar to TCP sequence num
    ack_seq_id: u32,
    ra_req_sender: std::process::ChildStdin,
}

impl ReqHelper {
    fn send_req(&mut self, method: &str, params: serde_json::Value) {
        let req = json!({
            "jsonrpc": "2.0",
            "id": self.ack_seq_id,
            "method": method,
            "params": params
        });
        self.ra_req_sender
            .write_all(serde_json::to_string(&req).unwrap().as_bytes())
            .unwrap();
        self.ack_seq_id += 1;
    }
}
