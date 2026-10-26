const DICTIONARY: &[(&[u8], &str)] = &[("imp".as_bytes(), "😈")];
const BACKSPACE: u8 = 8;

enum State {
    Idle,
    Recording,
}

struct Buffer {
    buf: [u8; 32],
    len: usize,
}
impl Buffer {
    fn new() -> Self {
        Self {
            buf: [0; 32],
            len: 0,
        }
    }

    fn append(&mut self, val: u8) {
        if self.len < 32 {
            self.buf[self.len] = val;
            self.len += 1;
        }
    }

    fn remove(&mut self) {
        self.len = self.len.saturating_sub(1);
    }

    fn as_bytes(&self) -> &[u8] {
        &self.buf[0..self.len]
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn reset(&mut self) {
        self.len = 0;
    }
}

pub struct Env {
    buffer: Buffer,
    state: State,
}
impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}
impl Env {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            state: State::Idle,
        }
    }

    pub fn handle_key(&mut self, key: u8) -> Option<(&'static str, usize)> {
        match (&self.state, key) {
            /*
            '/'                      -> start fresh, set to recording;
            idle                     -> nothing;
            recording, enter         -> search dictionary and reset buffer;
            recording, backspace     -> decrement buffer length (acts as pop);
            recording, any other key -> add to buffer;
            */
            (_, b'/') => {
                self.buffer.reset();
                self.state = State::Recording;
            }
            (State::Recording, b'\\') => {
                self.state = State::Idle;
                return self.handle_buffer();
            }
            (State::Recording, BACKSPACE) if self.buffer.is_empty() => {
                if self.buffer.is_empty() {
                    self.state = State::Idle;
                };
            }
            (State::Recording, BACKSPACE) => {
                self.buffer.remove();
            }
            (State::Recording, k) => {
                self.buffer.append(k);
            }
            (State::Idle, _) => {}
        }
        None
    }

    fn handle_buffer(&mut self) -> Option<(&'static str, usize)> {
        let typed_word = self.buffer.as_bytes();

        DICTIONARY
            .binary_search_by_key(&typed_word, |&(key, _)| key)
            .ok()
            .map(|index| (DICTIONARY[index].1, DICTIONARY[index].1.len()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imp_binary_input_4_none_1_some() {
        let keys = [b'/', b'i', b'm', b'p', b'\\'];
        let mut env = Env::new();
        for i in 0..keys.len() - 1 {
            assert_eq!(env.handle_key(keys[i]), None);
        }
        assert_eq!(env.handle_key(keys[keys.len() - 1]), Some(("😈", 4)));
    }

    #[test]
    fn test_imp_binary_input_with_backspace() {
        let keys = [b'/', b'i', b'n', BACKSPACE, b'm', b'p', b'\\'];
        let mut env = Env::new();
        for i in 0..keys.len() - 1 {
            assert_eq!(env.handle_key(keys[i]), None);
        }
        assert_eq!(env.handle_key(keys[keys.len() - 1]), Some(("😈", 4)));
    }

    #[test]
    fn test_imp_binary_input_with_underflow_prevention() {
        let keys = [
            BACKSPACE, BACKSPACE, BACKSPACE, BACKSPACE, BACKSPACE, b'/', b'i', b'm', b'p', b'\\',
        ];
        let mut env = Env::new();
        for i in 0..keys.len() - 1 {
            assert_eq!(env.handle_key(keys[i]), None);
        }
        assert_eq!(env.handle_key(keys[keys.len() - 1]), Some(("😈", 4)));
    }

    #[test]
    fn test_recording_state_persistence_when_slash_erased() {
        let keys = [b'/', BACKSPACE, b'i', b'm', b'p', b'\\'];
        let mut env = Env::new();
        for key in keys {
            assert_eq!(env.handle_key(key), None);
        }
    }

    #[test]
    fn test_recording_state_persistence_when_slash_erased_multiple() {
        let keys = [
            b'/', b'i', b'm', BACKSPACE, BACKSPACE, BACKSPACE, b'i', b'm', b'p', b'\\',
        ];
        let mut env = Env::new();
        for key in keys {
            assert_eq!(env.handle_key(key), None);
        }
    }
}
