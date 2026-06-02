# iterative-solvers

Iterative solvers in Rust. When direct methods can't fit in memory.

---

## What's Inside

| Category | Methods |
|---|---|
| **Iterative solvers** | Jacobi, Gauss-Seidel, SOR |
| **Krylov subspace** | Conjugate Gradient (CG), GMRES (restarted), BiCGSTAB |
| **Eigenvalue algorithms** | Power iteration, inverse iteration, QR algorithm, Lanczos |
| **Sparse solvers** | COO/CSR storage, sparse CG, sparse triangular solves |
| **Preconditioners** | Jacobi (diagonal), Incomplete Cholesky IC(0), preconditioned CG |
| **SVD** | Full SVD, truncated SVD, randomized SVD |
| **Least squares** | QR-based, SVD-based, normal equations |
| **Condition estimation** | κ(A) from eigenvalues or singular values |

All algorithms operate on plain `Vec<Vec<f64>>` (dense) or `CsrMatrix`/`CooMatrix` (sparse). No unsafe code, no external solver libraries — just clean iterative numerics.

## Install

```toml
[dependencies]
iterative-solvers = "0.1.0"
```

## Quick Start

### Solve a dense system with Conjugate Gradient

```rust
use iterative_solvers::{cg, ConvergenceCriteria};

let a = vec![
    vec![4.0, 1.0, 0.0],
    vec![1.0, 3.0, 1.0],
    vec![0.0, 1.0, 4.0],
];
let b = vec![5.0, 5.0, 5.0];

let result = cg(&a, &b, &ConvergenceCriteria::new(100, 1e-10));
assert!(result.converged);
println!("x = {:?}", result.x);
```

### Build a sparse system and solve it

```rust
use iterative_solvers::sparse::{CooMatrix, sparse_cg};

let mut coo = CooMatrix::new(3, 3);
coo.add_entry(0, 0, 4.0);
coo.add_entry(0, 1, 1.0);
coo.add_entry(1, 0, 1.0);
coo.add_entry(1, 1, 3.0);
coo.add_entry(1, 2, 1.0);
coo.add_entry(2, 1, 1.0);
coo.add_entry(2, 2, 4.0);

let csr = coo.to_csr();
let b = vec![5.0, 5.0, 5.0];

let result = sparse_cg(&csr, &b, 100, 1e-10);
assert!(result.converged);
```

### Preconditioned sparse CG

```rust
use iterative_solvers::sparse::CooMatrix;
use iterative_solvers::preconditioner::{JacobiPreconditioner, preconditioned_cg};

let csr = /* ... */;
let precond = JacobiPreconditioner::from_csr(&csr);
let result = preconditioned_cg(&csr, &b, |r| precond.apply(r), 100, 1e-10);
```

## License

MIT OR Apache-2.0
