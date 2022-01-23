use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

pub fn is_not_empty(e: &u8) -> bool {
    *e != 0x00
}

pub fn trim(buffer: &Vec<u8>) -> Vec<u8> {
    let begin = buffer.iter().position(is_not_empty);
    let end = buffer
        .iter()
        .rev()
        .position(is_not_empty)
        .map(|j| buffer.len() - j);
    let vec = begin
        .and_then(|i| end.map(|j| buffer[i..j].iter().cloned().collect()))
        .unwrap_or(Vec::new());
    vec
}

pub fn read_file(path: &PathBuf) -> Result<Vec<u8>, io::Error> {
    let mut data = Vec::new();
    File::open(path)?.read_to_end(&mut data)?;
    return Ok(data);
}

pub fn find_index(buffer: &Vec<u8>, index: &[u8]) -> usize {
    let index = buffer
        .windows(index.len())
        .position(|window| window == index)
        .unwrap();
    index
}

pub fn parse_buf(
    buffer: &Vec<u8>,
    first_index: &[u8],
    last_index: &[u8],
    offset: usize,
) -> Vec<u8> {
    let first = find_index(&buffer, first_index);
    let last = find_index(&buffer, last_index);
    return trim(&buffer[first + offset..last].to_vec());
}
