# Rlex

**Rlex** is a simple and flexible Rust character walker built for writing lexers and parsers. It supports forward and backward stepping, position marking, peeking, and character stashing.

---

## ✨ Features

- Step forward and backward
- Walk with callbacks
- Jump to start, end, or marked positions
- Mark/reset position
- Peek forward and back
- Stash characters and flush

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rlex = { git = "https://github.com/yourname/rlex" }
```

> Replace `yourname` with your GitHub username or use a path if you're working locally.

---

## 🔧 Usage

```rust
use rlex::Rlex;

fn main() {
    let mut rlex = Rlex::new("hello world".to_owned()).unwrap();

    rlex.step_forward(); // move to 'e'
    rlex.mark_current_position();

    rlex.walk_forward_until('w');
    println!("Reached: {}", rlex.current_char); // prints 'w'

    rlex.jump_to_mark(); // go back to 'e'
    println!("Back to: {}", rlex.current_char);
}
```

---

## 🔍 API Overview

### Creation

```rust
let mut rlex = Rlex::new("your string".to_owned()).unwrap();
```

### Movement

- `step_forward()`
- `step_back()`
- `walk_to_end(|&mut Rlex| -> bool)`
- `walk_to_start(|&mut Rlex| -> bool)`
- `walk_forward_until(char) -> bool`
- `walk_back_until(char) -> bool`
- `jump_to_start()`
- `jump_to_end()`
- `jump_to_mark()`

### Marking

- `mark_current_position()`
- `mark_reset()`

### Stashing

- `stash_current_char()`
- `stash_flush() -> Vec<char>`
- `stash_use_mark()`

### Peeking

- `peek_forward(steps: usize) -> Option<char>`
- `peek_back(steps: usize) -> Option<char>`

### State

- `at_start() -> bool`
- `at_end() -> bool`

---

## ✅ Running Tests

Run the built-in tests using:

```bash
cargo test
```

---

## 🪪 License

This project is licensed under the MIT License.
See [LICENSE](./LICENSE) for details.
