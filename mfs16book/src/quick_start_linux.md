# Quick Start - Linux

### 1. Preparations

Ensure that you have [rustup](https://rustup.rs/), [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git), [CMake](https://cmake.org/), and a C linker. A commonly-used C linker is included in the [GCC toolchain](https://gcc.gnu.org/).

### 2. Clone the Git repository

Clone the [Git repo](https://github.com/maxgmr/mfs16) to a convenient place, then enter the new `mfs16` directory.

```bash
git clone https://github.com/maxgmr/mfs16 && cd mfs16
```

## 3. Build the program

Generally speaking, the program needs to be built using the `release` profile in order to run at the proper clock speed.

```bash
cargo build --release
```

## 4. Run!

To run the hello world program:

```bash
target/release/mfs16desktop programs/hello_world
```

Why not try typing some text?

```bash
target/release/mfs16desktop programs/scribe
```
