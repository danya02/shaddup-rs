//! This module contains dummy implementations of the platform-specific functions
//! when the `allow_unsupported` or `no_op` feature is used.

pub fn run_quietly_with_opts<TOut>(
    action: impl FnOnce() -> TOut,
    _opts: crate::Opts,
) -> Result<TOut, crate::Error<TOut>> {
    Ok(action())
}
