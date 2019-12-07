use std::path::{Path, PathBuf};
use std::result;

use std::io;
use tokio::fs;
use super::errors::*;

pub struct Log {
    /// path to this log directory
    dir: PathBuf,

    /// path to meta file
    meta: PathBuf,

    /// paths to options files, ordered from newest to oldest
    opts_paths: Vec<PathBuf>,

    /// path to current object file, ordered from newest to oldest
    obj_paths: Vec<PathBuf>,

    /// path to current log file, ordered from newest to oldest
    log_paths: Vec<PathBuf>,
}

impl Log {
    pub async fn create<P>(dir: P) -> Result<Log>
    where
        P: AsRef<Path>,
    {
        fs::create_dir_all(&dir).await.chain_err(|| ErrorKind::LogCreateFailed)?;
        let mut meta_path = dir.as_ref().to_path_buf();
        meta_path.push("meta.toml");

        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&meta_path)
            .await
            .map_err(|e| Error::with_chain(e, ErrorKind::LogAlreadyExists))?;

        Log::open(dir).await
    }

    pub async fn open<P>(dir: P) -> Result<Log>
    where
        P: AsRef<Path>,
    {
        let mut meta_path = dir.as_ref().to_path_buf();
        meta_path.push("meta.toml");

        Ok(Log {
            dir: dir.as_ref().to_path_buf(),
            meta: meta_path,
            opts_paths: vec![],
            obj_paths: vec![],
            log_paths: vec![],
        })
    }
}
