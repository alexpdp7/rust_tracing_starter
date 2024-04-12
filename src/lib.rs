use tracing_subscriber::prelude::*;

pub fn run<T, E>(f: impl FnOnce() -> Result<T, E>) -> color_eyre::eyre::Result<T, E> {
    color_eyre::install().unwrap();

    let mut layers = Vec::new();

    #[cfg(not(feature = "indicatif"))]
    layers.push(tracing_subscriber::fmt::layer().boxed());

    #[cfg(feature = "indicatif")]
    {
        use tracing_indicatif::IndicatifLayer;
        use tracing_subscriber::prelude::*;

        let indicatif_layer = IndicatifLayer::new();

        let layer = tracing_subscriber::fmt::layer()
            .with_writer(indicatif_layer.get_stderr_writer())
            .boxed();
        layers.push(layer);
        layers.push(indicatif_layer.boxed());
    };

    #[cfg(feature = "opentelemetry")]
    {
        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(opentelemetry_otlp::new_exporter().http())
            .install_simple()
            .unwrap();

        let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer).boxed();
        layers.push(telemetry_layer);
    }

    #[allow(clippy::let_and_return)] // https://github.com/rust-lang/rust-clippy/pull/12558/
    tracing::subscriber::with_default(tracing_subscriber::registry().with(layers), || {
        let result = f();

        #[cfg(feature = "opentelemetry")]
        opentelemetry::global::shutdown_tracer_provider();
        result
    })
}

#[cfg(feature = "duct")]
#[tracing::instrument]
pub fn observe_duct(id: &str, cmd: &[String]) -> std::io::Result<()> {
    use std::io::BufRead;

    let (program, args) = cmd.split_at(1);
    for l in
        std::io::BufReader::new(duct::cmd(&program[0], args).stderr_to_stdout().reader()?).lines()
    {
        tracing::info!("{}", l?);
    }

    Ok(())
}
