# Rlex

## Outline
1. **Introduction**
2. **Installation**
3. **Usage**
   - Constructor
   - Navigation Methods
   - Peek Methods
   - Mark and Jump Methods
   - Stash Methods
   - State and Quotes Utilities
4. **Caveats / Gotchas**

---

## Introduction
**Rlex** is a lightweight Rust lexer utility for traversing and analyzing string inputs character-by-character. It supports controlled navigation (forwards and backwards), marking and jumping to positions, stashing characters, and even checking for quoted sections in text.

## Installation
Include `rlex.rs` in your project and ensure it's compiled with the rest of your application. You can also structure it as a module in a larger Rust crate.

```rust
cargo add rlex
```

## Usage

### Constructor
```rust
let mut rlex = Rlex::new("your string here".to_owned()).unwrap();
```
Creates a new Rlex instance. Panics if the string is empty.

### Navigation Methods
- `step_forward()` / `step_back()`: Moves one character forward or backward.
- `walk_to_end(F)` / `walk_to_start(F)`: Traverse based on a closure returning a bool.
- `walk_forward_until(char)` / `walk_back_until(char)`: Stop walking when a specific character is found.
- `jump_to_end()` / `jump_to_start()`: Jump to ends of the string.
- `jump_to_mark()`: Jump to a previously marked position.

### Peek Methods
- `peek_forward(steps)` / `peek_back(steps)`: Look ahead or behind without moving the current position.

### Mark and Jump Methods
- `mark_current_position()`: Save the current position.
- `mark_reset()`: Reset mark to the start.

### Stash Methods
- `stash_current_char()`: Save the current character to stash.
- `stash_flush()`: Clear and return the stash.
- `stash_use_mark()`: Fill stash with characters between current and marked positions.

### State and Quotes Utilities
- `at_start()` / `at_end()`: Checks for boundaries.
- `is_in_quotes()`: Detects if the current character is inside a quoted section (single or double quotes).

## Caveats / Gotchas
- `new()` returns an error if initialized with an empty string.
- `peek_forward()` and `peek_back()` return `None` if the peek would go out of bounds.
- `is_in_quotes()` involves jumping to start, traversing, and returning to the original position. Use with caution in performance-sensitive code.
- `stash_use_mark()` prints internal debug info using `println!`, which may be undesired in production. Remove or comment out if necessary.
