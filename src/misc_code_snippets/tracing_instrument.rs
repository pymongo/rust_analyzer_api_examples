#[derive(Debug)]
struct Args {
    b: i32,
}

/**
span 就是编译原理的概念大意就是源码文件名/行号/源码字符串的偏移及长度这种上下文信息

instrument 的好处是自动生成 span 但需要触发收集 span 的行为时才能打印出来例如:
1. 函数调用任意 log 宏
2. 用 SpanTrace 来收集
3. tracing::collect::with_default // 老版本叫 tracing::subscriber
...

可以给源码搜 nth_fibonacci 的例子学习下
TODO 不知道能不能像 uftrace 那样可视化树形打印 callgraph
*/
#[tracing::instrument]
fn f(_a: &str, b: Args) {
    log::info!("enter f");
    // g 函数内的 instrument 前缀不仅有 g() 还会带上 f()
    // f{_a="a" b=Args { b: 1 }}:g{b=Args { b: 1 }}:
    g(&b);
}

/// 用 log/tracing 的 info! 宏都可以，只不过 tracing 宏高级点支持像 femme 这样的 kv log
/// 我们都知道 rust-lang/log 只提供了 logger trait 的定义和日志宏，没有 logger 实现，不知道以后会不会有 trait KVLogger
#[tracing::instrument]
fn g(b: &Args) {
    // enter g b.b=1
    tracing::info!(message = "enter g", %b.b);
    assert!(b.b == 1);
    // 线程 spawn 用 线程名字记录，tokio spawn 用 InstrumentFuture 传入 span 记录
    std::thread::Builder::new()
        .name("working".to_string())
        .spawn(|| {
            // TODO 暂时不知 span! 除了在 tokio::spawn 中有用还有啥用
            let span = tracing::span!(tracing::Level::INFO, "my span");
            let _ = span.enter();

            tracing::info!("in thread");
        })
        .unwrap()
        .join()
        .unwrap();
}

/**
## tracing::instrument example
```text
Sep 26 14:28:14.851  INFO f{_a="a" b=Args { b: 1 }}: rust_analyzer_api::misc_code_snippets::tracing_instrument: enter f
Sep 26 14:28:14.851  INFO f{_a="a" b=Args { b: 1 }}:g{b=Args { b: 1 }}: rust_analyzer_api::misc_code_snippets::tracing_instrument: enter g
```

##
*/
#[test]
fn run() {
    tracing_subscriber::fmt()
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();
    // tracing::subscriber::with_default(subscriber, f)
    f("a", Args { b: 1 });
}

/// env_logger would not log span in tracing::instrument
#[test]
fn env_logger_log_instrument() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    f("a", Args { b: 1 });
}
