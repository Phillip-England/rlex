/// A generic lexer that allows traversal, peeking, marking, and collection of characters
/// from a string source. Useful for building parsers or tokenizers.
#[derive(Debug)]
pub struct Rlex<S, T> {
    source: String,
    chars: Vec<char>,
    position: usize,
    max_position: usize,
    marked_position: usize,
    state: S,
    collection: Vec<char>,
    collection_str: String,
    tokens: Vec<T>,
}

impl<S, T> Rlex<S, T> {
    /// Creates a new lexer from a non-empty string and an initial state.
    ///
    /// # Errors
    ///
    /// Returns an error if the source string is empty.
    pub fn new(source: &str, state: S) -> Rlex<S, T> {
        let chars: Vec<char> = source.chars().collect();
        let length = chars.len();
        let rlex = Rlex {
            source: source.to_owned(),
            chars,
            position: 0,
            max_position: length - 1,
            marked_position: 0,
            state,
            collection: vec![],
            collection_str: "".to_owned(),
            tokens: vec![],
        };
        rlex
    }

    /// Get the stashed tokens
    pub fn token_consume(self) -> Vec<T> {
        return self.tokens;
    }

    /// Adds a token to the stack.
    pub fn token_push(&mut self, tok: T) {
        return self.tokens.push(tok);
    }

    /// Removes and returns the last token.
    pub fn token_pop(&mut self) -> Option<T> {
        return self.tokens.pop()
    }

    /// Returns the last token without removing it.
    pub fn token_prev(&self) -> Option<&T> {
        return self.tokens.last().map(|s| s);
    }

    /// Returns a reference to the current state.
    pub fn state(&self) -> &S {
        &self.state
    }

    /// Sets the current state.
    pub fn state_set(&mut self, state: S) {
        self.state = state;
    }

    /// Returns the current character index position.
    pub fn pos(&self) -> usize {
        self.position
    }

    /// Advances the lexer by one character, unless already at the end.
    pub fn next(&mut self) -> &Rlex<S, T> {
        if self.position < self.max_position {
            self.position += 1;
        }
        self
    }

    /// Advances the lexer by a specified number of characters.
    pub fn next_by(&mut self, by: usize) -> &Rlex<S, T> {
        let mut count = 0;
        while count != by {
            self.next();
            count += 1;
        }
        self
    }

    /// Advances the lexer until a specific character is found or end is reached.
    pub fn next_until(&mut self, search: char) -> &Rlex<S, T> {
        while self.char() != search {
            if self.at_end() {
                break;
            }
            self.next();
        }
        self
    }

    /// Checks if the next character matches the given character.
    pub fn next_is(&mut self, check: char) -> bool {
        self.peek() == check
    }

    /// Checks if the character `by` positions ahead matches the given character.
    pub fn next_by_is(&mut self, check: char, by: usize) -> bool {
        self.peek_by(by) == check
    }

    /// Moves the lexer back by one character, unless at the start.
    pub fn prev(&mut self) -> &Rlex<S, T> {
        if self.position > 0 {
            self.position -= 1;
        }
        self
    }

    /// Moves the lexer back by a specified number of characters.
    pub fn prev_by(&mut self, mut by: usize) -> &Rlex<S, T> {
        while by != 0 {
            self.prev();
            by -= 1;
        }
        self
    }

    /// Moves the lexer backward until a specific character is found or start is reached.
    pub fn prev_until(&mut self, search: char) -> &Rlex<S, T> {
        while self.char() != search {
            if self.at_start() {
                break;
            }
            self.prev();
        }
        self
    }

    /// Checks if the previous character matches the given character.
    pub fn prev_is(&mut self, check: char) -> bool {
        self.peek_back() == check
    }

    /// Checks if the character `by` positions behind matches the given character.
    pub fn prev_by_is(&mut self, check: char, by: usize) -> bool {
        self.peek_back_by(by) == check
    }

    /// Returns the character at the current position.
    pub fn char(&self) -> char {
        self.chars[self.position]
    }

    /// Returns `true` if the lexer is at the end of the input.
    pub fn at_end(&mut self) -> bool {
        self.position == self.max_position
    }

