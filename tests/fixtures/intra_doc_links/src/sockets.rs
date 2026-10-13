//! Sockets.

/// Connect to a socket.
///
/// Fails with [`std::io::Error`] or [`core::fmt::Error`] (other crates),
/// or with a bare [`Error`] (only a variant has this name).
/// Sends a [`Frame::Error`] or a [`crate::frame::Frame`].
pub fn connect() {}
