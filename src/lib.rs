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
    should_trace: bool,
    trace: Vec<String>,
}

impl<S, T> Rlex<S, T>
where
    T: std::fmt::Debug,
    S: std::fmt::Debug,
{
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
            should_trace: false,
            trace: vec![],
        };
        rlex
    }

    /// Turns on the trace system
    pub fn trace_on(&mut self) {
        self.should_trace = true;
    }

    /// Turns off the trace system
    pub fn trace_off(&mut self) {
        self.should_trace = false;
    }

    /// If the trace is on, will push the msg to into the trace
    fn trace_log(&mut self, msg: &str) {
        self.trace
            .push(format!("{}:{}", self.trace.len(), msg.to_string() + "\n"));
    }

    /// Converts the trace into a String and returns it
    pub fn trace_emit(&self) -> String {
        let mut trace = "".to_string();
        for s in &self.trace {
            trace += &s;
        }
        return trace;
    }

    pub fn trace_clear(&mut self) {
        self.trace = vec![];
    }

    /// Get a reference to the tokens
    pub fn toks(&mut self) -> &Vec<T> {
        if self.should_trace {
            self.trace_log(&format!("toks() -> {:?}", self.tokens));
        }
        return &self.tokens;
    }

    /// Get the source
    pub fn src(&mut self) -> &str {
        if self.should_trace {
            self.trace_log(&format!("src()"));
        }
        return &self.source;
    }

    /// Get the stashed tokens
    pub fn token_consume(self) -> Vec<T> {
        return self.tokens;
    }

    /// Adds a token to the stack.
    pub fn token_push(&mut self, tok: T) {
        if self.should_trace {
            self.trace_log(&format!("token_push({:?})", tok));
        }
        return self.tokens.push(tok);
    }

    /// Removes and returns the last token.
    pub fn token_pop(&mut self) -> Option<T> {
        let tok = self.tokens.pop();
        if self.should_trace {
            self.trace_log(&format!("token_pop() -> {:?}", tok));
        }
        return tok;
    }

    /// Returns the last token without removing it.
    pub fn token_prev(&mut self) -> Option<&T> {
        if self.should_trace {
            self.trace_log(&format!(
                "token_prev() -> {:?}",
                self.tokens.last().map(|s| s)
            ));
        }
        return self.tokens.last().map(|s| s);
    }

    /// Returns a reference to the current state.
    pub fn state(&mut self) -> &S {
        if self.should_trace {
            self.trace_log(&format!("state() -> {:?}", &self.state));
        }
        &self.state
    }

    /// Sets the current state.
    pub fn state_set(&mut self, state: S) {
        if self.should_trace {
            self.trace_log(&format!("state_set({:?})", state));
        }
        self.state = state;
    }

    /// Returns the current character index position.
    pub fn pos(&mut self) -> usize {
        if self.should_trace {
            self.trace_log(&format!("pos() -> {}", self.position));
        }
        self.position
    }

    /// Advances the lexer by one character, unless already at the end.
    pub fn next(&mut self) -> &Rlex<S, T> {
        if self.should_trace {
            self.trace_log(&format!("next()"));
        }
        if self.position < self.max_position {
            self.position += 1;
        }
        self
    }

    /// Advances the lexer by a specified number of characters.
    pub fn next_by(&mut self, by: usize) -> &Rlex<S, T> {
        if self.should_trace {
            self.trace_log(&format!("next_by({})", by))
        }
        let mut count = 0;
        while count != by {
            self.next();
            count += 1;
        }
        self
    }

    /// Advances the lexer until a specific character is found or end is reached.
    pub fn next_until(&mut self, search: char) -> &Rlex<S, T> {
        if self.should_trace {
            self.trace_log(&format!("next_until({})", search));
        }
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
        if self.should_trace {
            self.trace_log(&format!("next_is({})", check));
        }
        self.peek() == check
    }

    /// Checks if the character `by` positions ahead matches the given character.
    pub fn next_by_is(&mut self, check: char, by: usize) -> bool {
        if self.should_trace {
            self.trace_log(&format!("next_by_is({}, {})", check, by));
        }
        self.peek_by(by) == check
    }

    /// Moves the lexer back by one character, unless at the start.
    pub fn prev(&mut self) -> &Rlex<S, T> {
        if self.should_trace {
            self.trace_log(&format!("prev()"))
        }
        if self.position > 0 {
            self.position -= 1;
        }
        self
    }

    /// Moves the lexer back by a specified number of characters.
    pub fn prev_by(&mut self, mut by: usize) -> &Rlex<S, T> {
        if self.should_trace {
            self.trace_log(&format!("prev_by({})", by));
        }
        while by != 0 {
            self.prev();
            by -= 1;
        }
        self
    }

    /// Moves the lexer backward until a specific character is found or start is reached.
    pub fn prev_until(&mut self, search: char) -> &Rlex<S, T> {
        if self.should_trace {
            self.trace_log(&format!("prev_until({})", search));
        }
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
        if self.should_trace {
            self.trace_log(&format!("prev_is({})", check));
        }
        self.peek_back() == check
    }

    /// Checks if the character `by` positions behind matches the given character.
    pub fn prev_by_is(&mut self, check: char, by: usize) -> bool {
        if self.should_trace {
            self.trace_log(&format!("prev_by_is({}, {})", check, by));
        }
        self.peek_back_by(by) == check
    }

    /// Returns the character at the current position.
    pub fn char(&mut self) -> char {
        if self.should_trace {
            self.trace_log(&format!("char() -> {}", self.chars[self.position]));
        }
        self.chars[self.position]
    }

    /// Returns `true` if the lexer is at the end of the input.
    pub fn at_end(&mut self) -> bool {
        let is_at_end = self.position == self.max_position;
        if self.should_trace {
            self.trace_log(&format!("at_end() -> {}", is_at_end));
        }
        is_at_end
    }

    /// Returns `true` if the lexer is at the beginning of the input.
    pub fn at_start(&mut self) -> bool {
        let is_at_start = self.position == 0;
        if self.should_trace {
            self.trace_log(&format!("at_start() -> {}", is_at_start));
        }
        is_at_start
    }

    /// Returns `true` if the current position is at the marked position.
    pub fn at_mark(&mut self) -> bool {
        let is_at_mark = self.marked_position == self.position;
        if self.should_trace {
            self.trace_log(&format!("at_mark() -> {}", is_at_mark));
        }
        is_at_mark
    }

    /// Marks the current position.
    pub fn mark(&mut self) -> &Rlex<S, T> {
        if self.should_trace {
            self.trace_log(&format!("mark()"));
        }
        self.marked_position = self.position;
        self
    }

    /// Moves the current position to a specific index.
    pub fn goto_pos(&mut self, pos: usize) -> &Rlex<S, T> {
        if self.should_trace {
            self.trace_log(&format!("goto_pos({})", pos));
        }
        if pos > self.max_position {
            self.position = self.max_position;
            return self;
        }
        self.position = pos;
        self
    }

    /// Moves the current position back to the previously marked index.
    pub fn goto_mark(&mut self) -> &Rlex<S, T> {
        if self.should_trace {
            self.trace_log(&format!("goto_mark()"));
        }
        self.position = self.marked_position;
        self
    }

    /// Moves the current position to the start of the input.
    pub fn goto_start(&mut self) -> &Rlex<S, T> {
        if self.should_trace {
            self.trace_log(&format!("goto_start()"));
        }
        self.position = 0;
        self
    }

    /// Moves the current position to the end of the input.
    pub fn goto_end(&mut self) -> &Rlex<S, T> {
        if self.should_trace {
            self.trace_log(&format!("goto_end()"));
        }
        self.position = self.max_position;
        self
    }

    /// Peeks at the next character without advancing the position.
    pub fn peek(&mut self) -> char {
        let start = self.position;
        self.next();
        let ch = self.char();
        self.goto_pos(start);
        if self.should_trace {
            self.trace_log(&format!("peek() -> {}", ch));
        }
        ch
    }

    /// Peeks ahead by `by` characters without advancing the position.
    pub fn peek_by(&mut self, by: usize) -> char {
        let start = self.position;
        self.next_by(by);
        let ch = self.char();
        self.goto_pos(start);
        if self.should_trace {
            self.trace_log(&format!("peek_by({}) -> {}", by, ch));
        }
        ch
    }

    /// Peeks at the previous character without changing the position.
    pub fn peek_back(&mut self) -> char {
        let start = self.position;
        self.prev();
        let ch = self.char();
        self.goto_pos(start);
        if self.should_trace {
            self.trace_log(&format!("peek_back() -> {}", ch));
        }
        ch
    }

    /// Peeks behind by `by` characters without changing the position.
    pub fn peek_back_by(&mut self, by: usize) -> char {
        let start = self.position;
        self.prev_by(by);
        let ch = self.char();
        self.goto_pos(start);
        if self.should_trace {
            self.trace_log(&format!("peek_back_by({}) -> {}", by, ch));
        }
        ch
    }

    /// Returns a string slice from the source based on start and end positions.
    pub fn str_from_rng(&mut self, mut start: usize, mut end: usize) -> &str {
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
        let str = &self.source[start_byte..start_byte + byte_len];
        return str;
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
    pub fn is_in_quote(&mut self) -> bool {
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
		let result = in_big_quote || in_lil_quote;
		if self.should_trace {
			self.trace_log(&format!("is_in_quote() -> {}", result));
		}
        in_big_quote || in_lil_quote
    }

    /// Adds the current character to the internal collection buffer.
    pub fn collect(&mut self) {
        if self.should_trace {
            self.trace_log(&format!("collect()"));
        }
        let char = self.char();
        self.collection.push(char);
    }

    /// Returns the string collected so far from the buffer.
    pub fn str_from_collection(&mut self) -> &str {
        self.collection_str = self.collection.iter().collect();
        &self.collection_str
    }

    /// Clears the internal character collection buffer.
    pub fn collect_clear(&mut self) {
		if self.should_trace {
			self.trace_log(&format!("collect_clear()"))
		}
        self.collection = vec![];
        self.collection_str = "".to_owned();
    }

    /// Removes and returns the last character from the collection buffer.
    pub fn collect_pop(&mut self) -> Option<char> {
		let option = self.collection.pop();
		if self.should_trace {
			self.trace_log(&format!("collect_pop() -> {:?}", option));
		}
		option
    }

    /// Adds a character to the collection buffer.
    pub fn collect_push(&mut self, c: char) {
		if self.should_trace {
			self.trace_log(&format!("collect_push({})", c));
		}
        self.collection.push(c);
    }
}

