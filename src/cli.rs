use clap::Parser;

#[derive(Parser)]
#[command(name = "poglo")]
#[command(author = "Emiliano Maccaferri <inbox@emilianomaccaferri.com>")]
#[command(version = "0.0.1")]
#[command(about = "A thing that broadcasts stuff")]
pub struct Cli {
    #[arg(long)]
    conf_path: String,
    #[arg(long)]
    templates_path: String,
}

impl Cli {
    pub fn conf_path(&self) -> &str {
        &self.conf_path
    }
    pub fn templates_path(&self) -> &str {
        &self.templates_path
    }
}
