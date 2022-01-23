use embed_resource;
use std::{env, path::Path};

fn main() {
  let curr_dir = env::current_dir().unwrap();
  embed_resource::compile(Path::new(&curr_dir).join("build").join("assets").join("icon.rc"));
}