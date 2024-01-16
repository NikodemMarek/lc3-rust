use std::fs::File;
use std::io::{self, Read};

pub fn run() {
    load();
}

fn load() {
    let prog = read_binary_file("2048.obj").unwrap();
    dbg!(prog);
}

fn read_binary_file(file_path: &str) -> io::Result<Vec<u16>> {
    let mut file = File::open(file_path)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let arr = buffer.chunks(2).map(|chunk| {
        let mut bytes = [0; 2];
        bytes.copy_from_slice(chunk);
        u16::from_le_bytes(bytes)
    }).collect::<Vec<_>>();

    Ok(arr)
}
