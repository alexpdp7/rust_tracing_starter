#![doc = include_str!("../README.md")]

use tracing_subscriber::prelude::*;

/// Runs `f` with tracing automatically configured.
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
#[tracing::instrument(err)]
/// Runs `cmd` using [Duct](https://github.com/oconnor663/duct.rs), adding instrumentation.
/// Each line of output creates a span.
///
/// Warning: in release mode without any features enabled, this adds about one second of runtime for every 50,000 lines of output.
pub fn observe_duct(id: &str, cmd: &[String]) -> std::io::Result<()> {
    use std::io::BufRead;

    let (program, args) = cmd.split_at(1);
    let mut span = tracing::info_span!("output", value = tracing::field::Empty);
    tracing::dispatcher::get_default(|d| {
        d.enter(&span.id().unwrap());
    });
    for l in
        std::io::BufReader::new(duct::cmd(&program[0], args).stderr_to_stdout().reader()?).lines()
    {
        match l {
            Ok(l) => {
                span.record("value", l.clone());
                tracing::info!("{}", l);
                tracing::dispatcher::get_default(|d| {
                    d.exit(&span.id().unwrap());
                    d.try_close(span.id().unwrap());
                });
                span = tracing::info_span!("output", value = tracing::field::Empty);
                tracing::dispatcher::get_default(|d| {
                    d.enter(&span.id().unwrap());
                });
            }
            Err(e) => {
                tracing::dispatcher::get_default(|d| {
                    d.exit(&span.id().unwrap());
                    d.try_close(span.id().unwrap());
                });
                return Err(e);
            }
        }
    }

    Ok(())
}
