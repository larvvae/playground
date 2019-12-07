use std::path::{
    Path,
    PathBuf
};
use tokio::fs;

use super::log::Log;

/// Streaming object writer that uses a separate log file for offset data
pub struct LoggedWriter<L> where L: AsRef<Log> {
    /// Directory containing object and log file
    dir: PathBuf,

    /// access to the log
    log: L,

    /// Current object file
    cur_file: fs::File,

    /// Current log file
    cur_log: fs::File
}

// impl LoggedWriter {
    // pub async fn create<P>(path: P, log: &) -> LoggedWriter
    // where P: AsRef<Path> {
    //     LoggedWriter {
    //         dir: path.as_ref().to_path_buf(),
    //         opts,
    //     }
    // }
// }
