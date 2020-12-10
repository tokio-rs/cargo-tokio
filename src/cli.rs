use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "Tokio Cargo CI tool",
    about = "A tool to make your CI life easier locally"
)]
pub(crate) struct TokioCLI {
    #[structopt(short)]
    pub step: Option<String>,
}
