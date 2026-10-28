use serde::Deserialize;
use std::collections::BTreeMap;

const BACKSPACE: u8 = 8;

#[derive(Deserialize)]
struct RawConfig {
    dictionary: BTreeMap<String, String>,
    keybinds: Keybinds,
}

struct Config {
    dictionary: Vec<(&'static [u8], &'static str)>,
    keybinds: Keybinds,
}

#[derive(Deserialize)]
struct Keybinds {
    start_char: char,
    end_char: char,
}

fn load_config() -> Config {
    let path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("config.toml");
    let data = std::fs::read_to_string(&path).unwrap();
    let raw_config = toml::from_str::<RawConfig>(&data).unwrap();

    let mut dictionary = Vec::new();

    for (key, value) in raw_config.dictionary {
        let static_key = key.leak().as_bytes();
        let static_val = value.leak() as &str;

        dictionary.push((static_key, static_val));
    }

    Config {
        dictionary,
        keybinds: raw_config.keybinds,
    }
}

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
    config: Config,
}
impl Default for Env {
    fn default() -> Self {
        Self {
            buffer: Buffer::new(),
            state: State::Idle,
            config: Config {
                dictionary: vec![(b"imp", "😈")],
                keybinds: Keybinds {
                    start_char: '/',
                    end_char: '\\',
                },
            },
        }
    }
}
impl Env {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            state: State::Idle,
            config: load_config(),
        }
    }

    pub fn handle_key(&mut self, key: u8) -> Option<(&'static str, usize)> {
        match (&self.state, key) {
            /*
            recording, end char      -> search dictionary and reset buffer;
            start char               -> start fresh, set to recording;
            idle                     -> nothing;
            recording, backspace     -> decrement buffer length (acts as pop) or set to idle if buffer is empty;
            recording, any other key -> add to buffer;
            */
            (State::Recording, k) if k == self.config.keybinds.end_char as u8 => {
                self.state = State::Idle;
                return self.handle_buffer();
            }
            (_, k) if k == self.config.keybinds.start_char as u8 => {
                self.buffer.reset();
                self.state = State::Recording;
            }
            (State::Recording, BACKSPACE) if self.buffer.is_empty() => {
                self.state = State::Idle;
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

        self.config
            .dictionary
            .binary_search_by_key(&typed_word, |&(key, _)| key)
            .ok()
            .map(|index| (self.config.dictionary[index].1, typed_word.len() + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_sequence(keys: &[u8], expected_end: Option<(&str, usize)>) {
        let mut env = Env::default();
        for i in 0..keys.len() - 1 {
            assert_eq!(env.handle_key(keys[i]), None);
        }
        assert_eq!(env.handle_key(keys[keys.len() - 1]), expected_end);
    }

    #[test]
    fn test_imp_binary_input_4_none_1_some() {
        assert_sequence(b"/imp\\", Some(("😈", 4)));
    }

    #[test]
    fn test_imp_binary_input_with_backspace() {
        assert_sequence(
            &[b'/', b'i', b'n', BACKSPACE, b'm', b'p', b'\\'],
            Some(("😈", 4)),
        );
    }

    #[test]
    fn test_imp_binary_input_with_underflow_prevention() {
        assert_sequence(
            &[
                BACKSPACE, BACKSPACE, BACKSPACE, BACKSPACE, BACKSPACE, b'/', b'i', b'm', b'p',
                b'\\',
            ],
            Some(("😈", 4)),
        );
    }

    #[test]
    fn test_recording_state_persistence_when_slash_erased() {
        assert_sequence(&[b'/', BACKSPACE, b'i', b'm', b'p', b'\\'], None);
    }

    #[test]
    fn test_recording_state_persistence_when_slash_erased_multiple() {
        assert_sequence(
            &[
                b'/', b'i', b'm', BACKSPACE, BACKSPACE, BACKSPACE, b'i', b'm', b'p', b'\\',
            ],
            None,
        );
    }
}
