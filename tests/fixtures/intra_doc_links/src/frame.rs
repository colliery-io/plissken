//! Frames on the wire.

/// A frame. Its variant `Error` shares a name with `std::io::Error`.
pub enum Frame {
    /// Data.
    Data(Vec<u8>),
    /// An error.
    Error(String),
}
