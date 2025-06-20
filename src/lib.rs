// Represents a lexer that can traverse, peek, and stash characters from a string source
#[derive(Debug)]
pub struct Rlex {
    source: String,
    chars: Vec<char>,
    position: usize,
    max_position: usize,
    marked_position: usize,
}

impl Rlex {
    pub fn new(source: &str) -> Result<Rlex, String> {
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
        };
        Ok(rlex)
    }

    fn next(&mut self) {
        if self.position < self.max_position {
            self.position += 1;
        }
    }

    fn next_by(&mut self, by: usize) {
        let mut count = 0;
        while count != by {
            self.next();
            count += 1;
        }
    }

    fn prev(&mut self) {
        if self.position > 0 {
            self.position -= 1;
        }
    }

    fn prev_by(&mut self, mut by: usize) {
        while by != 0 {
            self.prev();
            by -= 1;
        }
    }

    fn char(&self) -> char {
        return self.chars[self.position];
    }

    fn at_end(&mut self) -> bool {
        return self.position == self.max_position;
    }

    fn at_start(&mut self) -> bool {
        return self.position == 0;
    }

    fn at_mark(&mut self) -> bool {
        return self.position == self.marked_position;
    }

    fn mark(&mut self) {
        self.marked_position = self.position;
    }

    fn goto_pos(&mut self, pos: usize) {
        if pos > self.max_position {
            self.position = self.max_position;
            return;
        }
        self.position = pos;
    }

    fn goto_mark(&mut self) {
        self.position = self.marked_position;
    }

    fn goto_start(&mut self) {
        self.position = 0;
    }

    fn goto_end(&mut self) {
        self.position = self.max_position;
    }

    fn peek(&mut self) -> char {
        let start = self.position;
        self.next();
        let ch = self.char();
        self.goto_pos(start);
        return ch;
    }

    fn peek_by(&mut self, by: usize) -> char {
        let start = self.position;
        self.next_by(by);
        let ch = self.char();
        self.goto_pos(start);
        return ch;
    }

    fn peek_back(&mut self) -> char {
        let start = self.position;
        self.prev();
        let ch = self.char();
        self.goto_pos(start);
        return ch;
    }

    fn peek_back_by(&mut self, by: usize) -> char {
        let start = self.position;
        self.prev_by(by);
        let ch = self.char();
        self.goto_pos(start);
        return ch;
    }

    fn dump_from_mark(&self) -> &str {
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

    fn dump_from_start(&self) -> &str {
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

	fn dump_from_end(&self) -> &str {
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


    fn is_in_quote(&self) -> bool {
        let dump_start = self.dump_from_start();
		let dump_end = self.dump_from_end();
		println!("{}", dump_start.contains("\""));
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_rlex_throws_error() {
        let rlex = Rlex::new("");
        if rlex.is_ok() {
            panic!("rlex should not accept empty strings");
        }
        assert!(rlex.is_err());
    }

    #[test]
    fn test_rlex_next_and_prev() {
        let mut r = Rlex::new("abcd").unwrap();
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
        let mut r = Rlex::new("abcd").unwrap();
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
        let mut r = Rlex::new("abcd").unwrap();
        r.next_by(0); assert!(r.char() == 'a');
        r.next_by(1); assert!(r.char() == 'b');
        r.goto_start();
        r.next_by(2); assert!(r.char() == 'c');
        r.goto_start();
        r.next_by(3); assert!(r.char() == 'd');
        r.goto_start();
        r.next_by(4); assert!(r.char() == 'd');
    }

    #[test]
    fn test_rlex_peek() {
        let mut r = Rlex::new("abcd").unwrap();
        assert!(r.peek() == 'b');
        r.goto_end();
        assert!(r.peek() == 'd');
    }

    #[test]
    fn test_rlex_peek_by() {
        let mut r = Rlex::new("abcd").unwrap();
        assert!(r.peek_by(0) == 'a');
        assert!(r.peek_by(1) == 'b');
        assert!(r.peek_by(2) == 'c');
        assert!(r.peek_by(3) == 'd');
        assert!(r.peek_by(4) == 'd');
    }

    #[test]
    fn test_rlex_peek_back() {
        let mut r = Rlex::new("abcd").unwrap();
        r.goto_end();
        assert!(r.peek_back() == 'c');
        r.goto_start();
        assert!(r.peek_back() == 'a');
    }

    #[test]
    fn test_rlex_peek_back_by() {
        let mut r = Rlex::new("abcd").unwrap();
        r.goto_end();
        assert!(r.peek_back_by(0) == 'd');
        assert!(r.peek_back_by(1) == 'c');
        assert!(r.peek_back_by(2) == 'b');
        assert!(r.peek_back_by(3) == 'a');
        assert!(r.peek_back_by(4) == 'a');
    }

    #[test]
    fn test_rlex_dump() {
        let mut r = Rlex::new("abcd").unwrap();
        r.next(); assert!(r.dump_from_start() == "ab");
        r.goto_end(); assert!(r.dump_from_start() == "abcd");
        r.prev();
        r.mark();
        r.next(); assert!(r.dump_from_mark() == "cd");
		r.goto_start(); assert!(r.dump_from_end() == "abcd");
		r.next(); assert!(r.dump_from_end() == "bcd");
		r.next(); assert!(r.dump_from_end() == "cd");
		r.next(); assert!(r.dump_from_end() == "d");
    }

	#[test]
	fn test_rlex_is_in_quote() {
		assert!(1 == 2);
	}
}
