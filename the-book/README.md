# The Rust Programming Language - Exercises

Working through [The Rust Programming Language](https://doc.rust-lang.org/stable/book/).

> **Progress is tracked in the main [README.md](../README.md)**

## Structure

Each chapter gets its own folder. Projects within chapters use `cargo new`:

```text
the-book/
├── ch01/
│   └── hello_cargo/      # cargo project
├── ch02/
│   └── guessing_game/    # cargo project
├── ch03/
│   ├── variables/
│   ├── functions/
│   └── ...
└── ...
```

## Creating New Exercises

```bash
cd the-book/ch07
cargo new my_project
```

Then add it to the workspace in `getrusty/Cargo.toml`:

```toml
members = [
    # ...
    "the-book/ch07/my_project",
]
```
