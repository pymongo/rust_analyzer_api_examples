
Inspire by [Rust Code Analysis 实践 - RustChinaConf2020](https://github.com/rustcc/RustChinaConf2020/blob/master/rustchinaconf2020/RustChinaConf2020-28.%E5%B0%B9%E6%80%9D%E7%BB%B4-%E3%80%8ARust%20Code%20Analysis%20%E5%AE%9E%E8%B7%B5%E3%80%8B.pdf)

## How bo build

Since chinese network pretty slow to github.com, I download rust-analyzer source on disk.

Make sure rust-analyzer repo dir on parent directory(you can use symbolic link to link rust-analyzer repo dir),

because Cargo.toml would search `rust-analyzer = { path = "../rust-analyzer/crates/rust-analyzer" }`

## the problem in RustChinaConf2020 demo

最大问题就是没做完(仅仅打印并没 FindUsage)，仅仅是 BFS 打印某个 crate 的所有 pub fn 并没有 FindReference/FindUsage 分析未使用的 pub fn

demo 中 bottom-up 指的是光标放到某个源文件的第几行，然后分析这是个什么类型的 token

top-down 就是从 crate root 项目根目录开始遍历，一直遍历到每个文件的每个 token

### test environment

cargo workspace line of code: 42683

crate X in workspace line of code: 6761

analyze unused `pub` in crate X: 491.621485211s

#### too slow

almost cost 10 minute to analyze `unused pub` in one crate on workspace

#### use multi-core CPU to analyze?

RustChinaConf2020 demo only use one CPU core to analyze too slow 

#### false positive

I see some false positive?

#### ra API change a lot

the RustChinaConf2020 demo use ra API is quite different to master branch

## ra 源码学习指南

资料: docs/dev (contributors docs) 和 youtube 上的作者录制的 rust-analyzer explain 系列视频

学下别人 PR 怎么给 ra 加新的命令: <https://github.com/rust-analyzer/rust-analyzer/pull/10181/files>

学下 LSP 协议和 pub enum RustAnalyzerCmd (这个好像是 executable 文件的命令不是 vscode LSP 的命令集合)

LSP 协议通信的数据包格式是 JSON 

通信的信道是 vscode/IDE 跟 ra 可执行文件之间建立双向管道，LSP 客户端往 ra 可执行文件的 STDIN 写入 JSON 字符串即为发请求

> let (initialize_id, initialize_params) = connection.initialize_start()

> crates/rust-analyzer/src/handlers.rs, pub fn lens

> crates/ide_db/src/search.rs

## Reference

- <https://rust-analyzer.github.io/blog/2019/11/13/find-usages.html>
- <https://github.com/rustcc/RustChinaConf2020/blob/master/rustchinaconf2020>
