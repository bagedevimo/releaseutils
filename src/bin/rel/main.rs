use log::info;

mod args;

use args::parse;
use releaseutils::logging::setup_logging;

fn main() {
    setup_logging();

    let args = parse();

    info!("Hello!");
}
