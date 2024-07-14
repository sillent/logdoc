use std::path::PathBuf;

use clap::Parser;

use crate::files;

#[derive(Debug, Parser)]
#[command(name = "LogDoc")]
#[command(version, about="Create .MD files with information about logs", long_about=None)]
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

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Language::Golang => "golang".to_owned(),
            Language::C => "c".to_owned(),
            Language::Cpp => "cpp".to_owned(),
            Language::Python => "python".to_owned(),
            Language::Java => "java".to_owned(),
            Language::JavaScript => "javascript".to_owned(),
            Language::Ruby => "ruby".to_owned(),
            Language::Rust => "rust".to_owned(),
        }
    }
}

impl Arg {
    pub fn files_list(&self) -> Vec<String> {
        let mut result = vec![];
        if let Some(ref dirs) = self.directories {
            for dir in dirs {
                files::walk_dir(&PathBuf::from(dir), &mut result, self.recurse);
            }
        }
        if let Some(ref files) = self.files {
            files.iter().map(|x| result.push(x.clone())).count();
        }
        result
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
