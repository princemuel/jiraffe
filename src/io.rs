use std::io;

pub fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer
}

pub fn pause() { io::stdin().read_line(&mut String::new()).unwrap(); }
