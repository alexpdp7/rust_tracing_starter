fn main() -> color_eyre::eyre::Result<()> {
    rust_tracing_starter::run(|| {
        rust_tracing_starter::observe_duct(
            "cmd",
            std::env::args().skip(1).collect::<Vec<_>>().as_slice(),
        )
    })?;
    Ok(())
}
