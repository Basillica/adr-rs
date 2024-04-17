use log::{info, warn};

pub fn main() {
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
}

// env RUST_LOG=info cargo run --bin output-log
// https://rust-cli.github.io/book/tutorial/output.html
