use std::path::PathBuf;
use std::{fs, io};

pub fn collect(profile_paths: &Vec<PathBuf>) -> Result<Vec<PathBuf>, io::Error> {
    let mut prdr3s = Vec::new();

    for path in profile_paths {
        let entries = fs::read_dir(path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        for file in entries {
            let stem = file.file_stem().unwrap().to_str().unwrap();

            if stem.chars().count() >= 5 {
                let substr = &stem[0..5];

                if substr == "PRDR3" {
                    prdr3s.push(file)
                }
            }
        }
    }

    Ok(prdr3s)
}
