const DICTIONARY: &[(&str, &str)] = &[("imp", "😈")];

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

    fn as_str(&self) -> &str {
        let slice = &self.buf[0..self.len];
        str::from_utf8(slice).expect("Invalid")
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

    pub fn handle_key(&mut self, key: u8) -> Option<&str> {
        match (&self.state, key) {
            /*
            '/'                      -> start fresh, set to recording;
            idle                     -> nothing;
            recording, '\n'          -> search dictionary and reset buffer;
            recording, any other key -> add to buffer;
            */
            (_, b'/') => {
                self.buffer.reset();
                self.state = State::Recording;
            }
            (State::Recording, b'\n') => {
                self.state = State::Idle;
                return self.handle_buffer();
            }
            (State::Recording, k) => {
                self.buffer.append(k);
            }
            (State::Idle, _) => {}
        }
        None
    }

    fn handle_buffer(&mut self) -> Option<&str> {
        let typed_word = self.buffer.as_str();

        DICTIONARY
            .binary_search_by_key(&typed_word, |&(key, _)| key)
            .ok()
            .map(|index| DICTIONARY[index].1)
    }
}