    /// Returns `true` if the lexer is at the beginning of the input.
    pub fn at_start(&mut self) -> bool {
        self.position == 0
    }

    /// Returns `true` if the current position is at the marked position.
    pub fn at_mark(&mut self) -> bool {
        self.position == self.marked_position
    }

    /// Marks the current position.
    pub fn mark(&mut self) -> &Rlex<S, T> {
        self.marked_position = self.position;
        self
    }

    /// Moves the current position to a specific index.
    pub fn goto_pos(&mut self, pos: usize) -> &Rlex<S, T> {
        if pos > self.max_position {
            self.position = self.max_position;
            return self;
        }
        self.position = pos;
        self
    }

    /// Moves the current position back to the previously marked index.
    pub fn goto_mark(&mut self) -> &Rlex<S, T> {
        self.position = self.marked_position;
        self
    }

    /// Moves the current position to the start of the input.
    pub fn goto_start(&mut self) -> &Rlex<S, T> {
        self.position = 0;
        self
    }

    /// Moves the current position to the end of the input.
    pub fn goto_end(&mut self) -> &Rlex<S, T> {
        self.position = self.max_position;
        self
    }

    /// Peeks at the next character without advancing the position.
    pub fn peek(&mut self) -> char {
        let start = self.position;
        self.next();
        let ch = self.char();
        self.goto_pos(start);
        ch
    }

    /// Peeks ahead by `by` characters without advancing the position.
    pub fn peek_by(&mut self, by: usize) -> char {
        let start = self.position;
        self.next_by(by);
        let ch = self.char();
        self.goto_pos(start);
        ch
    }

    /// Peeks at the previous character without changing the position.
    pub fn peek_back(&mut self) -> char {
        let start = self.position;
        self.prev();
        let ch = self.char();
        self.goto_pos(start);
        ch
    }

    /// Peeks behind by `by` characters without changing the position.
    pub fn peek_back_by(&mut self, by: usize) -> char {
        let start = self.position;
        self.prev_by(by);
        let ch = self.char();
        self.goto_pos(start);
        ch
    }

    /// Returns a string slice from the source based on start and end positions.
    pub fn str_from_rng(&self, mut start: usize, mut end: usize) -> &str {
        if start > self.max_position {
            start = self.max_position;
        }
        if end > self.max_position {
            end = self.max_position;
        }
        if start > end {
            std::mem::swap(&mut start, &mut end);
        }
        let start_byte = self.chars[..start]
            .iter()
            .map(|c| c.len_utf8())
            .sum::<usize>();
        let byte_len = self.chars[start..=end]
            .iter()
            .map(|c| c.len_utf8())
            .sum::<usize>();
        &self.source[start_byte..start_byte + byte_len]
    }

    /// Returns a string slice between the marked position and the current position.
    pub fn str_from_mark(&self) -> &str {
        let (start, end) = if self.marked_position <= self.position {
            (self.marked_position, self.position)
        } else {
            (self.position, self.marked_position)
        };
        let start_byte = self.chars[..start]
            .iter()
            .map(|c| c.len_utf8())
            .sum::<usize>();

        let byte_len = self.chars[start..=end]
            .iter()
            .map(|c| c.len_utf8())
            .sum::<usize>();
        &self.source[start_byte..start_byte + byte_len]
    }

    /// Returns a string slice from the start up to the current position.
    pub fn str_from_start(&self) -> &str {
        let start = 0;
        let end = self.position.min(self.max_position) + 1;
        let start_byte = self.chars[start..end]
            .iter()
            .map(|c| c.len_utf8())
            .take(start)
            .sum::<usize>();
        let byte_len = self.chars[start..end]
            .iter()
            .map(|c| c.len_utf8())
            .sum::<usize>();
        &self.source[start_byte..start_byte + byte_len]
    }

    /// Returns a string slice from the current position to the end.
    pub fn str_from_end(&self) -> &str {
        let start = self.position;
        let end = self.max_position + 1;
        let start_byte = self.chars[..start]
            .iter()
            .map(|c| c.len_utf8())
            .sum::<usize>();
        let byte_len = self.chars[start..end]
            .iter()
            .map(|c| c.len_utf8())
            .sum::<usize>();
        &self.source[start_byte..start_byte + byte_len]
    }

