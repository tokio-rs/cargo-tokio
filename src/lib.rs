mod ci;
mod cli;
mod targets;

use ci::TokioCIStep;
use cli::CargoTokio;

use structopt::StructOpt;

use std::io;

pub fn run() -> io::Result<()> {
    let cli = CargoTokio::from_args();

    match cli.step() {
        Some("test") => TokioCIStep::test_tokio_full(),
        Some("test-unstable") => TokioCIStep::test_tokio_full_unstable(),
        Some("miri") => TokioCIStep::miri(),
        Some("san") => TokioCIStep::san(),
        Some("cross") => TokioCIStep::cross(),
        Some("features") => TokioCIStep::features(),
        Some("minrust") => TokioCIStep::minrust(),
        Some("fmt") => TokioCIStep::fmt(),
        Some("clippy") => TokioCIStep::clippy(),
        Some("docs") => TokioCIStep::docs(),
        Some("loom") => TokioCIStep::loom(),
        None => ci::run_all_steps(),
        _ => Err(std::io::ErrorKind::InvalidInput.into()),
    }
}
