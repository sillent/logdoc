use clap::Parser;

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
    #[clap(value_parser, default_value_t=Language::Rust)]
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

impl Language {
    pub fn query(&self) -> &str {
        match self {
            Language::Golang => {
                r#"
(
	(
    	(
    		(comment) @severity
    	)
        (#match? @severity "^// (Info|Debug|Critical|Trace|Fatal):")
    )
   	.
    (comment) @subject
    .
    (comment)*? @description

)
"#
            }
            _ => r#"()"#,
        }
    }
    pub fn sitter_language(&self) -> tree_sitter::Language {
        match self {
            Language::Golang => tree_sitter_go::language(),
            _ => unimplemented!("TODO: implement other languages tree-sitter"),
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