    /// Checks whether the lexer is currently inside a quoted string.
    pub fn is_in_quote(&self) -> bool {
        let mut in_big_quote = false;
        let mut in_lil_quote = false;
        let mut escaped = false;
        for c in self.str_from_start().chars() {
            if escaped {
                escaped = false;
                continue;
            }
            if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_big_quote = !in_big_quote;
            } else if c == '\'' {
                in_lil_quote = !in_lil_quote;
            }
        }
        in_big_quote || in_lil_quote
    }

    /// Adds the current character to the internal collection buffer.
    pub fn collect(&mut self) {
        self.collection.push(self.char());
    }

    /// Returns the string collected so far from the buffer.
    pub fn str_from_collection(&mut self) -> &str {
        self.collection_str = self.collection.iter().collect();
        &self.collection_str
    }

    /// Clears the internal character collection buffer.
    pub fn collect_reset(&mut self) {
        self.collection = vec![];
        self.collection_str = "".to_owned();
    }

    /// Removes and returns the last character from the collection buffer.
    pub fn collect_pop(&mut self) -> Option<char> {
        self.collection.pop()
    }

    /// Adds a character to the collection buffer.
    pub fn collect_push(&mut self, c: char) {
        self.collection.push(c);
    }
}


#[derive(Debug, PartialEq, Eq)]
enum State {
    Init,
    Open,
    Closed,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    Tok1,
    Tok2,
    Tok3,
}

