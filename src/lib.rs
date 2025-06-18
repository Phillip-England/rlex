// Represents a lexer that can traverse, peek, and stash characters from a string source
#[derive(Debug)]
pub struct Rlex {
  source: String,                 // Original input string
  chars: Vec<char>,              // Vector of characters from the source
  current_position: usize,       // Current index in the chars vector
  max_position: usize,           // Maximum valid index in the chars vector
  pub current_char: char,        // Currently selected character
  marked_position: usize,        // Saved position for future reference
  stash: Vec<char>,              // Temporary storage for selected characters
  pub state: String,             // Arbitrary state information
}

impl Rlex {

  // Constructs a new Rlex instance from a given string, ensuring it's not empty
  pub fn new(source: String) -> Result<Rlex, String> {
    let chars: Vec<char> = source.chars().collect();
    if chars.is_empty() {
      return Err("Rlex must contain at least one character".to_owned());
    }
    let length = chars.len();
    Ok(Rlex {
      source,
      chars: chars.clone(),
      current_position: 0,
      max_position: length - 1,
      current_char: chars[0],
      marked_position: 0,
      stash: vec!(),
      state: "".to_owned(),
    })
  }

  // Advances to the next character if not at the end
  pub fn step_forward(&mut self) {
    if self.current_position == self.max_position {
      return;
    }
    self.current_position += 1;
    self.current_char = self.chars[self.current_position];
  }

  // Moves one character backward if not at the start
  pub fn step_back(&mut self) {
    if self.current_position == 0 {
      return;
    }
    self.current_position -= 1;
    self.current_char = self.chars[self.current_position];
  }

  // Continues moving forward as long as the provided function returns true
  pub fn walk_to_end<F>(&mut self, mut f: F)
  where
    F: FnMut(&mut Rlex) -> bool,
  {
    loop {
      let should_continue = f(self);
      if should_continue == false {
        break;
      }
      if self.current_position == self.max_position {
        break;
      }
      self.step_forward();
    }
  }

  // Continues moving backward as long as the provided function returns true
  pub fn walk_to_start<F>(&mut self, mut f: F) 
  where
    F: FnMut(&mut Rlex) -> bool,
  {
    loop {
      let should_continue = f(self);
      if should_continue == false {
        break;
      }
      if self.current_position == 0 {
        break;
      }
      self.step_back();
    }
  }

  // Moves forward until the specified character is found
  pub fn walk_forward_until(&mut self, ch: char) -> bool {
    let mut found = false;
    self.step_forward();
    self.walk_to_end(|me| {
      if me.current_char == ch {
        found = true;
        return false;
      }
      return true;
    });
    return found
  }

  // Moves backward until the specified character is found
  pub fn walk_back_until(&mut self, ch: char) -> bool {
    let mut found = false;
    self.step_back();
    self.walk_to_start(|me| {
      if me.current_char == ch {
        found = true;
        return false;
      }
      return true;
    });
    return found;
  }

  // Instantly moves to the end of the input
  pub fn jump_to_end(&mut self) {
    if self.at_end() {
      return;
    }
    self.current_position = self.max_position;
    self.current_char = self.chars[self.current_position];
  }

  // Instantly moves to the start of the input
  pub fn jump_to_start(&mut self) {
    if self.at_start() {
      return;
    }
    self.current_position = 0;
    self.current_char = self.chars[self.current_position];
  }

  // Jumps to the previously marked position
  pub fn jump_to_mark(&mut self) {
    self.current_position = self.marked_position;
    self.current_char = self.chars[self.current_position];
  }

  // Marks the current position for later use
  pub fn mark_current_position(&mut self) {
    self.marked_position = self.current_position;
  }

  // Resets the mark to the start
  pub fn mark_reset(&mut self) {
    self.marked_position = 0;
  }

  // Adds the current character to the stash
  pub fn stash_current_char(&mut self) {
    self.stash.push(self.current_char);
  }

  // Clears the stash and returns its contents
  pub fn stash_flush(&mut self) -> Vec<char> {
    let flushed: Vec<char> = self.stash.clone();
    self.stash = vec!();
    return flushed;
  }

  // Fills the stash with characters between the current position and the marked position
  pub fn stash_use_mark(&mut self) {
    let saved = self.current_position;
    if self.current_position > self.marked_position {
      self.jump_to_mark();
      println!("{:?}", self);
      self.walk_to_end(|me| {
        me.stash_current_char();
        if me.current_position == saved {
          return false;
        } 
        return true;
      });
      return;
    }
    self.walk_to_end(|me| {
      me.stash_current_char();
      if me.current_position == me.marked_position {
        return false;
      }
      return true;
    });
  }

  // Peeks forward a number of characters without changing position
  pub fn peek_forward(&self, steps: usize) -> Option<char> {
    if self.current_position < self.max_position {
      let peek_position = self.current_position + steps;
      if peek_position > self.max_position {
        return None;
      }
      return Some(self.chars[steps])
    } else {
      return None
    }
  }

  // Peeks backward a number of characters without changing position
  pub fn peek_back(&self, steps: usize) -> Option<char> {
      if self.current_position > 0 {
        let target_position = self.current_position.checked_sub(steps);
        if target_position.is_none() {
          return None;
        }
        return Some(self.chars[self.current_position - steps])
      } else {
        return None
      }
  }

