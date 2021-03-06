#![allow(missing_docs)]

//! Gates and Gate Generation
//!
//! Matrices are in row major format, and
//! all gates use the `num_complex::Complex<f32>;`
//! datatype.

use num_complex::Complex32;
use std::fmt;
use std::f32::consts::{FRAC_1_SQRT_2, E};

/// Representation of a gate
///
/// ```
///# extern crate qcgpu;
///# extern crate num_complex;
///# use qcgpu::Gate;
///# use num_complex::Complex32;
/// Gate {
///    a: Complex32::new(0.0, 0.0), b: Complex32::new(1.0, 0.0),
///    c: Complex32::new(1.0, 0.0), d: Complex32::new(0.0, 0.0)
/// };
///
///
#[derive(Debug, Clone, Copy)]
pub struct Gate {
    pub a: Complex32,
    pub b: Complex32,
    pub c: Complex32,
    pub d: Complex32,
}

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[[{}, {}], [{}, {}]]", self.a, self.b, self.c, self.d)
    }
}

/// Identity gate
///
/// [1, 0]
///
/// [0, 1]
#[inline]
pub fn id() -> Gate {
    Gate {
        a: Complex32::new(1.0, 0.0),
        b: Complex32::new(0.0, 0.0),
        c: Complex32::new(0.0, 0.0),
        d: Complex32::new(1.0, 0.0),
    }
}

/// Hadamard Gate
///
/// [0.70710678118, 0.70710678118]
///
/// [0.70710678118, -0.70710678118]
#[inline]
pub fn h() -> Gate {
    Gate {
        a: Complex32::new(FRAC_1_SQRT_2, 0.0),
        b: Complex32::new(FRAC_1_SQRT_2, 0.0),
        c: Complex32::new(FRAC_1_SQRT_2, 0.0),
        d: Complex32::new(-FRAC_1_SQRT_2, 0.0),
    }
}

/// Negative Hadamard Gate
///
/// [-0.70710678118, -0.70710678118]
///
/// [-0.70710678118, 0.70710678118]
#[inline]
pub fn negh() -> Gate {
    Gate {
        a: Complex32::new(-FRAC_1_SQRT_2, 0.0),
        b: Complex32::new(-FRAC_1_SQRT_2, 0.0),
        c: Complex32::new(-FRAC_1_SQRT_2, 0.0),
        d: Complex32::new(FRAC_1_SQRT_2, 0.0),
    }
}

/// Pauli X / NOT Gate
///
/// [0, 1]
///
/// [1, 0]
#[inline]
pub fn x() -> Gate {
    Gate {
        a: Complex32::new(0.0, 0.0),
        b: Complex32::new(1.0, 0.0),
        c: Complex32::new(1.0, 0.0),
        d: Complex32::new(0.0, 0.0),
    }
}

/// Pauli Y Gate
///
/// [0, -i]
///
/// [i, 0]
#[inline]
pub fn y() -> Gate {
    Gate {
        a: Complex32::new(0.0, 0.0),
        b: Complex32::new(0.0, -1.0),
        c: Complex32::new(0.0, 1.0),
        d: Complex32::new(0.0, 0.0),
    }
}

/// Pauli Z Gate
///
/// [1, 0]
///
/// [0, -1]
#[inline]
pub fn z() -> Gate {
    Gate {
        a: Complex32::new(1.0, 0.0),
        b: Complex32::new(0.0, 0.0),
        c: Complex32::new(0.0, 0.0),
        d: Complex32::new(-1.0, 0.0),
    }
}

/// S / Phase Gate
///
/// [1, 0]
///
/// [0, i]
#[inline]
pub fn s() -> Gate {
    Gate {
        a: Complex32::new(1.0, 0.0),
        b: Complex32::new(0.0, 0.0),
        c: Complex32::new(0.0, 0.0),
        d: Complex32::new(0.0, 1.0),
    }
}

/// T Gate
///
/// [1, 0]
///
/// [0,(1/sqrt(2)) * 1 + 1i]
#[inline]
pub fn t() -> Gate {
    Gate {
        a: Complex32::new(1.0, 0.0),
        b: Complex32::new(0.0, 0.0),
        c: Complex32::new(0.0, 0.0),
        d: Complex32::new(
            0.707_106_781_186_547_524_400_844_362_104_849_039_3,
            0.707_106_781_186_547_524_400_844_362_104_849_039_3,
        ),
    }
}

/// The Phase Shift / R gate.
/// Note that this function has an argument of `angle`, which corresponds to the
/// rotation angle of the gate.
///
/// [1 ,0]
///
/// [0, e^i*angle]
pub fn r(angle: f32) -> Gate {
    Gate {
        a: Complex32::new(1.0, 0.0),
        b: Complex32::new(0.0, 0.0),
        c: Complex32::new(0.0, 0.0),
        d: Complex32::new(E, 0.0).powc(Complex32::new(0.0, angle)),
    }
}
