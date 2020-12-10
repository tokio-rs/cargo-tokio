use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "Cargo Tokio CI tool",
    about = "A tool to make your CI life easier locally",
    bin_name = "cargo"
)]
pub(crate) enum CargoTokio {
    #[structopt(name = "tokio")]
    Tokio { step: Option<String> },
}

impl CargoTokio {
    pub fn step(&self) -> Option<&str> {
        match self {
            CargoTokio::Tokio { step } => step.as_deref(),
        }
    }
}
