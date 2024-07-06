use clap::Parser;
#[derive(Debug, Parser)]
#[command(name = "LogDoc `Go`")]
#[command(version, about, long_about=None)]
pub struct Arg {
    #[arg(short, long)]
    pub project_name: String,

    #[arg(short, long)]
    pub directories: Option<Vec<String>>,

    #[arg(short)]
    pub recurse: bool,

    #[arg(short, long)]
    pub file: Option<String>,
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::Arg;
    #[test]
    fn verify_cli() {
        Arg::command().debug_assert();
    }
}
