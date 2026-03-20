# RPL
[The Rust Programming Language book (2024)](https://doc.rust-lang.org/stable/book/) - exercises


Chapter-based run exercises  

cargo run -p ch01-getting-started

or 

cd ch01-getting-started
cargo run


Rustlings (interactive)
cd /rustlings
rustlings


Execute by exercise in chapter
ch01-getting-started/
└── src/bin/
    ├── ex01_hello.rs
    ├── ex02_variables.rs
    └── ex03_mutability.rs

cargo run -p ch01-getting-started --bin ex01_hello


Useful commands

Auto rebuild
cargo watch -x run

Lint
cargo clippy

Test (Rustlings uses tests internally)
cargo nextest run