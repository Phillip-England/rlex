// Represents a lexer that can traverse, peek, and stash characters from a string source
#[derive(Debug)]
pub struct Rlex<T> {
    source: String,
    chars: Vec<char>,
    position: usize,
    max_position: usize,
    marked_position: usize,
    state: T,
    collection: Vec<char>,
    collection_str: String,
}

impl<T> Rlex<T> {
    pub fn new(source: &str, state: T) -> Result<Rlex<T>, String> {
        if source.is_empty() {
            return Err("MALFORMED INPUT: rlex does not accept empty strings".to_owned());
        }
        let chars: Vec<char> = source.chars().collect();
        let length = chars.len();
        let rlex = Rlex {
            source: source.to_owned(),
            chars,
            position: 0,
            max_position: length - 1,
            marked_position: 0,
            state: state,
            collection: vec![],
            collection_str: "".to_owned(),
        };
        Ok(rlex)
    }

    pub fn state(&self) -> &T {
        return &self.state;
    }

    pub fn state_set(&mut self, state: T) {
        self.state = state;
    }

    pub fn pos(&self) -> usize {
        return self.position;
    }

    pub fn next(&mut self) -> &Rlex<T> {
        if self.position < self.max_position {
            self.position += 1;
        }
        return self;
    }

    pub fn next_by(&mut self, by: usize) -> &Rlex<T> {
        let mut count = 0;
        while count != by {
            self.next();
            count += 1;
        }
        return self;
    }

    pub fn next_until(&mut self, search: char) -> &Rlex<T> {
        while self.char() != search {
            if self.at_end() {
                break;
            }
            self.next();
        }
        return self;
    }

    pub fn next_is(&mut self, check: char) -> bool {
        return self.peek() == check;
    }

    pub fn next_by_is(&mut self, check: char, by: usize) -> bool {
        return self.peek_by(by) == check;
    }

    pub fn prev(&mut self) -> &Rlex<T> {
        if self.position > 0 {
            self.position -= 1;
        }
        return self;
    }

    pub fn prev_by(&mut self, mut by: usize) -> &Rlex<T> {
        while by != 0 {
            self.prev();
            by -= 1;
        }
        return self;
    }

    pub fn prev_until(&mut self, search: char) -> &Rlex<T> {
        while self.char() != search {
            if self.at_start() {
                break;
            }
            self.prev();
        }
        return self;
    }

    pub fn prev_is(&mut self, check: char) -> bool {
        return self.peek_back() == check;
    }

    pub fn prev_by_is(&mut self, check: char, by: usize) -> bool {
        return self.peek_back_by(by) == check;
    }

    pub fn char(&self) -> char {
        return self.chars[self.position];
    }

    pub fn at_end(&mut self) -> bool {
        return self.position == self.max_position;
    }

    pub fn at_start(&mut self) -> bool {
        return self.position == 0;
    }

    pub fn at_mark(&mut self) -> bool {
        return self.position == self.marked_position;
    }

    pub fn mark(&mut self) -> &Rlex<T> {
        self.marked_position = self.position;
        return self;
    }

    pub fn goto_pos(&mut self, pos: usize) -> &Rlex<T> {
        if pos > self.max_position {
            self.position = self.max_position;
            return self;
        }
        self.position = pos;
        return self;
    }

    pub fn goto_mark(&mut self) -> &Rlex<T> {
        self.position = self.marked_position;
        return self;
    }

    pub fn goto_start(&mut self) -> &Rlex<T> {
        self.position = 0;
        return self;
    }

    pub fn goto_end(&mut self) -> &Rlex<T> {
        self.position = self.max_position;
        return self;
    }

    pub fn peek(&mut self) -> char {
        let start = self.position;
        self.next();
        let ch = self.char();
        self.goto_pos(start);
        return ch;
    }

    pub fn peek_by(&mut self, by: usize) -> char {
        let start = self.position;
        self.next_by(by);
        let ch = self.char();
        self.goto_pos(start);
        return ch;
    }

