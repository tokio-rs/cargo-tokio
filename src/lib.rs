mod ci;
mod cli;
mod targets;

use ci::TokioCIStep;
use cli::TokioCLI;

use structopt::StructOpt;

pub fn run() -> std::io::Result<()> {
    let cli = TokioCLI::from_args();

    match cli.step.as_str() {
        "test" => TokioCIStep::test_tokio_full(),
        "test-unstable" => TokioCIStep::test_tokio_full_unstable(),
        "miri" => TokioCIStep::miri(),
        "san" => TokioCIStep::san(),
        "cross" => TokioCIStep::cross(),
        "features" => TokioCIStep::features(),
        "minrust" => TokioCIStep::minrust(),
        "fmt" => TokioCIStep::fmt(),
        "clippy" => TokioCIStep::clippy(),
        "docs" => TokioCIStep::docs(),
        "loom" => TokioCIStep::loom(),
        _ => Err(std::io::ErrorKind::InvalidInput.into()),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn bogus_test() {
        assert_eq!(1, 1)
    }
}