#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn test_tokens() {
        let mut r: Rlex<State, Token> = Rlex::new("abcd", State::Init);
        r.token_push(Token::Tok1);
        assert!(r.token_prev().unwrap() == &Token::Tok1);
        assert!(r.token_pop().unwrap() == Token::Tok1);
        assert!(r.token_prev() == None);
        r.token_push(Token::Tok1);
        r.token_push(Token::Tok2);
        assert!(r.token_consume() == vec![Token::Tok1, Token::Tok2]);
    }

    #[test]
    fn test_rlex_next_and_prev() {
        let mut r: Rlex<State, Token> = Rlex::new("abcd", State::Init);
        assert_eq!(r.char(), 'a');
        r.next();
        assert_eq!(r.char(), 'b');
        r.next();
        assert_eq!(r.char(), 'c');
        r.next();
        assert_eq!(r.char(), 'd');
        r.next();
        assert_eq!(r.char(), 'd');
        r.next();
        assert_eq!(r.char(), 'd');
        r.prev();
        assert_eq!(r.char(), 'c');
        r.prev();
        assert_eq!(r.char(), 'b');
        r.prev();
        assert_eq!(r.char(), 'a');
        r.prev();
        assert_eq!(r.char(), 'a');
        r.prev();
        assert_eq!(r.char(), 'a');
    }

    #[test]
    fn test_rlex_at_start_and_at_end() {
        let mut r: Rlex<State, Token> = Rlex::new("abcd", State::Init);
        while !r.at_end() {
            r.next();
        }
        assert!(r.at_end());
        while !r.at_start() {
            r.prev();
        }
        assert!(r.at_start());
    }

    #[test]
    fn test_rlex_next_by() {
        let mut r: Rlex<State, Token> = Rlex::new("abcd", State::Init);
        r.next_by(0);
        assert!(r.char() == 'a');
        r.next_by(1);
        assert!(r.char() == 'b');
        r.goto_start();
        r.next_by(2);
        assert!(r.char() == 'c');
        r.goto_start();
        r.next_by(3);
        assert!(r.char() == 'd');
        r.goto_start();
        r.next_by(4);
        assert!(r.char() == 'd');
    }

    #[test]
    fn test_rlex_peek() {
        let mut r: Rlex<State, Token> = Rlex::new("abcd", State::Init);
        assert!(r.peek() == 'b');
        r.goto_end();
        assert!(r.peek() == 'd');
    }

    #[test]
    fn test_rlex_peek_by() {
        let mut r: Rlex<State, Token> = Rlex::new("abcd", State::Init);
        assert!(r.peek_by(0) == 'a');
        assert!(r.peek_by(1) == 'b');
        assert!(r.peek_by(2) == 'c');
        assert!(r.peek_by(3) == 'd');
        assert!(r.peek_by(4) == 'd');
    }

    #[test]
    fn test_rlex_peek_back() {
        let mut r: Rlex<State, Token> = Rlex::new("abcd", State::Init);
        r.goto_end();
        assert!(r.peek_back() == 'c');
        r.goto_start();
        assert!(r.peek_back() == 'a');
    }

    #[test]
    fn test_rlex_peek_back_by() {
        let mut r: Rlex<State, Token> = Rlex::new("abcd", State::Init);
        r.goto_end();
        assert!(r.peek_back_by(0) == 'd');
        assert!(r.peek_back_by(1) == 'c');
        assert!(r.peek_back_by(2) == 'b');
        assert!(r.peek_back_by(3) == 'a');
        assert!(r.peek_back_by(4) == 'a');
    }

    #[test]
    fn test_rlex_str_from() {
        let mut r: Rlex<State, Token> = Rlex::new("abcd", State::Init);
        r.next();
        assert!(r.str_from_start() == "ab");
        r.goto_end();
        assert!(r.str_from_start() == "abcd");
        r.prev();
        r.mark();
        r.next();
        assert!(r.str_from_mark() == "cd");
        r.goto_start();
        assert!(r.str_from_end() == "abcd");
        r.next();
        assert!(r.str_from_end() == "bcd");
        r.next();
        assert!(r.str_from_end() == "cd");
        r.next();
        assert!(r.str_from_end() == "d");
        assert!(r.str_from_rng(0, 0) == "a");
        assert!(r.str_from_rng(0, 1) == "ab");
        assert!(r.str_from_rng(0, 2) == "abc");
        assert!(r.str_from_rng(0, 3) == "abcd");
        assert!(r.str_from_rng(0, 22) == "abcd");
        assert!(r.str_from_rng(22, 0) == "abcd");
    }

    #[test]
    fn test_rlex_is_in_quote() {
        let mut r: Rlex<State, Token> = Rlex::new("\"Hello, I am Quoted!\"", State::Init);
        while !r.at_end() {
            assert!(r.is_in_quote());
            r.next();
        }
        let mut r: Rlex<State, Token> = Rlex::new("Hello, I am not Quoted!", State::Init);
        while !r.at_end() {
            assert!(!r.is_in_quote());
            r.next();
        }
        let mut r: Rlex<State, Token> = Rlex::new("<p name='bob'>", State::Init);
        r.next_until('b');
        assert!(r.is_in_quote());
    }

    #[test]
    fn test_rlex_next_until_and_prev_until() {
        let mut r: Rlex<State, Token> = Rlex::new("abcd", State::Init);
        r.next_until('c');
        assert!(r.pos() == 2);
        r.next();
        r.prev_until('b');
        assert!(r.pos() == 1);
    }

    #[test]
    fn test_rlex_surrounding_comparisons() {
        let mut r: Rlex<State, Token> = Rlex::new("abcd", State::Init);
        assert!(r.next_is('b'));
        assert!(r.next_by_is('a', 0));
        assert!(r.next_by_is('b', 1));
        assert!(r.next_by_is('c', 2));
        assert!(r.next_by_is('d', 3));
        assert!(r.next_by_is('d', 4));
        r.goto_end();
        assert!(r.prev_is('c'));
        assert!(r.prev_by_is('d', 0));
        assert!(r.prev_by_is('c', 1));
        assert!(r.prev_by_is('b', 2));
        assert!(r.prev_by_is('a', 3));
        assert!(r.prev_by_is('a', 4));
    }

    #[test]
    fn test_rlex_state() {
        let mut r: Rlex<State, Token> = Rlex::new("abcd", State::Init);
        assert!(r.state() == &State::Init);
        r.state_set(State::Open);
        assert!(r.state() == &State::Open);
    }

    #[test]
    fn test_rlex_collect() {
        let mut r: Rlex<State, Token> = Rlex::new("abcd", State::Init);
        r.collect();
        assert!(r.str_from_collection() == "a");
        let c = r.collect_pop();
        assert!(c.unwrap() == 'a');
        r.collect_push('a');
        assert!(r.str_from_collection() == "a");
        r.collect_reset();
        assert!(r.str_from_collection() == "");
    }

}
