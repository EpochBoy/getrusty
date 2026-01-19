# Learning Path: TypeScript → Rust

A high-level curriculum. Track progress in [README.md](./README.md).

---

## Timeline

### Week 1-2: Foundations

| Material               | Topics                                           |
|------------------------|--------------------------------------------------|
| Rust Book Ch 1-4       | Setup, basics, **ownership & borrowing** ⚠️      |
| Rust Book Ch 5-6       | Structs, enums, pattern matching                 |
| Rust Book Ch 7-10      | Modules, collections, error handling, generics   |
| Rustlings 1-61         | Exercises reinforcing all above concepts         |

### Week 3: Advanced Concepts

| Material               | Topics                                           |
|------------------------|--------------------------------------------------|
| Rust Book Ch 11-13     | Testing, CLI project, iterators & closures       |
| Rust Book Ch 14-16     | Cargo, smart pointers, concurrency               |
| Rustlings 62-94        | Lifetimes, threads, macros, conversions          |

### Week 4: Build Something

- Build a CLI tool (file processor, API client, etc.)
- Apply ownership, error handling, and iterators
- Publish to crates.io (optional)

### Week 5+: Async & Web

- Learn Tokio async runtime
- Build a web API with Axum
- Database integration with SQLx

---

## Quick Reference: TS → Rust

| TypeScript       | Rust                                |
|------------------|-------------------------------------|
| `number`         | `i32`, `i64`, `f64`                 |
| `string`         | `String` (owned), `&str` (borrowed) |
| `boolean`        | `bool`                              |
| `T[]`            | `Vec<T>`                            |
| `T \| null`      | `Option<T>`                         |
| `Record<K, V>`   | `HashMap<K, V>`                     |
| `interface`      | `struct` + `trait`                  |
| `class`          | `struct` + `impl`                   |
| `try/catch`      | `Result<T, E>` + `?`                |
| `async/await`    | `async fn` + `.await`               |
| `Promise.all`    | `tokio::join!`                      |

---

## Async & Frameworks

After completing the book and rustlings, explore:

| Category      | Library    | Link                                    |
|---------------|------------|-----------------------------------------|
| Async Runtime | Tokio      | <https://tokio.rs/tokio/tutorial>       |
| Web Framework | Axum       | <https://docs.rs/axum/latest/axum/>     |
| Database      | SQLx       | <https://docs.rs/sqlx/latest/sqlx/>     |
| Serialization | Serde      | <https://serde.rs/>                     |

---

## Resources

### Essential Reading

1. [The Rust Book](https://doc.rust-lang.org/book/)
2. [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
3. [Rustlings](https://github.com/rust-lang/rustlings)

### Framework Docs

1. [Axum](https://docs.rs/axum/latest/axum/)
2. [Tokio](https://tokio.rs/tokio/tutorial)
3. [SQLx](https://docs.rs/sqlx/latest/sqlx/)
4. [Serde](https://serde.rs/)

### Video Courses

1. [Let's Get Rusty](https://www.youtube.com/@letsgetrusty) — Great for beginners
2. [Jon Gjengset](https://www.youtube.com/@jonhoo) — Advanced Rust concepts
3. [Rust for JavaScript Developers](https://rustforjs.dev/)

### Cheat Sheets

1. [Rust Cheat Sheet](https://cheats.rs/)
