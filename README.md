## The Rust Programming Language

__________________________________

### Exercises from [The Book](https://doc.rust-lang.org/book/)

#### Getting started
```bash
rustc --version # check version
rustup update
rustup doc # open local docs
```

#### Cargo
```bash
cargo --version
cargo new my_project
rustc main.rs # create an executable in the current directory
cargo build  # create an executable in target/debug/main
carbo build --release # build with optimizations
cargo run # compile and run the executable
cargo check # compile without producing the executable
```