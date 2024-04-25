# Optional Log

## What

This crate provides an optional wrapper around the ["log" crate](https://crates.io/crates/log),
which allows you to provide an optional "log" feature for you crates easily.

## How

In your "Cargo.toml":

```toml
[dependencies]
log = { version = "0.4", optional = true }
optional-log = "0.1"

[feature]
log = ["dep:log", "optional-log/log"]
```

Then use macros of "optional-log" crate instead of those of the ["log" crate](https://crates.io/crates/log).

In this way, once the "log" feature of your crate is enabled by downstream, these macros will be expanded to
the corresponding macros of the ["log" crate](https://crates.io/crates/log), otherwise they do nothing.

The `log_enabled!` macro will always return `false` if the "log" feature is not enabled.
