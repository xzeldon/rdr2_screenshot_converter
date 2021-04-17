mod buffertrim;

use dirs;
use colored::*;
use serde::Deserialize;
use fs::File;
use io::Read;
use std::{fs, io, path::{Path, PathBuf}};

pub fn run(config: Config) -> Result<(), String> {

  let mut profiles = Vec::new();
  let paths = fs::read_dir(config.profile_path).unwrap();

  for path in paths {
    profiles.push(path.unwrap().path());
  }

  if profiles.len() <= 0 {
    return Err(format!("{}: {} {} \n{}", "> Error".red().bold(), "Profile folder not found.".white(), "You must run the game at least once!".white().bold(), "> Aborting.".red()));
  }

  let scrns = grab_scrn_files(&profiles);

  if scrns.len() <= 0 {
    return Err(format!("{}: {} \n{}", "> Error".red().bold(), "Screenshot files not found".white(), "> Aborting.".red()));
  }

  println!("{} {} {} \n{}", ">".green(), scrns.len().to_string().green(), "Screenshots found.".green(), "> Converting...".green().bold());

  let mut images = Vec::new();

  for file in &scrns {
    let image = convert_file(file.to_path_buf()).unwrap();
    images.push(image);
  }

  for image in &images {
    match fs::write(Path::new(&config.export_path).join(image.metadata.uid.to_string() + ".jpg"), &image.image_data) {
        Ok(ok) => ok,
        Err(_err) => {
          return Err(format!("{}: {} \n{}", "> Error".red().bold(), "System cannot find or create the specified path", "> Aborting.".red()))
        }
    }
  }

  println!("{}", "> Converting completed!".green());
  println!("{} {}", "> Files saved to".green().bold(), Path::new(&config.export_path).to_string_lossy().bright_blue());

  println!("{}", "> Removing old files...".green());
  for path in scrns {
    let _remove = fs::remove_file(path).unwrap();
  }

  Ok(())
}

pub struct Config {
  pub profile_path: PathBuf,
  pub export_path: PathBuf,
}

impl Config {
  pub fn new() -> Result<Config, io::Error> {
    let home_dir = dirs::home_dir().unwrap();
    let profile_path = Path::new(&home_dir).join("Documents").join("Rockstar Games").join("Red Dead Redemption 2").join("Profiles");
    let export_path = Path::new(&home_dir).join("Documents").join("Rockstar Games").join("Red Dead Redemption 2").join("Screenshots");

    let _create_dirs = fs::create_dir_all(&export_path);

    Ok(Config {profile_path, export_path})
  }

  pub fn set_custom_export_path<'a>(&'a mut self, path: &[String]) -> Result<&'a mut Config, String> {
    if path.len() < 2 {
      return Err(format!("{}", "Not enough arguments"));
    }

    let path = path[1].clone();
    let custom_export_path = Path::new(&path).to_path_buf();
    // TODO: Folders containing Cyrillic characters and spaces are created incorrectly i. e. anything after a space is ignored
    // let _create_dirs = fs::create_dir_all(&custom_export_path);

    self.export_path = custom_export_path;

    Ok(self)
  }
}

fn grab_scrn_files(profile_paths: &[PathBuf]) -> Vec<PathBuf> {
  let mut prdr3s = Vec::new();

  for path in profile_paths {
    match fs::read_dir(path) {
      Err(err) => println!("! {:?}", err.kind()),
      Ok(paths) => for path in paths {
        let file_path = path.unwrap().path();
        let file_stem = file_path.file_stem().unwrap().to_str().unwrap();

        if file_stem.chars().count() >= 5 {
          let file_stem_substr = &file_stem[0..5];

          if file_stem_substr == "PRDR3" {
            prdr3s.push(file_path)
          }
        }
      }
    }
  }
  prdr3s
}

fn read_file_buf(path: PathBuf) -> Result<Vec<u8>, io::Error> {
  let mut file = File::open(path)?;
  let mut data = Vec::new();
  file.read_to_end(&mut data)?;
  
  return Ok(data);
}

fn find_index_in_buf(buffer: &Vec<u8>, index: &[u8]) -> usize {
  let index = buffer.windows(index.len()).position(|window| window == index).unwrap();
  index
}

fn grab_image_from_buf(buffer: &Vec<u8>) -> Vec<u8> {
  let first_index = find_index_in_buf(&buffer, b"JPEG");
  let last_index = find_index_in_buf(&buffer, b"JSON");
  let image_data = buffertrim::trim(&buffer[first_index + 12..last_index]);
  image_data
}

fn grab_metadata_from_buf(buffer: &Vec<u8>) -> Vec<u8> {
  let first_index = find_index_in_buf(&buffer, b"JSON");
  let last_index = find_index_in_buf(&buffer, b"TITL");
  let metadata = buffertrim::trim(&buffer[first_index + 8..last_index]);
  metadata
}

#[derive(Deserialize, Debug)]
struct Loc {
  x: f64,
  y: f64,
  z: f64
}

#[derive(Deserialize, Debug)]
struct Time {
  hour: i32,
  minute: i32,
  second: i32,
  day: i32,
  month: i32,
  year: i32
}

#[derive(Deserialize, Debug)]
struct Meta {
  horse: Option<Vec<i32>>
}

#[derive(Deserialize, Debug)]
struct ImageMetadata {
  loc: Loc,
  regionname: usize,
  districtname: usize,
  statename: usize,
  nm: String,
  sid: String,
  crewid: usize,
  mid: String,
  mode: String,
  meme: bool,
  mug: bool,
  uid: usize,
  time: Time,
  creat: usize,
  slf: bool,
  drctr: bool,
  rsedtr: bool,
  inphotomode: bool,
  advanced: bool,
  width: usize,
  height: usize,
  size: usize,
  sign: usize,
  meta: Meta
}

#[derive(Deserialize, Debug)]
struct Image {
  metadata: ImageMetadata,
  image_data: Vec<u8>
}

fn convert_file(path: PathBuf) -> Result<Image, io::Error> {
  let mut file_buf = read_file_buf(path).unwrap();
  file_buf = buffertrim::trim(&file_buf);

  let image_data = grab_image_from_buf(&file_buf);
  let metadata = grab_metadata_from_buf(&file_buf);
  let parsed_metadata: ImageMetadata = serde_json::from_slice(&metadata).unwrap();

  Ok(Image { metadata: parsed_metadata, image_data })
}