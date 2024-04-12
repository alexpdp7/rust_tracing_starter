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
