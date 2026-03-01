use clap::Parser;
use std::fmt::Display;

use crate::queries;

#[derive(Debug, Clone, Parser, clap::ValueEnum, Default)]
pub enum Language {
    Golang,
    C,
    Cpp,
    Python,
    Java,
    JavaScript,
    Ruby,
    #[default]
    Rust,
}

#[derive(Debug, Clone)]
pub enum Comment {
    Dash,
    Slash,
}

impl Comment {
    pub fn variants() -> Vec<Self> {
        vec![Self::Dash, Self::Slash]
    }

    pub fn remove(&self, text: &str) -> String {
        match self {
            Self::Dash => text.trim_start_matches('#').trim().to_string(),
            Self::Slash => text.trim_start_matches('/').trim().to_string(),
        }
    }
}

impl Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Comment::*;
        match self {
            Dash => write!(f, "#"),
            Slash => write!(f, "//"),
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
        use Language::*;
        match self {
            Golang => queries::QUERY_GOLANG,
            Rust => queries::QUERY_RUST,
            C => unimplemented!(),
            Cpp => unimplemented!(),
            Ruby => unimplemented!(),
            Python => unimplemented!(),
            Java => unimplemented!(),
            JavaScript => unimplemented!(),
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
    pub fn comment(&self) -> Comment {
        use Language::*;
        match self {
            Golang | C | Cpp | Java | JavaScript | Rust => Comment::Slash,
            Python | Ruby => Comment::Dash,
        }
    }
    pub fn file_ending(&self) -> &'static str {
        use Language::*;
        match self {
            Rust => ".rs",
            C => ".c",
            Cpp => ".cpp",
            Java => ".java",
            JavaScript => ".js",
            Python => ".py",
            Golang => ".go",
            Ruby => ".rb",
        }
    }
}
