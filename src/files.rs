use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::mpsc as std_mpsc;
use std::thread;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: String,
    pub hash: String,
    pub content: Vec<u8>,
}

pub fn collect_files_info(dir_path: &str) -> std::io::Result<HashMap<String, FileInfo>> {
    let mut files_info: HashMap<String, FileInfo> = HashMap::new();
    let mut stack = vec![dir_path.to_string()];

    while let Some(current_dir) = stack.pop() {
        if let Ok(entries) = fs::read_dir(&current_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_dir() {
                            stack.push(path.display().to_string());
                        } else if metadata.is_file() {
                            if let Some(file_info) = get_file_info(&path, dir_path) {
                                // println!("Loaded file: {}", file_info.path);
                                files_info.insert(file_info.path.clone(), file_info);
                            }
                        }
                    }
                }
            }
        } else {
            eprintln!("Failed to read directory: {}", current_dir);
        }
    }

    Ok(files_info)
}

fn get_file_info(path: &Path, dir_path: &str) -> Option<FileInfo> {
    if let Ok(content) = fs::read(path) {
        let hash = calculate_hash(&content);
        let relative_path = format!(
            "/{}",
            path.strip_prefix(dir_path)
                .unwrap_or(path)
                .display()
                .to_string()
        );
        Some(FileInfo {
            path: relative_path,
            hash,
            content,
        })
    } else {
        None
    }
}

pub fn watch_files(
    dir_path: &str,
    on_change: impl Fn(&HashMap<String, FileInfo>) + Send + Sync + 'static,
) -> notify::Result<RecommendedWatcher> {
    // println!("Starting file watcher for directory: {}", dir_path);

    let dir_path = dir_path.to_string();
    let (tx, rx) = std_mpsc::channel();

    let mut watcher: RecommendedWatcher = RecommendedWatcher::new(
        move |res| {
            tx.send(res).unwrap();
        },
        Config::default(),
    )?;

    watcher.watch(Path::new(&dir_path), RecursiveMode::Recursive)?;

    thread::spawn(move || {
        let mut files_info = collect_files_info(&dir_path).unwrap_or_default();

        for res in rx {
            match res {
                Ok(event) => {
                    println!("Event detected: {:?}", event);

                    match event.kind {
                        EventKind::Create(_) | EventKind::Modify(_) => {
                            if let Some(file_path) = event.paths.first() {
                                println!("File changed: {:?}", file_path);
                                if let Some(file_info) = get_file_info(file_path, &dir_path) {
                                    files_info.insert(file_info.path.clone(), file_info);
                                }
                            }
                        }
                        EventKind::Remove(_) => {
                            if let Some(file_path) = event.paths.first() {
                                println!("File removed: {:?}", file_path);
                                if let Ok(relative_path) = file_path.strip_prefix(&dir_path) {
                                    let relative_path = format!("/{}", relative_path.display());
                                    files_info.remove(&relative_path);
                                }
                            }
                        }
                        _ => {}
                    }

                    println!("Updated files_info: {:?}", files_info.keys());
                    on_change(&files_info);
                }
                Err(e) => {
                    println!("Watch error: {:?}", e);
                }
            }
        }
    });

    Ok(watcher)
}

fn calculate_hash(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let result = hasher.finalize();
    format!("{:x}", result)
}
