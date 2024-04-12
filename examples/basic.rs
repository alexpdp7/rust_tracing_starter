#[tracing::instrument]
fn instrumented() {
    tracing::error!("error in instrumented function");
}

fn main() {
    rust_tracing_starter::run(|| {
        tracing::error!("print an error");
        let span = tracing::span!(tracing::Level::ERROR, "example span");
        let _ = span.enter();
        tracing::error!("error in span");
        instrumented();
    });
}
