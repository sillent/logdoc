use clap::Parser;

#[derive(Debug, Parser, Clone)]
#[command(name = "LogDoc")]
#[command(version, about="Create .MD or .CSV files with information about logs", long_about=None)]
pub struct Arg {
    /// Project name (used in generated files)
    #[arg(short, long)]
    pub project_name: String,

    /// Directories in which files are searched for processing
    #[arg(short, long)]
    pub directories: Option<Vec<String>>,

    /// Recursively search files in directories
    #[arg(short)]
    pub recurse: bool,

    /// Additionaly passed files
    #[arg(short, long)]
    pub files: Option<Vec<String>>,

    /// Specify language that should be proceeded
    #[arg(short, long)]
    #[clap(value_parser)]
    pub language: Language,

    /// Specify directory when data should be saved
    #[arg(short, long)]
    #[clap(default_value = ".")]
    pub save_path: String,

    /// Specify format for save data
    #[arg(long, short = 't')]
    #[clap(value_parser, default_value = "md")]
    pub save_type: SaveType,

    /// Description for Info log
    #[arg(long)]
    #[clap(env = "INFO_DESC")]
    pub info_desc: Option<String>,

    /// Description for Debug log
    #[arg(long)]
    #[clap(env = "DEBUG_DESC")]
    pub debug_desc: Option<String>,

    /// Description for Trace log
    #[arg(long)]
    #[clap(env = "TRACE_DESC")]
    pub trace_desc: Option<String>,

    /// Description for Warn log
    #[arg(long)]
    #[clap(env = "WARN_DESC")]
    pub warn_desc: Option<String>,

    /// Description for Fatal log
    #[arg(long)]
    #[clap(env = "FATAL_DESC")]
    pub fatal_desc: Option<String>,
}

#[derive(Debug, Parser, clap::ValueEnum, Clone)]
pub enum Language {
    Golang,
    C,
    Cpp,
    Python,
    Java,
    JavaScript,
    Ruby,
    Rust,
}

#[derive(Debug, Parser, clap::ValueEnum, Default, Clone)]
pub enum SaveType {
    #[default]
    MD,
    CSV,
}

impl ToString for Language {
    fn to_string(&self) -> String {
        use Language::*;
        match self {
            Golang => "golang".to_owned(),
            C => "c".to_owned(),
            Cpp => "cpp".to_owned(),
            Python => "python".to_owned(),
            Java => "java".to_owned(),
            JavaScript => "javascript".to_owned(),
            Ruby => "ruby".to_owned(),
            Rust => "rust".to_owned(),
        }
    }
}

impl ToString for SaveType {
    fn to_string(&self) -> String {
        use SaveType::*;
        match self {
            MD => "markdown".to_owned(),
            CSV => "csv".to_owned(),
        }
    }
}

impl Arg {
    pub fn directories(&self) -> Vec<String> {
        let dirs = if let Some(dirs) = self.directories.clone() {
            dirs
        } else {
            vec![]
        };
        dirs
    }
    pub fn directories_ref(&self) -> Vec<&String> {
        if let Some(dirs) = &self.directories {
            let v = dirs.iter().map(|x| x).collect();
            return v;
        }
        return vec![];
    }
    pub fn file_suffix(&self) -> String {
        match self.save_type {
            SaveType::MD => "md".to_owned(),
            SaveType::CSV => "csv".to_owned(),
        }
    }
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
