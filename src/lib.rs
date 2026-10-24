const DICTIONARY: &[(&[u8], &str)] = &[("imp".as_bytes(), "😈")];
const BACKSPACE: u8 = 8;
const ENTER: u8 = 13;

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
            recording, enter         -> search dictionary and reset buffer;
            recording, backspace     -> decrement buffer length (acts as pop);
            recording, any other key -> add to buffer;
            */
            (_, b'/') => {
                self.buffer.reset();
                self.state = State::Recording;
            }
            (State::Recording, ENTER) => {
                self.state = State::Idle;
                return self.handle_buffer();
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

    fn handle_buffer(&mut self) -> Option<&str> {
        let typed_word = self.buffer.as_bytes();

        DICTIONARY
            .binary_search_by_key(&typed_word, |&(key, _)| key)
            .ok()
            .map(|index| DICTIONARY[index].1)
    }
}
