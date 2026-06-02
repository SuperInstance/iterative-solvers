//! # iterative-solvers
//!
//! Iterative linear algebra solvers for large-scale computations.
//! CG, GMRES, BiCGSTAB, Krylov subspace methods, eigenvalue algorithms,
//! sparse solvers, preconditioners, SVD, and least squares.

pub mod iterative;
pub mod krylov;
pub mod eigen;
pub mod sparse;
pub mod preconditioner;
pub mod svd;
pub mod least_squares;
pub mod condition;
pub mod types;

pub use types::*;
