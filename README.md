# Rlex

**Rlex** is a lightweight lexer utility for traversing, peeking, and extracting parts of a UTF-8 string. It operates on a `Vec<char>` and retains the original string to allow for accurate byte-range slicing. It is ideal for building scanners, parsers, or any tool that needs detailed and controlled inspection of characters in a string.

## Installation

Install via `cargo`

```sh
cargo add rlex
```

---

## Features

### Creating a Lexer

First, you need an enum to represent the state of your lexer and a token type:
```rust
#[derive(Debug, PartialEq, Eq)]
enum MyState {
    Init,
    Open,
    Closed,
}

#[derive(Debug, PartialEq, Eq)]
enum MyToken {
    Tok1,
    Tok2,
    Tok3,
}
```

Then use the enums to create a new lexer:
```rust
let r: Rlex<MyState, MyToken> = Rlex::new("hello", MyState::Init);
```

### Using Default State / Default Token
If you don't care to collect tokens or track state, use `DefaultState` and `DefaultToken` upon initalization.

```rust
let r: Rlex<DefaultState, DefaultToken> = Rlex::new("hello", DefaultState::Default);
```

### State Handling

```rust
r.state();              // Get a reference to the current state
r.state_set(MyState::Open);  // Set a new state
```

### Position Utilities

```rust
r.pos();            // Current position
r.mark();           // Mark current position
r.goto_start();     // Go to start of input
r.goto_end();       // Go to end of input
r.goto_pos(2);      // Go to a specific position
r.goto_mark();      // Go back to marked position
```

### Navigation

```rust
r.next();           // Move forward one
r.next_by(3);       // Move forward by n
r.prev();           // Move backward one
r.prev_by(2);       // Move backward by n
r.next_until('x');  // Advance until char
r.prev_until('x');  // Rewind until char
```

### Peeking

```rust
r.peek();            // Look at next char
r.peek_by(2);        // Look ahead by n
r.peek_back();       // Look behind one
r.peek_back_by(3);   // Look back by n
```

### Char Checks

```rust
r.char();             // Get current char
r.next_is('x');       // Check if next char is x
r.next_by_is('x', 2); // Check if x is n chars ahead
r.prev_is('x');       // Check if previous char is x
r.prev_by_is('x', 3); // Check if x is n chars behind
```

### Position Queries

```rust
r.at_start();     // At beginning?
r.at_end();       // At end?
r.at_mark();      // At previously marked spot?
```

### String Extraction

```rust
r.src() // Read the Lexer source
r.toks() // Get a reference to the collected tokens
r.str_from_mark();  // Slice from mark to current
r.str_from_start(); // Slice from start to current
r.str_from_end();   // Slice from current to end
r.str_from_collection(); // Convert the collection into a slice
r.str_from_rng(0, 2); // Index-based slice from source 
```

### Quote Detection

```rust
r.is_in_quote(); // Returns true if current position is inside a quote block
```

### Collecting Characters

```rust
r.collect(); // Collect the character at the current position
r.collect_pop(); // Get the newest character added to the collection
r.collect_push('a'); // Push a character of your choice into the collection
r.collect_clear(); // Clears the current collection
```

### Working With Tokens

```rust
r.token_push(MyToken::Tok1); // Push a token into the collection
r.token_pop(); // Remove and obtain the last token in the collection
r.token_prev(); // Peek at the last token in the collection
r.token_consume(); // Consumes the lexer and outputs the collected tokens
```

---

## License

This project is licensed under the MIT License.