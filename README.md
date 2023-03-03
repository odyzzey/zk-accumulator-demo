# zk-transaction-accumulator demo 

In an environment where external, but approved, public-facing methods do not have access to the state during execution (read: write only), how do we safely transition the contract's state with only the outputs of these methods?

Proposed Solution: Require that contracts implement accumulator logic to add output substates.

We can build state transition logic into an addition operation between the state and public method output types, meaning that host method outputs propose increments to the contract's state that the state object's addition operand can choose to accept or reject. These increments, or substates, can be added together, or accumulated to reduce the number of operations on the contract state itself.

For this to work it should be true that types $P$ and $V$ are such that if $P_t$ represents the contract's state at time $t$, and $V =\{ V_1, V_2, .., V_n\}$ is a set of $n$ valid increments to $P$, we can say that $P_t + V_1 + V_2 + .. + V_n = P_t + \sum\limits_{i=1}^n (V_n)$

This repo is a simulation of what a system like this may look like on a live chain.

## Quick Start

First, make sure [rustup](https://rustup.rs) is installed. This project uses a [nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) version of [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html). The [`rust-toolchain`](rust-toolchain) file will be used by `cargo` to automatically install the correct version.

To build all methods and execute the method within the zkVM, run the following command:

```
cargo run
```

## Expected output
```
Transaction 1:
         Receipt: [45, 45, 10]
         ContractPoint { x: 45, y: 45, total: 10 }
Transaction 2:
         Receipt: [90, 90, 10]
         ContractPoint { x: 135, y: 135, total: 20 }
```
