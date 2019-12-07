use std::path::{Path, PathBuf};

use super::errors::*;
use futures::future::{self, FutureExt};
use io::{AsyncRead, AsyncWrite};
use tokio::{fs, io};

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
        fs::create_dir_all(&dir)
            .await
            .chain_err(|| ErrorKind::LogCreateError(
                String::from(dir.as_ref().to_string_lossy()),
                String::from("failed to create directory")
            ))?;
        let mut meta_path = dir.as_ref().to_path_buf();
        meta_path.push("meta.toml");

        let mut f = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&meta_path)
            .await
            .chain_err(|| ErrorKind::LogCreateError(
                String::from(dir.as_ref().to_string_lossy()),
                String::from("failed to create meta file")
            ))?;

        io::copy(&mut "hey there".as_bytes(), &mut f)
            .await
            .chain_err(|| ErrorKind::LogCreateError(
                String::from(dir.as_ref().to_string_lossy()),
                String::from("failed to write meta data")
            ))?;

        Log::open(dir).await
    }

    pub async fn ensure<P>(dir: P) -> Result<Log>
    where
        P: AsRef<Path>,
    {
        Log::create(&dir)
            .await
            .map(move |r| future::ok(r).boxed_local())
            .or_else(|e| match e {
                Error(ErrorKind::LogCreateError(_, _), _) => Ok(Log::open(&dir).boxed_local()),
                err => Err(err),
            })?.await
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