/// A public default state for when you want an Rlex and don't care about the state
#[derive(Debug, PartialEq, Eq)]
pub enum DefaultState {
    Default,
}

/// A public default token for when you want an Rlex and don't care to collect tokens
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DefaultToken {
    Default,
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
    fn test_trace() {
        let mut r: Rlex<State, Token> = Rlex::new("abcd", State::Init);
        r.token_push(Token::Tok1);
        r.trace_on();
        r.toks();
        assert!(r.trace_emit() == "0:toks() -> [Tok1]\n");
        r.trace_clear();
        r.src();
        assert!(r.trace_emit() == "0:src()\n");
        r.trace_clear();
        r.token_push(Token::Tok1);
        assert!(r.trace_emit() == "0:token_push(Tok1)\n");
    }

    #[test]
    fn test_src() {
        let mut r: Rlex<State, Token> = Rlex::new("abcd", State::Init);
        assert!(r.src() == "abcd");
    }

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
        assert!(!r.is_in_quote());
        assert!(r.char() == '"');
        let mut r: Rlex<State, Token> = Rlex::new("Hello, I am not Quoted!", State::Init);
        while !r.at_end() {
            assert!(!r.is_in_quote());
            r.next();
        }
        let mut r: Rlex<State, Token> = Rlex::new("<p name='bob'>", State::Init);
        r.next_until('b');
        assert!(r.is_in_quote());
        r.next_until('\'');
        assert!(!r.is_in_quote());
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
        r.collect_clear();
        assert!(r.str_from_collection() == "");
    }
}
