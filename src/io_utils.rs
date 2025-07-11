use std::io;

pub fn get_input() -> String {
    let mut buffer = String::with_capacity(64);
    io::stdin().read_line(&mut buffer).unwrap();
    buffer
}
