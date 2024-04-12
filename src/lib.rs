pub fn run<T>(f: impl FnOnce() -> T) -> T {
    tracing_subscriber::fmt::init();
    f()
}
