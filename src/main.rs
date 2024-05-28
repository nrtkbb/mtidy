use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use chrono::prelude::*;
use chrono::offset::TimeZone;
use chrono::LocalResult;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_folder> <output_folder> [move]", args[0]);
        std::process::exit(1);
    }

    let input_folder = &args[1];
    let output_folder = &args[2];
    let move_flag = if args.len() == 4 && &args[3] == "move" { true } else { false };

    process_files(input_folder, output_folder, move_flag);
}

fn process_files(input_folder: &str, output_folder: &str, move_flag: bool) {
    for entry in WalkDir::new(input_folder) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "WAV" || ext == "wav" {
                    let parent_folder = path.parent().unwrap();
                    let timestamp = get_timestamp(path);
                    let new_folder_path = create_new_folder_path(output_folder, timestamp);

                    if !new_folder_path.exists() {
                        fs::create_dir_all(&new_folder_path).unwrap();
                    }

                    process_folder_files(parent_folder, &new_folder_path, move_flag);
                }
            }
        }
    }
}

fn get_timestamp(path: &Path) -> u64 {
    match path.metadata().unwrap().modified() {
        Ok(time) => time.duration_since(UNIX_EPOCH).unwrap().as_secs(),
        Err(_) => 0,
    }
}

fn create_new_folder_path(output_folder: &str, timestamp: u64) -> PathBuf {
    let datetime: DateTime<Utc> = match Utc.timestamp_opt(timestamp as i64, 0) {
        LocalResult::Single(dt) => dt,
        _ => panic!("Invalid timestamp"),
    };
    let new_folder_name = format!("{}", datetime.format("%Y%m%d_%H%M%S"));
    PathBuf::from(output_folder)
        .join(datetime.format("%Y").to_string())
        .join(datetime.format("%m").to_string())
        .join(new_folder_name)
}

fn process_folder_files(parent_folder: &Path, new_folder_path: &Path, move_flag: bool) {
    let mut copied_or_moved = false;
    let mut skipped = false;
    for entry in fs::read_dir(parent_folder).unwrap() {
        let entry = entry.unwrap();
        let src_path = entry.path();
        let dest_path = new_folder_path.join(src_path.file_name().unwrap());

        if dest_path.exists() {
            let src_metadata = src_path.metadata().unwrap();
            let dest_metadata = dest_path.metadata().unwrap();

            if src_metadata.len() > dest_metadata.len() {
                fs::copy(&src_path, &dest_path).unwrap();
                copied_or_moved = true;
            } else if src_metadata.len() == dest_metadata.len() {
                let src_modified = src_metadata.modified().unwrap();
                let dest_modified = dest_metadata.modified().unwrap();
                if src_modified > dest_modified {
                    fs::copy(&src_path, &dest_path).unwrap();
                    copied_or_moved = true;
                } else {
                    skipped = true;
                }
            } else {
                skipped = true;
            }
        } else {
            if move_flag {
                fs::rename(&src_path, &dest_path).unwrap();
            } else {
                fs::copy(&src_path, &dest_path).unwrap();
            }
            copied_or_moved = true;
        }
    }
    if copied_or_moved {
        println!(
            "{} {} to {}",
            if move_flag { "Moved" } else { "Copied" },
            parent_folder.display(),
            new_folder_path.display()
        );
    } else if skipped {
        println!(
            "Skipped {} to {}",
            parent_folder.display(),
            new_folder_path.display()
        );
    }
}