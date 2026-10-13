//! Handles.

/// The backend of a handle. It is private, so it is not documented.
enum Backend {
    Local,
}

/// An error of a call.
pub enum CallError {
    /// The backend failed.
    Backend(String),
    /// The plugin failed.
    Plugin(String),
}

/// A handle to a [`Backend`].
///
/// A call fails with [`CallError`], for example [`CallError::Plugin`].
pub struct Handle {
    backend: Backend,
}
