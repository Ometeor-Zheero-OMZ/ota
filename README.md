<h1>The Ota Language</h1>

---

<p>
    This language is designed with a focus on low-level memory control and safety, making it suitable for systems programming tasks such as developing kernel drivers and operating systems. It combines the robustness of Rust with the flexible memory manipulation capabilities of C.
</p>

<h2>Directories</h2>

---

```
rust/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── compiler/
│   │   ├── ast/
|   |   |   ├── common.rs
|   |   |   ├── declarations.rs
|   |   |   ├── expressions.rs
|   |   |   ├── memory.rs
|   |   |   ├── pattern.rs
|   |   |   ├── precedence.rs
|   |   |   ├── statements.rs
|   |   |   ├── types.rs
|   |   |   └── mod.rs
│   │   ├── core/
|   |   |   ├── mod.rs (preparing...)
│   │   ├── parsing/
|   |   |   ├── lexer/
|   |   |   |   ├── token.rs
|   |   |   |   └── mod.rs
|   |   |   ├── parser/
|   |   |   |   ├── expression/
|   |   |   |   |   ├── function.rs
|   |   |   |   |   ├── object.rs
|   |   |   |   |   ├── operator.rs
|   |   |   |   |   ├── primitive.rs
|   |   |   |   |   └── mod.rs
|   |   |   |   ├── statement/
|   |   |   |   |   ├── block.rs
|   |   |   |   |   ├── for.rs
|   |   |   |   |   ├── if.rs
|   |   |   |   |   ├── switch.rs
|   |   |   |   |   ├── variables.rs
|   |   |   |   |   └── mod.rs
|   |   |   |   └── mod.rs
|   |   |   └── mod.rs
│   │   └── mod.rs
├── target/
├── Cargo.lock
└── Cargo.toml
```

<h2>Status</h2>

---

> ⚠️ This project is still under active development and not yet ready for production use.

![status-badge](https://img.shields.io/badge/status-in--development-yellow?style=flat-square)
