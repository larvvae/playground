// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate slog;

#[macro_use]
extern crate clap;

// Import the macro. Don't forget to add `error-chain` in your
// `Cargo.toml`!
#[macro_use]
extern crate error_chain;

mod errors;
mod f2s_copy;
mod kf_log;
mod raft_multi;
mod tk_echo;

use futures::stream::StreamExt;
use tokio::net::TcpListener;
use tokio::prelude::*;

#[derive(Clap)]
#[clap(version = "v1.0-beta")]
struct Opts {
    main: String,
}

use errors::*;

#[tokio::main]
async fn main() {
    if let Err(ref e) = run().await {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    } else {
        println!("no error");
    }
}

async fn run() -> Result<()> {
    let opts = Opts::parse();

    match opts.main.as_ref() {
        "raft_multi" => {
            raft_multi::run();
        }
        "f2s_copy" => {
            f2s_copy::run();
        }
        "tk_echo" => {
            tk_echo::run().await;
        }
        "kf_log" => {
            kf_log::run().await?;
        }
        main => {
            println!("provided an invalid main {}", main);
        }
    };

    Ok(())
}
