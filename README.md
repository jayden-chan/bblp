CSC-445 Linear Program Solver
=============================

`bblp` is a simple linear program solver. It was originally built for an assignment in
CSC-445: Operations Research: Linear Programming at the University of Victoria.

Installation
------------

```
git clone git@github.com:jayden-chan/bblp.git
cd bblp
cargo build --release
```

Installation for linux.csc.uvic.ca
----------------------------------

Execute `make`.

This will install the Rust toolchain to the current directory **only**.  After
installation it will compile the program. The resulting binary is called `bblp` and will
be present in the same directory as the Makefile. This should take about 2 minutes.

If it is necessary to re-compile the program, just run `make build-local`.

Feature Overview
----------------

| Category                        | Implementation                   |
| ------------------------------- | -------------------------------- |
| Solve method                    | Linear Algebraic Revised Simplex |
| Pivot strategy                  | Largest coefficient              |
| Cycle-avoidance                 | Perturbation                     |
| Initially-infeasible resolution | Two-phase primal-dual            |

### Solve Method
The program implements the Revised Simplex Method. It does not compute any inverse
matrices and instead uses LU-decomposition for solving the linear systems described on
slides 103 and 104 of lecture 14. The relevant portions of code are commented to point
this out.

### Pivot Strategy
The program uses the largest coefficient pivot selection rule for all pivots. The routine
for computing this is called `select_entering` and can be found in the `src/utils.rs`
file.

### Cycle-avoidance
The program uses the perturbation method for avoiding cycles. The implementation is based
on the following document from Carleton University:
https://people.math.carleton.ca/~kcheung/math/notes/MATH5801/1/01_perturb.html.

The routine is called `perturb` and can be found in `src/utils.rs`.

To prove that the implementation of perturbation does indeed prevent cycling, you can run
the cycle-testing LP which I have included:
```
./bblp < ./lp_tests/input/cycle.txt
./bblp --no-perturb < ./lp_tests/input/cycle.txt
```
You will see that the program computes the correct optimal solution with perturbation
enabled but cycles without it. This LP is the same example of largest-coefficient cycling
that was given on slide 49 of lecture 8.

### Initially-infeasible resolution
The program uses a two-phase primal-dual method for solving initially infeasible
problems. The dual simplex routine can be found in `src/solve/dual.rs` and the logic for
running the dual simplex auxiliary problem can be found in `src/main.rs`