  // Checks if the current position is at the end
  pub fn at_end(&self) -> bool {
    self.current_position == self.max_position
  }

  // Checks if the current position is at the start
  pub fn at_start(&self) -> bool {
    self.current_position == 0
  }

  // Determines whether the current position is inside quotes (single or double)
  pub fn is_in_quotes(&mut self) -> bool {
    let original_position = self.current_position;
    let original_char = self.current_char;
    self.jump_to_start();
    let mut is_in_little_quotes = false;
    let mut is_in_big_quotes = false;
    self.walk_to_end(|me| {
        if me.current_char == '"' {
            is_in_big_quotes = !is_in_big_quotes;
        } else if me.current_char == '\'' {
            is_in_little_quotes = !is_in_little_quotes;
        }
        if me.current_position == original_position {
            return false;
        }
        true
    });
    if !is_in_little_quotes && !is_in_big_quotes {
        self.current_position = original_position;
        self.current_char = original_char;
        return false;
    }
    self.current_position = original_position;
    self.current_char = self.chars[original_position];
    let mut found = false;
    self.walk_to_end(|me| {
        if (is_in_big_quotes && me.current_char == '"') ||
          (is_in_little_quotes && me.current_char == '\'') {
            found = true;
            return false;
        }
        true
    });
    self.current_position = original_position;
    self.current_char = original_char;
    found
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_in_quote() {
    let mut rlex = Rlex::new("Someone said \"something big\" but this 'is' not in \"quotes".to_owned()).unwrap();
    rlex.walk_forward_until('s');
    rlex.walk_forward_until('s');
    assert!(rlex.is_in_quotes());
    rlex.walk_forward_until('b');
    assert!(rlex.is_in_quotes());
    rlex.walk_forward_until('b');
    assert!(!rlex.is_in_quotes());
    rlex.walk_forward_until('i');
    rlex.walk_forward_until('i');
    assert!(rlex.is_in_quotes());
    rlex.walk_forward_until('i');
    assert!(!rlex.is_in_quotes());
  }

  #[test]
  fn test_walk_forward_until() {
    let mut rlex = Rlex::new("Melody".to_owned()).unwrap();
    let found = rlex.walk_forward_until('d');
    assert!(found);
    assert_eq!(rlex.current_char, 'd');
  }

  #[test]
  fn test_walk_back_until() {
    let mut rlex = Rlex::new("Melody".to_owned()).unwrap();
    rlex.jump_to_end();
    let found = rlex.walk_back_until('e');
    assert!(found);
    assert_eq!(rlex.current_char, 'e');
  }

  #[test]
  fn test_stash_use_mark() {
    let mut rlex = Rlex::new("abcde".to_owned()).unwrap();
    rlex.step_forward(); // 'b'
    rlex.mark_current_position(); // mark at 'b'
    rlex.step_forward(); // 'c'
    rlex.step_forward(); // 'd'
    rlex.stash_use_mark(); // stash b, c, d
    let stash = rlex.stash_flush();
    println!("{:?}", stash);
    assert_eq!(stash, vec!['b', 'c', 'd']);
  }

  #[test]
  fn test_peek_forward_and_back() {
    let mut rlex = Rlex::new("hello".to_owned()).unwrap();
    assert_eq!(rlex.peek_forward(1), Some('e'));
    rlex.step_forward(); // move to 'e'
    assert_eq!(rlex.peek_back(1), Some('h'));
  }

  #[test]
  fn test_jump_to_start_and_end() {
    let mut rlex = Rlex::new("rust".to_owned()).unwrap();
    rlex.jump_to_end();
    assert_eq!(rlex.current_char, 't');
    rlex.jump_to_start();
    assert_eq!(rlex.current_char, 'r');
  }

  #[test]
  fn test_mark_and_jump_to_mark() {
    let mut rlex = Rlex::new("abcxyz".to_owned()).unwrap();
    rlex.step_forward(); // 'b'
    rlex.mark_current_position();
    rlex.step_forward(); // 'c'
    rlex.step_forward(); // 'x'
    assert_eq!(rlex.current_char, 'x');
    rlex.jump_to_mark(); // back to 'b'
    assert_eq!(rlex.current_char, 'b');
  }

  #[test]
  fn test_stash_flush_and_reset() {
    let mut rlex = Rlex::new("xyz".to_owned()).unwrap();
    rlex.stash_current_char(); // 'x'
    rlex.step_forward();       // 'y'
    rlex.stash_current_char();
    let flushed = rlex.stash_flush();
    assert_eq!(flushed, vec!['x', 'y']);
    assert!(rlex.stash.is_empty());
  }

  #[test]
  fn test_walk_to_end_stops_correctly() {
    let mut rlex = Rlex::new("abc".to_owned()).unwrap();
    let mut visited = vec![];
    rlex.walk_to_end(|me| {
      visited.push(me.current_char);
      true
    });
    assert_eq!(visited, vec!['a', 'b', 'c']);
    assert_eq!(rlex.current_char, 'c');
  }

  #[test]
  fn test_peek_out_of_bounds_returns_none() {
    let mut rlex = Rlex::new("go".to_owned()).unwrap();
    assert_eq!(rlex.peek_forward(5), None);
    assert_eq!(rlex.peek_back(1), None);
    rlex.step_forward();
    assert_eq!(rlex.peek_forward(1), None);
    assert_eq!(rlex.peek_back(2), None);
  }


}
