/// Options for how exactly to perform the silencing.
#[derive(Copy, Clone, Debug, Default)]
pub struct Opts {
    /// Descriptors to be silenced.
    pub descriptors: Descriptors,
}

/// Combination of file descriptors to be silenced.
/// By default, both stdout and stderr are silenced.
#[derive(Copy, Clone, Debug, Default)]
pub enum Descriptors {
    /// Only stdout is silenced, stderr is allowed.
    Stdout,
    /// Only stderr is silenced, stdout is allowed.
    Stderr,
    /// Both stdout and stderr are silenced.
    /// This is the default.
    #[default]
    StdoutStderr,
}

impl Descriptors {
    /// Returns a tuple of (stdout, stderr),
    /// true if the corresponding descriptor is to be silenced.
    pub fn to_bools(self) -> (bool, bool) {
        match self {
            Descriptors::Stdout => (true, false),
            Descriptors::Stderr => (false, true),
            Descriptors::StdoutStderr => (true, true),
        }
    }
}
