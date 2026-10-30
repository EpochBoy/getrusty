# 🦀 Get Rusty

Personal learning repository for Rust.

## Progress

| Ch  | Topic                       | Rustlings             | Book | Ex   |
|-----|-----------------------------|-----------------------|------|------|
| 1-3 | Getting Started & Basics    | 1-22                  | done | done |
| 4   | Understanding Ownership     | 25-29                 | done | done |
| 5   | Using Structs               | 30-32                 | done | done |
| 6   | Enums and Pattern Matching  | 33-35, 46-49          | done | done |
| 7   | Packages & Modules          | 40-42                 | done | done |
| 8   | Common Collections          | 23-24, 36-39, 43-45   | done | done |
| 9   | Error Handling              | 50-54                 | done | done |
| 10  | Generics, Traits, Lifetimes | 55-64                 | done | done |
| 11  | Writing Automated Tests     | 65-67                 | done | done |
| 12  | CLI Project                 | —                     |      |      |
| 13  | Iterators & Closures        | 68-72                 | done | done |
| 14  | More about Cargo            | —                     |      |      |
| 15  | Smart Pointers              | 73-76                 |      |      |
| 16  | Fearless Concurrency        | 77-79                 |      |      |
| 17  | Async and Await             | —                     |      |      |
| 18  | OOP Features                | —                     |      |      |
| 19  | Patterns and Matching       | —                     |      |      |
| 20  | Advanced Features           | 80-91                 |      |      |
| 21  | Final Project               | —                     |      |      |
| —   | Quizzes                     | 92-94                 | —    |      |

---

## Repository Structure

```text
getrusty/
├── the-book/       # Book exercises
├── rustlings/      # Rustlings (run: cd rustlings && rustlings)
├── by-example/     # Reference examples
└── playground/     # Scratch space
```

## Quick Commands

Install [Rust through rustup](https://www.rust-lang.org/tools/install). The checked-in
`rust-toolchain.toml` selects Rust 1.99.1, Clippy and rustfmt for this repository
without changing your global default.

The Rustlings course is already initialized. Install its compatible CLI from the
repository root, then continue the existing exercises:

```bash
cargo install rustlings --version 6.5.0 --locked
cd rustlings && rustlings
```

Run Cargo commands inside the book package you are studying:

```bash
cd the-book/ch13/iterators
cargo run
cargo test
```

Unfinished exercises intentionally contain compiler errors. Keep their progress
and solutions intact; do not reinitialize the course or use an all-workspace
build as a repository health check.

## Documentation

- [Rust Reference Docs](https://doc.rust-lang.org/reference/introduction.html)
- [Rust Stable Standard Library](https://doc.rust-lang.org/stable/std/index.html)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Rust Crate Docs](https://docs.rs/)
- [Rust Cargo Docs](https://doc.rust-lang.org/cargo/index.html)
- [Rust Unstable Library](https://doc.rust-lang.org/beta/unstable-book/)

## Resources

- [The Rust Book](https://doc.rust-lang.org/stable/book/)
- [Rust Brown University Book](https://rust-book.cs.brown.edu/)
- [Rustonomicon](https://doc.rust-lang.org/stable/nomicon/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Cheat Sheet](https://cheats.rs/)

---

Started: January 2026
