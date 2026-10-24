use custom_cmds::Env;
use rdev::listen;

fn main() {
    let mut env = Env::new();

    if let Err(error) = listen(move |event| {
        if let Some(value) = event.name {
            let key = *value.as_bytes().first().unwrap_or(&b'\0');
            if let Some(emoji) = env.handle_key(key) {
                println!("{}", emoji);
            }
        }
    }) {
        println!("Error: {:?}", error);
    }
}
