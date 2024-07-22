use std::fmt::Display;

#[derive(Debug, Clone)]
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

impl From<&crate::args::Language> for Language {
    fn from(value: &crate::args::Language) -> Self {
        use crate::args;
        match value {
            args::Language::Golang => Self::Golang,
            args::Language::C => Self::C,
            args::Language::Cpp => Self::Cpp,
            args::Language::Python => Self::Python,
            args::Language::Java => Self::Java,
            args::Language::JavaScript => Self::JavaScript,
            args::Language::Ruby => Self::Ruby,
            args::Language::Rust => Self::Rust,
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Language::*;
        match self {
            Golang => write!(f, "golang"),
            C => write!(f, "c"),
            Cpp => write!(f, "c++"),
            Python => write!(f, "python"),
            Java => write!(f, "java"),
            JavaScript => write!(f, "javascript"),
            Ruby => write!(f, "ruby"),
            Rust => write!(f, "rust"),
        }
    }
}

impl Language {
    pub fn query(&self) -> &str {
        match self {
            Language::Golang => query_go(),
            _ => r#"()"#,
        }
    }
    pub fn sitter_language(&self) -> tree_sitter::Language {
        use Language::*;
        match self {
            Golang => tree_sitter_go::language(),
            C => tree_sitter_c::language(),
            Cpp => tree_sitter_cpp::language(),
            Python => tree_sitter_python::language(),
            Java => tree_sitter_java::language(),
            JavaScript => tree_sitter_javascript::language(),
            Ruby => tree_sitter_ruby::language(),
            Rust => tree_sitter_rust::language(),
        }
    }
}

fn query_go() -> &'static str {
    r#"(
	(
    	(
    		(comment) @severity
    	)
        (#match? @severity "^// (INFO|DEBUG|TRACE|WARN|FATAL):")
    )
   	.
    (comment) @subject
    .
    (comment)*? @description
    )"#
}
