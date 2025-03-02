//! # `shaddup`!
//!
//! This library prevents your program from printing to stdout and stderr
//! by using platform-specific utilities to redirect them to /dev/null (or equivalent).
//!
//! By default, this will cause a compile-error if the platform is not supported,
//! because we don't know how to perform the redirection.
//! If you'd like the library to be a no-op in these cases,
//! use the `allow_unsupported` feature.
//!
//! If you want to make this library be a no-op even if the platform is supported
//! (for example, for debugging),
//! add the `no_op` feature:
//! this turns the entire library into a no-op.
//!
//! ## Usage
//!
//! ```rust
//! use shaddup::run_quietly;
//!
//! let result = run_quietly(|| {
//!     println!("This will not be printed");
//!     eprintln!("This will also not be printed");
//!     123
//! });
//! assert_eq!(result.unwrap(), 123);
//! ```
//!
//! ## Features
//!
//! - `no_op`: turns the entire library into a no-op.
//! - `allow_unsupported`: turns the library into a no-op if the platform is supported (otherwise, the library will cause a compile-error).
//!
//! ## See also
//!
//! [`gag`](https://docs.rs/gag/latest/gag/) is another library that implements this functionality.
//! It uses a different API, based on guards
//! rather than closures.
//! It supports Unix and Windows.

pub mod error;
pub mod opts;
pub use error::Error;
pub use opts::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "no_op")] {
        mod unsupported;
        use unsupported as platform;
    } else if #[cfg(unix)] {
        mod unix;
        use unix as platform;
    } else if #[cfg(feature = "allow_unsupported")] {
        mod unsupported;
        use unsupported as platform;
    } else {
        compile_error!("Currently, only the `unix` target is supported");
    }
}

/// Run the given function, preventing it from printing to stdout and stderr.
///
/// ```
/// use shaddup::run_quietly;
///
/// let result = run_quietly(|| {
///     println!("This will not be printed");
///     123
/// });
/// assert_eq!(result.unwrap(), 123);
/// ```
pub fn run_quietly<TOut>(
    action: impl FnOnce() -> TOut + std::panic::UnwindSafe,
) -> Result<TOut, error::Error<TOut>> {
    platform::run_quietly_with_opts(action, Opts::default())
}

/// Run the given function, preventing it from printing to descriptors.
/// You can customize which ones.
///
/// ```
/// use shaddup::run_quietly_with_opts;
///
/// let result = run_quietly_with_opts(|| {
///     println!("This will not be printed,");
///     eprintln!("but this will be!");
///     123
/// }, shaddup::opts::Opts {
///     descriptors: shaddup::opts::Descriptors::Stdout,
/// });
/// assert_eq!(result.unwrap(), 123);
/// ```
pub fn run_quietly_with_opts<TOut>(
    action: impl FnOnce() -> TOut + std::panic::UnwindSafe,
    opts: Opts,
) -> Result<TOut, error::Error<TOut>> {
    platform::run_quietly_with_opts(action, opts)
}
