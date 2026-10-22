use custom_cmds::Env;

fn main() {
    let key_presses: Vec<u8> = vec![
        b'p', b'/', b'i', b'm', b'p', b'o', b'/', b'i', b'm', b'p', b'\n', b'o', b'/', b'i', b'm',
        b'p', b'p', b'\n',
    ];
    let mut env = Env::new();
    for key in key_presses {
        if let Some(value) = env.handle_key(key) {
            println!("{}", value);
        }
    }
}
