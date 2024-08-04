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

#[derive(Debug, Clone)]
pub enum Comment {
    Dash,
    Slash,
}

impl Comment {
    pub fn variants() -> Vec<Self> {
        vec![Self::Dash, Self::Slash]
    }
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
            Golang => query_go(),
            Rust => query_rust(),
            C => query_c(),
            Cpp => query_cpp(),
            Ruby => query_ruby(),
            Python => query_python(),
            Java => query_java(),
            JavaScript => query_javascript(),
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
}

fn query_go() -> &'static str {
    r#"(
	(
    	(
    		(comment) @severity
    	)
        (#match? @severity "^//(\\s)*([Ii][Nn][Ff][Oo]|[Dd][Ee][Bb][Uu][Gg]|[Tt][Rr][Aa][Cc][Ee]|[Ww][Aa][Rr][Nn]|[Ff][Aa][Tt][Aa][Ll])")
    )
   	.
    (comment) @subject
    .
    (comment)*? @description
    )"#
}

fn query_rust() -> &'static str {
    r#"(
	(
    	(
    		(line_comment) @level
    	)
        (#match? @level "^//(\\s)*([Ii][Nn][Ff][Oo]|[Dd][Ee][Bb][Uu][Gg]|[Ww][Aa][Rr][Nn]|[Tt][Rr][Aa][Cc][Ee]|[Ff][Aa][Tt][Aa][Ll])")
    )
   	.
    (line_comment) @subject
    .
    (line_comment)*? @description
    )"#
}

fn query_c() -> &'static str {
    r#"(
	(
    	(
    		(comment) @level
    	)
        (#match? @level "^//(\\s)*([Ii][Nn][Ff][Oo]|[Dd][Ee][Bb][Uu][Gg]|[Ww][Aa][Rr][Nn]|[Tt][Rr][Aa][Cc][Ee]|[Ff][Aa][Tt][Aa][Ll])")
    )
   	.
    (comment) @subject
    .
    (comment)*? @description
    )"#
}

fn query_cpp() -> &'static str {
    r#"(
	(
    	(
    		(comment) @level
    	)
        (#match? @level "^//(\\s)*([Ii][Nn][Ff][Oo]|[Dd][Ee][Bb][Uu][Gg]|[Ww][Aa][Rr][Nn]|[Tt][Rr][Aa][Cc][Ee]|[Ff][Aa][Tt][Aa][Ll])")
    )
   	.
    (comment) @subject
    .
    (comment)*? @description
    )"#
}

fn query_ruby() -> &'static str {
    r#"(
	(
    	(
    		(comment) @level
    	)
        (#match? @level "^#(\\s)*([Ii][Nn][Ff][Oo]|[Dd][Ee][Bb][Uu][Gg]|[Ww][Aa][Rr][Nn]|[Tt][Rr][Aa][Cc][Ee]|[Ff][Aa][Tt][Aa][Ll])")
    )
   	.
    (comment) @subject
    .
    (comment)*? @description
    )"#
}

fn query_python() -> &'static str {
    r#"(
	(
    	(
    		(comment) @level
    	)
        (#match? @level "^#(\\s)*([Ii][Nn][Ff][Oo]|[Dd][Ee][Bb][Uu][Gg]|[Ww][Aa][Rr][Nn]|[Tt][Rr][Aa][Cc][Ee]|[Ff][Aa][Tt][Aa][Ll])")
    )
   	.
    (comment) @subject
    .
    (comment)*? @description
    )"#
}
fn query_java() -> &'static str {
    r#"(
	(
    	(
    		(line_comment) @level
    	)
        (#match? @level "^//(\\s)*([Ii][Nn][Ff][Oo]|[Dd][Ee][Bb][Uu][Gg]|[Ww][Aa][Rr][Nn]|[Tt][Rr][Aa][Cc][Ee]|[Ff][Aa][Tt][Aa][Ll])")
    )
   	.
    (line_comment) @subject
    .
    (line_comment)*? @description
    )"#
}

fn query_javascript() -> &'static str {
    r#"(
	(
    	(
    		(comment) @level
    	)
        (#match? @level "^//(\\s)*([Ii][Nn][Ff][Oo]|[Dd][Ee][Bb][Uu][Gg]|[Ww][Aa][Rr][Nn]|[Tt][Rr][Aa][Cc][Ee]|[Ff][Aa][Tt][Aa][Ll])")
    )
   	.
    (comment) @subject
    .
    (comment)*? @description
    )"#
}
