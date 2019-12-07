mod log;
mod reader;
mod writer;
pub mod errors;

use errors::*;

pub use log::{
    Log
};

pub use writer::{
    LoggedWriter
};

pub async fn run() -> Result<()> {
    Log::create("hello").await?;
    Ok(())
}
