# RISC-V (RV32I) Emulator
A RISC-V emulator, specifically the RV32I base integer instruction set.
Made entirely to learn how to write an emulator and gain experience with Rust.

At this moment, this emulator prints the instructions being executed, and when it reaches an `unimp` instruction or the program counter overflows the emulation stops and the cpu's registers are printed. Only the `exit` syscall is implemented.

Tests are also included in this repo. The tests are built from [riscv-tests](https://github.com/riscv/riscv-tests).
## Build
You need `rust` and `cargo` installed in order to build the emulator.
```bash
$ git clone https://github.com/kiraleos/riscv32i-emulator.git
$ cd riscv32i-emulator
$ cargo build --release
$ mv ./target/release/riscv-emulator ../
```
## Usage
```bash
$ riscv-emulator <file>
```
or, to run every test in the tests directory
```bash
$ riscv-emulator
```

### Example usage
```bash
$ riscv-emulator ./tests/add
```
