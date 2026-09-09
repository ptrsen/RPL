# RPL
[The Rust Programming Language book (2024)](https://doc.rust-lang.org/stable/book/) - exercises


Chapter-based run exercises:  
cargo run -p ch01-getting-started

or 

cd ch01-getting-started
cargo run 
cargo run src/main.rs

or (direclty execute binary)
cargo build
./target/debug/ch01-getting-started




Execute by exercise in chapter:
ch01-getting-started/
└── src/bin/
    ├── ex01_hello.rs
    ├── ex02_variables.rs
    └── ex03_mutability.rs


cargo run -p <path> --bin <name>
cargo run -p ch01-getting-started --bin ex01_hello


Useful commands:
cargo 
cargo chek
cargo build
cargo build --release
cargo run 
cargo update


Internal docs:
cargo doc --open 

Create package:
cargo new new_package
cd new_package
cargo run

Create package and init also git:
cargo new --vcs=git new_package
cd new_package
cargo run


Auto rebuild:
cargo watch -x run

Lint:
cargo clippy



Rustlings (interactive):
cd /rustlings
rustlings


Test (Rustlings uses tests internally):
cargo nextest run