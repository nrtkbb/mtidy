use std::fs;
use std::fmt;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use fs_extra::dir::get_size;
use chrono::prelude::{DateTime, Local};
use thiserror::Error;
use walkdir::WalkDir;

type Result<T> = std::result::Result<T, CustomError>;

#[derive(Debug, Error)]
enum CustomError {
    #[error("walkdir error")]
    WalkDirError,
}

#[derive(Debug)]
struct MovieFolder {
    path: PathBuf,
    size: u64,
    m_time: DateTime<Local>,
}

impl fmt::Display for MovieFolder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.path.display(), self.size, self.m_time)
    }
}

fn get_movie_folders(input_path: &Path) -> Result<Vec<MovieFolder>> {
    let mut movie_folders = vec![];
    for entry in WalkDir::new(input_path) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_e) => {
                return Err(CustomError::WalkDirError);
           }
        };
        if entry.file_type().is_dir() {
            continue;
        }
        let entry_extension = match entry.path().extension() {
            Some(e) => e,
            None => {
                continue;
            }
        };
        if entry_extension == "wav" || entry_extension == "WAV" {
            let entry_path = entry.path();
            let movie_path = if let Some(movie_path) = entry_path.parent() {
                movie_path
            } else {
                panic!("Not found parent path for {}", entry_path.display());
            };
            let movie_size = if let Ok(movie_size) = get_size(&movie_path) {
                movie_size
            } else {
                panic!("fs_extra::dir::get_size for {}", movie_path.display());
            };
            let entry_meta = if let Ok(entry_meta) = fs::metadata(&entry.path()) {
                entry_meta
            } else {
                panic!("Get metadata for {}", entry_path.display());
            };
            let entry_mtime = if let Ok(entry_mtime) = entry_meta.modified() {
                entry_mtime
            } else {
                panic!("Get modified time for {}", entry_path.display());
            };
            let movie_folder = MovieFolder {
                path: movie_path.to_path_buf(),
                size: movie_size,
                m_time: entry_mtime.into(),

            };
            movie_folders.push(movie_folder);
        }
    }
    Ok(movie_folders)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("使用法: {} <入力ファイルパス> <出力ファイルパス>", args[0]);
        return;
    }

    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);

    if !input_path.exists() {
        println!("入力ファイルパスが存在しません。");
        return;
    }

    let movie_folders = match get_movie_folders(input_path) {
        Ok(m) => m,
        Err(e) => panic!("{:?}", e),
    };
    for movie_folder in movie_folders {
        let mk_dir = output_path.join(format!(
            "{}/{}/{}",
            movie_folder.m_time.format("%Y"),
            movie_folder.m_time.format("%m"),
            movie_folder.m_time.format("%d")
        ));
        if !mk_dir.exists() {
            let mk_status = Command::new("mkdir")
                .arg("-p")
                .arg(&mk_dir)
                .status()
                .expect("mkdirコマンドの実行に失敗しました。");
            if !mk_status.success() {
                panic!("mkdirコマンドの実行に失敗したため終了します");
            }
        }
        let cp_path = output_path.join(format!(
            "{}/{}/{}/{}",
            movie_folder.m_time.format("%Y"),
            movie_folder.m_time.format("%m"),
            movie_folder.m_time.format("%d"),
            movie_folder.m_time.format("%Y%m%d_%H%M%S")
        ));
        println!("{} to {}", movie_folder, cp_path.display());

        let status = Command::new("cp")
            .arg("-rp")
            .arg(&movie_folder.path)
            .arg(&cp_path)
            .status()
            .expect("cp コマンドの実行に失敗しました。");

        if !status.success() {
            panic!("cpコマンドの実行に失敗したため終了します")
        }

        let cp_size = if let Ok(cp_size) = get_size(&cp_path) {
            cp_size
        } else {
            panic!("fs_extra::dir::get_size for {}", cp_path.display());
        };
        if cp_size != movie_folder.size {
            panic!("source:{}, to:{} was not eq. cp_path:{}",
                movie_folder.size, cp_size, cp_path.display()
            );
        }
        println!("ok! {} to {}", movie_folder, cp_path.display());
    }
}