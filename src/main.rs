#[macro_use]
extern crate slog;

mod raft_multi;

fn main() {
    raft_multi::run();
}