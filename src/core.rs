use std::{path::PathBuf, time::SystemTime};

use chrono::{DateTime, Local};
use walkdir::WalkDir;

pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub modified: SystemTime,
}

pub fn search(name: &str, path: &str, depth: u8, show_debug_info: bool, exact_match: bool) {
    let file_list = walk(path, depth);

    if show_debug_info {
        print_debug_info(&file_list);
    }

    for info in &file_list {
        let mut file_name = match info.path.file_name() {
            Some(os_str) => String::from(os_str.to_string_lossy()),
            None => ".".to_owned(),
        };
        let file_size = bytes_to_human_readable(info.size);
        let datetime: DateTime<Local> = info.modified.into();
        let file_modified = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        if file_name.len() > 48 {
            file_name.truncate(48);
            file_name.push_str("...");
        }

        if name.is_empty()
            || (exact_match && file_name == name)
            || (!exact_match && file_name.contains(name))
        {
            print!("{:54}\t{:10}\t{}\n", file_name, file_size, file_modified);
        }
    }
}

fn walk(path: &str, depth: u8) -> Vec<FileInfo> {
    let mut file_list = Vec::new();

    for entry in WalkDir::new(path)
        .max_depth(if depth == 0 { usize::MAX } else { depth.into() })
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let metadata = entry.metadata().unwrap();
        let file_info = FileInfo {
            path: entry.path().to_path_buf(),
            size: metadata.len(),
            modified: metadata.modified().unwrap(),
        };
        file_list.push(file_info);
    }

    file_list
}

fn print_debug_info(file_list: &Vec<FileInfo>) {
    for file_info in file_list {
        println!("Path: {:?}", file_info.path.display());
        println!("Size: {}", file_info.size);
        println!("Modified: {:?}", file_info.modified);
        println!();
    }
}

fn bytes_to_human_readable(size: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = KIB * 1024;
    const GIB: u64 = MIB * 1024;

    if size >= GIB {
        format!("{:.2} GiB", size as f64 / GIB as f64)
    } else if size >= MIB {
        format!("{:.2} MiB", size as f64 / MIB as f64)
    } else if size >= KIB {
        format!("{:.2} KiB", size as f64 / KIB as f64)
    } else {
        format!("{} B", size)
    }
}
