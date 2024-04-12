#[tracing::instrument]
fn instrumented() {
    std::thread::sleep(std::time::Duration::from_secs(1));
    tracing::error!("error in instrumented function");
}

fn main() {
    rust_tracing_starter::run(|| {
        tracing::error!("print an error");
        let span = tracing::span!(tracing::Level::ERROR, "example span");
        let _guard = span.enter();
        tracing::error!("error in span");
        instrumented()
    })
}
