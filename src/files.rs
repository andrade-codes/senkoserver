use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;

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
        // `fs::read_dir` pode falhar, mas não precisamos criar um iterador vazio manualmente.
        if let Ok(entries) = fs::read_dir(&current_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_dir() {
                            stack.push(path.display().to_string());
                        } else if metadata.is_file() {
                            match fs::read(&path) {
                                Ok(content) => {
                                    let hash = calculate_hash(&content);

                                    let relative_path = format!(
                                        "/{}",
                                        path.strip_prefix(dir_path)
                                            .unwrap_or(&path)
                                            .display()
                                            .to_string()
                                    );

                                    files_info.insert(
                                        relative_path.clone(),
                                        FileInfo {
                                            path: relative_path,
                                            hash,
                                            content,
                                        },
                                    );

                                    // println!("Found file: {:?}", path);
                                }
                                Err(err) => {
                                    eprintln!("Failed to read file {:?}: {}", path.display(), err);
                                }
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

fn calculate_hash(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let result = hasher.finalize();
    format!("{:x}", result)
}
