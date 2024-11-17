use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tokio::sync::mpsc::{self, Receiver};

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: String,
    pub hash: String,
    pub content: Vec<u8>,
}

// Função para coletar informações de todos os arquivos inicialmente
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

// Função para obter informações de um único arquivo
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

// Função para monitorar mudanças no sistema de arquivos
pub async fn watch_files(
    dir_path: &str,
    on_change: impl Fn(HashMap<String, FileInfo>) + Send + Sync + 'static,
) -> notify::Result<()> {
    // Copia o valor de `dir_path` para evitar problemas de lifetime
    let dir_path = dir_path.to_string();

    // Canal para eventos
    let (tx, mut rx) = mpsc::channel(100);

    // Configuração do watcher
    let mut watcher: RecommendedWatcher = Watcher::new(
        move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event); // Envia o evento para o canal
            }
        },
        Config::default(),
    )?;

    watcher.watch(Path::new(&dir_path), RecursiveMode::Recursive)?;

    // Processa eventos em tempo real
    let mut files_info = collect_files_info(&dir_path).unwrap_or_default();

    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event.kind {
                EventKind::Create(_) | EventKind::Modify(_) => {
                    if let Some(file_path) = event.paths.first() {
                        if let Some(file_info) = get_file_info(file_path, &dir_path) {
                            files_info.insert(file_info.path.clone(), file_info);
                        }
                    }
                }
                EventKind::Remove(_) => {
                    if let Some(file_path) = event.paths.first() {
                        if let Ok(relative_path) = file_path.strip_prefix(&dir_path) {
                            let relative_path = format!("/{}", relative_path.display());
                            files_info.remove(&relative_path);
                        }
                    }
                }
                _ => {}
            }
            on_change(files_info.clone());
        }
    });

    Ok(())
}

// Função para calcular o hash do conteúdo do arquivo
fn calculate_hash(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let result = hasher.finalize();
    format!("{:x}", result)
}
