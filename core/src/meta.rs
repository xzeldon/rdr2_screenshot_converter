use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Loc {
    x: f64,
    y: f64,
    z: f64,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Time {
    hour: i32,
    minute: i32,
    second: i32,
    day: i32,
    month: i32,
    year: i32,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]

pub struct Meta {
    horse: Option<Vec<i32>>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]

pub struct ImageMetadata {
    pub loc: Loc,
    pub regionname: usize,
    pub districtname: usize,
    pub statename: usize,
    pub nm: String,
    pub sid: String,
    pub crewid: usize,
    pub mid: String,
    pub mode: String,
    pub meme: bool,
    pub mug: bool,
    pub uid: usize,
    pub time: Time,
    pub creat: usize,
    pub slf: bool,
    pub drctr: bool,
    pub rsedtr: bool,
    pub inphotomode: bool,
    pub advanced: bool,
    pub width: usize,
    pub height: usize,
    pub size: usize,
    pub sign: usize,
    pub meta: Meta,
}
