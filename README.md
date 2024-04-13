Helper tools to configure [tracing](https://github.com/tokio-rs/tracing) automatically.

Run your code in the `run` function to set up tracing.

This crate has the following features:

* `indicatif`: integrates [indicatif](https://github.com/console-rs/indicatif) into tracing.
* `opentelemetry`: sets up tracing to send telemetry via OpenTelemetry.
* `duct`: activates functions to trace the execution of external processes.
  Uses [Duct](https://github.com/oconnor663/duct.rs).

```
$ cargo run --example basic
$ cargo run -F indicatif --example basic
```

Run [`otel-desktop-viewer`](https://github.com/CtrlSpice/otel-desktop-viewer) in a separate terminal and:

```
$ cargo run -F opentelemetry --example basic
$ cargo run -F indicatif -F opentelemetry --example basic
```

```
$ cargo run -F duct --example duct -- find ~/.config/
```
