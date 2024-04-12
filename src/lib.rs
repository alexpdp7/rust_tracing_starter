pub fn run<T>(f: impl FnOnce() -> T) -> T {
    #[cfg(not(feature = "indicatif"))]
    return {
        tracing_subscriber::fmt::init();
        f()
    };

    #[cfg(feature = "indicatif")]
    return {
        use tracing_indicatif::IndicatifLayer;
        use tracing_subscriber::prelude::*;

        let indicatif_layer = IndicatifLayer::new();

        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().with_writer(indicatif_layer.get_stderr_writer()))
            .with(indicatif_layer)
            .init();

        f()
    };
}