    pub fn peek_back(&mut self) -> char {
        let start = self.position;
        self.prev();
        let ch = self.char();
        self.goto_pos(start);
        return ch;
    }

    pub fn peek_back_by(&mut self, by: usize) -> char {
        let start = self.position;
        self.prev_by(by);
        let ch = self.char();
        self.goto_pos(start);
        return ch;
    }

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

    pub fn collect(&mut self) {
        self.collection.push(self.char());
    } 

    pub fn str_from_collection(&mut self) -> &str {
        self.collection_str = self.collection.iter().collect();
        return &self.collection_str;
    }

    pub fn collect_reset(&mut self) {
        self.collection = vec![];
        self.collection_str = "".to_owned();
    }

    pub fn collect_pop(&mut self) -> Option<char> {
        return self.collection.pop()
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_rlex_throws_error() {
        let rlex = Rlex::new("", State::Init);
        if rlex.is_ok() {
            panic!("rlex should not accept empty strings");
        }
        assert!(rlex.is_err());
    }

    #[test]
    fn test_rlex_next_and_prev() {
        let mut r = Rlex::new("abcd", State::Init).unwrap();
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
        let mut r = Rlex::new("abcd", State::Init).unwrap();
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
        let mut r = Rlex::new("abcd", State::Init).unwrap();
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
        let mut r = Rlex::new("abcd", State::Init).unwrap();
        assert!(r.peek() == 'b');
        r.goto_end();
        assert!(r.peek() == 'd');
    }

    #[test]
    fn test_rlex_peek_by() {
        let mut r = Rlex::new("abcd", State::Init).unwrap();
        assert!(r.peek_by(0) == 'a');
        assert!(r.peek_by(1) == 'b');
        assert!(r.peek_by(2) == 'c');
        assert!(r.peek_by(3) == 'd');
        assert!(r.peek_by(4) == 'd');
    }

    #[test]
    fn test_rlex_peek_back() {
        let mut r = Rlex::new("abcd", State::Init).unwrap();
        r.goto_end();
        assert!(r.peek_back() == 'c');
        r.goto_start();
        assert!(r.peek_back() == 'a');
    }

    #[test]
    fn test_rlex_peek_back_by() {
        let mut r = Rlex::new("abcd", State::Init).unwrap();
        r.goto_end();
        assert!(r.peek_back_by(0) == 'd');
        assert!(r.peek_back_by(1) == 'c');
        assert!(r.peek_back_by(2) == 'b');
        assert!(r.peek_back_by(3) == 'a');
        assert!(r.peek_back_by(4) == 'a');
    }

    #[test]
    fn test_rlex_dump() {
        let mut r = Rlex::new("abcd", State::Init).unwrap();
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
    }

    #[test]
    fn test_rlex_is_in_quote() {
        let mut r = Rlex::new("\"Hello, I am Quoted!\"", State::Init).unwrap();
        while !r.at_end() {
            assert!(r.is_in_quote());
            r.next();
        }
        let mut r = Rlex::new("Hello, I am not Quoted!", State::Init).unwrap();
        while !r.at_end() {
            assert!(!r.is_in_quote());
            r.next();
        }
        let mut r = Rlex::new("<p name='bob'>", State::Init).unwrap();
        r.next_until('b');
        assert!(r.is_in_quote());
    }

    #[test]
    fn test_rlex_next_until_and_prev_until() {
        let mut r = Rlex::new("abcd", State::Init).unwrap();
        r.next_until('c');
        assert!(r.pos() == 2);
        r.next();
        r.prev_until('b');
        assert!(r.pos() == 1);
    }

    #[test]
    fn test_rlex_surrounding_comparisons() {
        let mut r = Rlex::new("abcd", State::Init).unwrap();
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
        let mut r = Rlex::new("abcd", State::Init).unwrap();
        assert!(r.state() == &State::Init);
        r.state_set(State::Open);
        assert!(r.state() == &State::Open);
    }

    #[test]
    fn test_rlex_collect() {
        let mut r = Rlex::new("abcd", State::Init).unwrap();
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
