use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: String,
    pub hash: String,
    pub content: String,
}

pub fn collect_files_info(dir_path: &str) -> std::io::Result<HashMap<String, FileInfo>> {
    let mut files_info: HashMap<String, FileInfo> = HashMap::new();
    let mut stack = vec![dir_path.to_string()];

    while let Some(current_dir) = stack.pop() {
        for entry in fs::read_dir(&current_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                stack.push(path.display().to_string());
            } else if path.is_file() {
                let content = fs::read_to_string(&path)?;
                let hash = calculate_hash(&content);

                let relative_path = path
                    .display()
                    .to_string()
                    .replace(&format!("{}", dir_path), "");

                files_info.insert(
                    relative_path.clone(),
                    FileInfo {
                        path: relative_path,
                        hash,
                        content,
                    },
                );
            }
        }
    }

    Ok(files_info)
}

fn calculate_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let result = hasher.finalize();
    format!("{:x}", result)
}
