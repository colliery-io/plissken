//! Const generics.

use std::ops::{Add, Index, IndexMut};

/// Fixed-size buffer using const generics.
#[derive(Debug, Clone, Copy)]
pub struct Buffer<const N: usize> {
    data: [u8; N],
    len: usize,
}

impl<const N: usize> Buffer<N> {
    /// Create a new empty buffer.
    pub const fn new() -> Self {
        Self {
            data: [0; N],
            len: 0,
        }
    }

    /// Get the capacity.
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Get the current length.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Check if empty.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Push a byte, returns false if full.
    pub fn push(&mut self, byte: u8) -> bool {
        if self.len < N {
            self.data[self.len] = byte;
            self.len += 1;
            true
        } else {
            false
        }
    }

    /// Get as slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.len]
    }
}

impl<const N: usize> Default for Buffer<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Matrix with const dimensions.
#[derive(Debug, Clone, Copy)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    /// Create a matrix filled with default values.
    pub fn new() -> Self {
        Self {
            data: [[T::default(); COLS]; ROWS],
        }
    }

    /// Get the dimensions.
    pub const fn dimensions(&self) -> (usize, usize) {
        (ROWS, COLS)
    }

    /// Get a reference to an element.
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row).and_then(|r| r.get(col))
    }

    /// Get a mutable reference to an element.
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.data.get_mut(row).and_then(|r| r.get_mut(col))
    }
}

impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Default for Matrix<T, ROWS, COLS> {
    fn default() -> Self {
        Self::new()
    }
}

/// Matrix multiplication with const generics.
///
/// Multiplies an MxN matrix with an NxP matrix to produce an MxP matrix.
pub fn matmul<T, const M: usize, const N: usize, const P: usize>(
    a: &Matrix<T, M, N>,
    b: &Matrix<T, N, P>,
) -> Matrix<T, M, P>
where
    T: Default + Copy + Add<Output = T> + std::ops::Mul<Output = T>,
{
    let mut result = Matrix::new();
    for i in 0..M {
        for j in 0..P {
            let mut sum = T::default();
            for k in 0..N {
                sum = sum + a.data[i][k] * b.data[k][j];
            }
            result.data[i][j] = sum;
        }
    }
    result
}

/// Const generic with expression.
pub struct Padded<T, const N: usize, const ALIGN: usize>
where
    [(); ALIGN - (N % ALIGN)]:,  // Compile-time constraint
{
    data: [T; N],
    _padding: [u8; ALIGN - (N % ALIGN)],
}

/// Multiple const parameters with constraints.
pub fn transfer<const SRC: usize, const DST: usize>(
    src: &Buffer<SRC>,
) -> Option<Buffer<DST>>
where
    [(); DST - SRC]:,  // DST must be >= SRC (compile-time check)
{
    if src.len() > DST {
        return None;
    }
    let mut dst = Buffer::<DST>::new();
    for &byte in src.as_slice() {
        dst.push(byte);
    }
    Some(dst)
}

/// Const generics with default values.
pub struct RingBuffer<T, const N: usize = 64> {
    data: [Option<T>; N],
    head: usize,
    tail: usize,
}

impl<T, const N: usize> RingBuffer<T, N> {
    /// Create a new ring buffer.
    pub fn new() -> Self
    where
        T: Copy,
    {
        Self {
            data: [None; N],
            head: 0,
            tail: 0,
        }
    }
}
