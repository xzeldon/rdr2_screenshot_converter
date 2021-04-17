fn is_not_empty(e: &u8) -> bool {
  *e != 0x00
}

pub fn trim(buffer: &[u8]) -> Vec<u8> {
  let begin = buffer.iter().position(is_not_empty);
  let end = buffer.iter().rev().position(is_not_empty).map(|j| buffer.len() - j);
  let vec = begin.and_then(|i| end.map(|j| buffer[i..j].iter().cloned().collect())).unwrap_or(Vec::new());
  vec
}