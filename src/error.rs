//! # Errors
//!
//! Depending on the specific platform calls, different errors can happen while
//! performing redirections.
//! These errors can happen before or after having executed your code.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error<T> {
    /// Could not set up the redirection to /dev/null.
    /// Your code did not run, and output is working.
    FailedToBeginRedirection,

    /// Could not undo the redirection to /dev/null.
    /// Your code did run, but output is not working anymore.
    FailedToUndoRedirection(T),

    /// Your code panicked.
    /// The output has been successfully restored.
    Panicked,

    /// Your code panicked,
    /// and also we could not restore the output.
    PanickedAndFailedToUndoRedirection,
}
