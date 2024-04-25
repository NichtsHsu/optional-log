//! # What
//!
//! This crate provides an optional wrapper around the ["log" crate](https://crates.io/crates/log),
//! which allows you to provide an optional "log" feature for you crates easily.
//!
//! # How
//!
//! In your "Cargo.toml":
//!
//! ```toml
//! [dependencies]
//! log = { version = "0.4", optional = true }
//! optional-log = "0.1"
//!
//! [feature]
//! log = ["dep:log", "optional-log/log"]
//! ```
//!
//! Then use macros of "optional-log" crate instead of those of the ["log" crate](https://crates.io/crates/log).
//!
//! In this way, once the "log" feature of your crate is enabled by downstream, these macros will be expanded to
//! the corresponding macros of the ["log" crate](https://crates.io/crates/log), otherwise they do nothing.
//!
//! The [`log_enabled!`] macro will always return `false` if the "log" feature is not enabled.

#[macro_export]
#[cfg(feature = "log")]
macro_rules! log {
    ($($t:tt)*) => {
        ::log::log!($($t)*)
    };
}

#[macro_export]
#[cfg(not(feature = "log"))]
macro_rules! log {
    ($($t:tt)*) => {
        ()
    };
}

#[macro_export]
#[cfg(feature = "log")]
macro_rules! trace {
    ($($t:tt)*) => {
        ::log::trace!($($t)*)
    };
}

#[macro_export]
#[cfg(not(feature = "log"))]
macro_rules! trace {
    ($($t:tt)*) => {
        ()
    };
}

#[macro_export]
#[cfg(feature = "log")]
macro_rules! debug {
    ($($t:tt)*) => {
        ::log::debug!($($t)*)
    };
}

#[macro_export]
#[cfg(not(feature = "log"))]
macro_rules! debug {
    ($($t:tt)*) => {
        ()
    };
}

#[macro_export]
#[cfg(feature = "log")]
macro_rules! info {
    ($($t:tt)*) => {
        ::log::info!($($t)*)
    };
}

#[macro_export]
#[cfg(not(feature = "log"))]
macro_rules! info {
    ($($t:tt)*) => {
        ()
    };
}

#[macro_export]
#[cfg(feature = "log")]
macro_rules! warn {
    ($($t:tt)*) => {
        ::log::warn!($($t)*)
    };
}

#[macro_export]
#[cfg(not(feature = "log"))]
macro_rules! warn {
    ($($t:tt)*) => {
        ()
    };
}

#[macro_export]
#[cfg(feature = "log")]
macro_rules! error {
    ($($t:tt)*) => {
        ::log::error!($($t)*)
    };
}

#[macro_export]
#[cfg(not(feature = "log"))]
macro_rules! error {
    ($($t:tt)*) => {
        ()
    };
}

#[macro_export]
#[cfg(feature = "log")]
macro_rules! log_enabled {
    ($($t:tt)*) => {
        ::log::log_enabled!($($t)*)
    };
}

#[macro_export]
#[cfg(not(feature = "log"))]
macro_rules! log_enabled {
    ($($t:tt)*) => {
        false
    };
}
