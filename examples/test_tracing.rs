#[tracing::instrument]
fn parent_task2(subtasks: usize, b: &str) -> i32 {
    // tracing::info!("spawning subtasks...");
    2
}

fn main() {
    // tracing_subscriber::fmt()
    // .with_max_level(tracing::Level::TRACE)
    // .init();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .try_init()
        .unwrap();
    parent_task2(2, "");
}
