#[macro_use]
extern crate slog;

#[macro_use]
extern crate clap;

mod raft_multi;
mod f2s_copy;

use clap::App;

#[derive(Clap)]
#[clap(version = "v1.0-beta")]
struct Opts {
    main: String
}

fn main() {
    let opts = Opts::parse();

    match opts.main.as_ref() {
        "raft_multi" => {
            raft_multi::run();
        },
        "f2s_copy" => {
            f2s_copy::run();
        },
        main => {
            println!("provided an invalid main {}", main);
        }
    };
}