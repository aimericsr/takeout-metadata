use chrono::DateTime;
use clap::{arg, command, Parser};
use log::{error, info};
use serde::Deserialize;
use std::{
    ffi::{OsStr, OsString},
    fs::{self, File, FileTimes},
    path::PathBuf,
    time::SystemTime,
};

/// Re-order export of Google Takeout photos with correct date stored in .jpg.json files
#[derive(Parser, Debug)]
#[command(name = "TakeoutReorder")]
#[command(version = "0.0.1")]
#[command(about, long_about = None)]
struct Args {
    /// Path to folder to search for photos
    #[arg(short, long, value_parser = valid_path)]
    folder: String,
}

fn valid_path(s: &str) -> Result<String, String> {
    Ok(s.into())
}

pub fn lunch() {
    let args = Args::parse();

    let dir = fs::read_dir(args.folder.clone()).expect("failed to read filesystem");

    for path in dir {
        let current_path = path.expect("failed to read filesystem").path();

        if current_path.is_file() {
            if let Some(extension) = current_path.extension() {
                if extension == OsStr::new("jpg") || extension == OsStr::new("mp4") {
                    info!("current file : {:?}", current_path);

                    let file_to_search =
                        OsString::from(format!("{}.json", current_path.to_str().unwrap()));

                    info!("file to search : {:?}", file_to_search);

                    let mut dir =
                        fs::read_dir(args.folder.clone()).expect("failed to read filesystem");
                    let metadata_file = dir.find(|file| {
                        file.as_ref()
                            .unwrap()
                            .path()
                            .to_str()
                            .unwrap()
                            .contains(file_to_search.to_str().unwrap())
                    });

                    if let Some(metadata) = metadata_file {
                        let metadata_file = metadata
                            .expect("An intermittent IO error has been encountered during process")
                            .path();
                        info!(
                            "metadata file found : {:?} for {:?}",
                            metadata_file, current_path
                        );
                        let data = fs::read_to_string(metadata_file)
                            .expect("error reading the metadata file");

                        let updated_date = parse_json_metadata(data);

                        change_photo_metadata(current_path, updated_date);
                    } else {
                        error!("metadata file not found for {:?}", current_path);
                    }
                }
            }
        }
    }
}

fn change_photo_metadata(file: PathBuf, date: String) {
    let timestamp = date.parse::<i64>().expect("Failed to parse date");
    let date: SystemTime = DateTime::from_timestamp(timestamp, 0)
        .expect("Failed to convert date")
        .into();

    let dest = File::options().write(true).open(file.as_path()).unwrap();

    let times = FileTimes::new().set_accessed(date).set_modified(date);
    dest.set_times(times).unwrap();
}

fn parse_json_metadata(data: String) -> String {
    let photo_data: PhotoData =
        serde_json::from_str(&data).expect("error parsing the metadata file");
    photo_data.photo_taken_time.timestamp
}

#[derive(Debug, Deserialize)]
struct PhotoData {
    #[serde(rename(deserialize = "photoTakenTime"))]
    photo_taken_time: TimeData,
}

#[derive(Debug, Deserialize)]
struct TimeData {
    timestamp: String,
}
