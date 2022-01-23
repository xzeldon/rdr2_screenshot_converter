use colored::*;
use dirs;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use core::files;
use core::{buffer, meta::ImageMetadata};

struct Image {
    metadata: ImageMetadata,
    image_data: Vec<u8>,
}

fn convert_file(path: PathBuf) -> Result<Image, io::Error> {
    let file = buffer::read_file(&path).unwrap();
    let buf = buffer::trim(&file);

    let image_data = buffer::parse_buf(&buf, b"JPEG", b"JSON", 12);
    let metadata = buffer::parse_buf(&buf, b"JSON", b"TITL", 8);
    let parsed_metadata: ImageMetadata = serde_json::from_slice(&metadata).unwrap();

    Ok(Image {
        metadata: parsed_metadata,
        image_data,
    })
}

pub fn run(config: Config) -> Result<(), String> {
    let mut profiles = Vec::new();
    let paths = fs::read_dir(config.profile_path).unwrap();

    for path in paths {
        profiles.push(path.unwrap().path());
    }

    if profiles.len() <= 0 {
        return Err(format!(
            "{}: {} {} \n{}",
            "> Error".red().bold(),
            "Profile folder not found.".white(),
            "You must run the game at least once!".white().bold(),
            "> Aborting.".red()
        ));
    }

    let scrns = files::collect(&profiles).unwrap();

    if scrns.len() <= 0 {
        return Err(format!(
            "{}: {} \n{}",
            "> Error".red().bold(),
            "Screenshot files not found".white(),
            "> Aborting.".red()
        ));
    }

    println!(
        "{} {} {} \n{}",
        ">".green(),
        scrns.len().to_string().green(),
        "Screenshots found.".green(),
        "> Converting...".green().bold()
    );

    let mut images = Vec::new();

    for file in &scrns {
        let image = convert_file(file.to_path_buf()).unwrap();
        images.push(image);
    }

    for image in &images {
        match fs::write(
            Path::new(&config.export_path).join(image.metadata.uid.to_string() + ".jpg"),
            &image.image_data,
        ) {
            Ok(ok) => ok,
            Err(_err) => {
                return Err(format!(
                    "{}: {} \n{}",
                    "> Error".red().bold(),
                    "System cannot find or create the specified path",
                    "> Aborting.".red()
                ))
            }
        }
    }

    println!("{}", "> Converting completed!".green());
    println!(
        "{} {}",
        "> Files saved to".green().bold(),
        Path::new(&config.export_path)
            .to_string_lossy()
            .bright_blue()
    );

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
        let profile_path = Path::new(&home_dir)
            .join("Documents")
            .join("Rockstar Games")
            .join("Red Dead Redemption 2")
            .join("Profiles");
        let export_path = Path::new(&home_dir)
            .join("Documents")
            .join("Rockstar Games")
            .join("Red Dead Redemption 2")
            .join("Screenshots");

        let _create_dirs = fs::create_dir_all(&export_path);

        Ok(Config {
            profile_path,
            export_path,
        })
    }

    pub fn set_custom_export_path<'a>(
        &'a mut self,
        path: &[String],
    ) -> Result<&'a mut Config, String> {
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
