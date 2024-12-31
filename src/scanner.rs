// src: https://depth-first.com/articles/2021/12/16/a-beginners-guide-to-parsing-in-rust/
pub struct Scanner {
    cursor: usize,
    characters: Vec<char>,
}

impl Scanner {
    pub fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            characters: string.chars().collect(),
        }
    }

    /// Returns the current cursor. Useful for reporting errors.
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Returns the next character without advancing the cursor.
    /// AKA "lookahead"
    pub fn peek(&self) -> Option<&char> {
        self.characters.get(self.cursor)
    }

    /// Returns true if further progress is not possible.
    pub fn is_done(&self) -> bool {
        self.cursor == self.characters.len()
    }

    /// Returns the next character (if available) and advances the cursor.
    pub fn pop(&mut self) -> Option<&char> {
        match self.characters.get(self.cursor) {
            Some(character) => {
                self.cursor += 1;

                Some(character)
            }
            None => None,
        }
    }

    /// Returns true if the `target` is found at the current cursor position,
    /// and advances the cursor.
    /// Otherwise, returns false leaving the cursor unchanged.
    pub fn take(&mut self, target: &char) -> bool {
        match self.characters.get(self.cursor) {
            Some(character) => {
                if target == character {
                    self.cursor += 1;

                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    /// Invoke `cb` once. If the result is not `None`, return it and advance
    /// the cursor. Otherwise, return None and leave the cursor unchanged.
    pub fn transform<T>(&mut self, cb: impl FnOnce(&char) -> Option<T>) -> Option<T> {
        match self.characters.get(self.cursor) {
            Some(input) => match cb(input) {
                Some(output) => {
                    self.cursor += 1;

                    Some(output)
                }
                None => None,
            },
            None => None,
        }
    }

    pub fn scan<T>(&mut self, cb: impl Fn(&str) -> Option<Action<T>>) -> Result<Option<T>, Error> {
        let mut sequence = String::new();
        let mut require = false;
        let mut request = None;

        loop {
            match self.characters.get(self.cursor) {
                Some(target) => {
                    sequence.push(*target);

                    match cb(&sequence) {
                        Some(Action::Return(result)) => {
                            self.cursor += 1;

                            break Ok(Some(result));
                        }
                        Some(Action::Request(result)) => {
                            self.cursor += 1;
                            require = false;
                            request = Some(result);
                        }
                        Some(Action::Require) => {
                            self.cursor += 1;
                            require = true;
                        }
                        None => {
                            if require {
                                break Err(Error::Character(self.cursor));
                            } else {
                                break Ok(request);
                            }
                        }
                    }
                }
                None => {
                    if require {
                        break Err(Error::EndOfLine);
                    } else {
                        break Ok(request);
                    }
                }
            }
        }
    }
}

pub enum Action<T> {
    /// If next iteration returns None, return T without advancing
    /// the cursor.
    Request(T),

    /// If the next iteration returns None, return None without advancing
    /// the cursor.
    Require,

    /// Immediately advance the cursor and return T.
    Return(T),
}

pub enum Error {
    Character(usize),
    EndOfLine,
}
