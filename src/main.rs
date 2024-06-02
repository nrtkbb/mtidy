use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use filetime::{FileTime, set_file_times};

use chrono::offset::TimeZone;
use chrono::LocalResult;
use chrono::Local;
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
    let datetime = match Local.timestamp_opt(timestamp as i64, 0) {
        LocalResult::Single(dt) => dt,
        _ => panic!("Invalid timestamp"),
    };
    let new_folder_name = format!("{}", datetime.format("%Y%m%d_%H%M%S"));
    PathBuf::from(output_folder)
        .join(datetime.format("%Y").to_string())
        .join(datetime.format("%m").to_string())
        .join(datetime.format("%d").to_string()) 
        .join(new_folder_name)
}

fn process_folder_files(parent_folder: &Path, new_folder_path: &Path, move_flag: bool) {
    let copied_files = match copy_files(parent_folder, new_folder_path) {
        Ok(copied_files) => copied_files,
        Err(_) => return, 
    };
    
    // Check if all files are copied correctly
    let all_copied = check_all_files(&copied_files);
    
    if all_copied {
        if move_flag {
            for (src_path, _, copied) in copied_files {
                if !copied {
                    continue;
                }
                if let Err(e) = fs::remove_file(&src_path) {
                    eprintln!("Error removing file {}: {}", src_path.display(), e);
                }
            }
        }
        println!(
            "{} {} to {}",
            if move_flag { "Moved" } else { "Copied" },
            parent_folder.display(),
            new_folder_path.display()
        );
    } else {
        println!(
            "Skipped {} to {} due to copy errors",
            parent_folder.display(),
            new_folder_path.display()
        );
    }
}

fn get_file_timestamps(file_path: &Path) -> io::Result<(FileTime, FileTime)> {
    let metadata = match fs::metadata(file_path) {
        Ok(metadata) => metadata,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("Failed to get metadata for file {}: {}", file_path.display(), e))),
    };

    let modified = FileTime::from_last_modification_time(&metadata);

    let created = match FileTime::from_creation_time(&metadata) {
        Some(created_time) => created_time,
        None => return Err(io::Error::new(io::ErrorKind::Other, format!("Creation time not available for file {}", file_path.display()))),
    };

    Ok((created, modified))
}

fn copy_file(src_path: &Path, dest_path: &Path) -> io::Result<()> {
    let (src_created, src_modified) = get_file_timestamps(&src_path)?;

    fs::copy(&src_path, &dest_path)?;

    set_file_times(&dest_path, src_created, src_modified)
        .map_err(|e| {
            match e.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    io::Error::new(io::ErrorKind::PermissionDenied, format!("Failed to set creation time for {}: {}", dest_path.display(), e))
                },
                _ => {
                    io::Error::new(io::ErrorKind::Other, format!("Failed to set modification time for {}: {}", dest_path.display(), e))
                },
            }
        })?;

    Ok(())
}

fn copy_files(parent_folder: &Path, new_folder_path: &Path) -> io::Result<Vec<(PathBuf, PathBuf, bool)>> {
    let mut copied_files = Vec::new();
    for entry in fs::read_dir(parent_folder)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = new_folder_path.join(src_path.file_name().unwrap());

        let copied = if dest_path.exists() {
            let (_, src_modified) = match get_file_timestamps(&src_path) {
                Ok(timestamps) => timestamps,
                Err(e) => {
                    eprintln!("Failed to get timestamps for source file {}: {}", src_path.display(), e);
                    continue;
                },
            };

            let (_, dest_modified) = match get_file_timestamps(&dest_path) {
                Ok(timestamps) => timestamps,
                Err(e) => {
                    eprintln!("Failed to get timestamps for destination file {}: {}", dest_path.display(), e);
                    continue;
                },
            };

            if src_modified > dest_modified {
                match copy_file(&src_path, &dest_path) {
                    Ok(()) => {
                        println!("{} - Copied file from {} to {}", Local::now().format("%Y-%m-%d %H:%M:%S"), src_path.display(), dest_path.display());
                        true
                    },
                    Err(e) => {
                        eprintln!("Error copying file {} to {}: {}", src_path.display(), dest_path.display(), e);
                        false
                    },
                }
            } else {
                false
            }
        } else {
            match copy_file(&src_path, &dest_path) {
                Ok(()) => {
                    println!("{} - Copied file from {} to {}", Local::now().format("%Y-%m-%d %H:%M:%S"), src_path.display(), dest_path.display());
                    true
                },
                Err(e) => {
                    eprintln!("Error copying file {} to {}: {}", src_path.display(), dest_path.display(), e);
                    false
                },
            }
        };

        copied_files.push((src_path, dest_path, copied));
    }
    Ok(copied_files)
}


fn check_all_files(copied_files: &Vec<(PathBuf, PathBuf, bool)>) -> bool {
    for (src_path, dest_path, _copied) in copied_files {
        if let Ok(src_metadata) = src_path.metadata() {
            if let Ok(dest_metadata) = dest_path.metadata() {
                if src_metadata.len() != dest_metadata.len() {
                    eprintln!("File size mismatch for {} and {}", src_path.display(), dest_path.display());
                    return false;
                }
            } else {
                eprintln!("Failed to get metadata for {}", dest_path.display());
                return false;
            }
        } else {
            eprintln!("Failed to get metadata for {}", src_path.display());
            return false;
        }
    }
    true
}