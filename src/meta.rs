use std::fmt::Display;

use tree_sitter::QueryCapture;

use crate::files;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Message(pub String);
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Comments {
    pub subject: String,
    pub description: Vec<String>,
}
#[derive(Debug, Default)]
pub struct Meta {
    pub level: Level,
    pub comments: Comments,
    pub message: Message,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Typo {
    #[default]
    Comments,
    Level,
    Content,
}

#[derive(Debug, Clone, Default)]
pub struct Pos {
    pub typo: Typo,
    pub start: (u32, u32),
    pub end: (u32, u32),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, Default)]
pub enum Level {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
    Fatal,
}

impl Level {
    pub const ALL: [Level; 6] = [
        Level::Trace,
        Level::Debug,
        Level::Info,
        Level::Warn,
        Level::Error,
        Level::Fatal,
    ];
}
impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let st = self.as_ref();
        write!(f, "{}", st)
    }
}

impl AsRef<str> for Level {
    fn as_ref(&self) -> &str {
        match self {
            Level::Trace => "trace",
            Level::Debug => "debug",
            Level::Info => "info",
            Level::Warn => "warn",
            Level::Error => "error",
            Level::Fatal => "fatal",
        }
    }
}

impl<T> From<T> for Message
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Message(trim_pairs(value.as_ref()).to_owned())
    }
}
fn trim_pairs(input: &str) -> &str {
    let mut start = 0;
    let mut end = input.len();
    while start < end && input[start..start + 1] == input[end - 1..end] {
        start += 1;
        end -= 1;
    }

    &input[start..end]
}

impl Message {
    pub fn format(&self) -> String {
        self.0.clone()
    }
}

impl Comments {
    pub fn format_subject(&self) -> String {
        self.subject.clone()
    }
    pub fn format_description(&self) -> Vec<String> {
        self.description.clone()
    }
    pub fn is_empty(&self) -> bool {
        self.subject.is_empty() && self.description.is_empty()
    }
}

fn delete_spaces(line: &mut String) {
    loop {
        if line.starts_with(" ") {
            crop_letters(line, 1);
        } else {
            break;
        }
    }
}

fn crop_letters(s: &mut String, pos: usize) {
    match s.char_indices().nth(pos) {
        Some((pos, _)) => {
            s.drain(..pos);
        }
        None => {
            s.clear();
        }
    }
}

impl From<&String> for Level {
    fn from(value: &String) -> Self {
        let line = value.to_lowercase().trim().to_owned();
        match line {
            line if line.contains("trace") => Level::Trace,
            line if line.contains("debug") => Level::Debug,
            line if line.contains("info") => Level::Info,
            line if line.contains("warn") => Level::Warn,
            line if line.contains("error") => Level::Error,
            line if line.contains("fatal") => Level::Fatal,
            _ => Level::Info,
        }
    }
}

impl From<u32> for Typo {
    fn from(value: u32) -> Self {
        match value {
            0 => Typo::Comments,
            1 => Typo::Level,
            2 => Typo::Content,
            _ => Typo::default(),
        }
    }
}

impl<'e> From<&QueryCapture<'e>> for Pos {
    fn from(value: &QueryCapture) -> Self {
        Pos {
            start: (
                value.node.start_position().row as u32,
                value.node.start_position().column as u32,
            ),
            end: (
                value.node.end_position().row as u32,
                value.node.end_position().column as u32,
            ),
            typo: Typo::from(value.index),
        }
    }
}

impl files::WalkInPosition for Pos {
    fn line_start(&self) -> usize {
        self.start.0 as usize
    }
    fn line_end(&self) -> usize {
        self.end.0 as usize
    }
    fn pos_start(&self) -> usize {
        self.start.1 as usize
    }
    fn pos_end(&self) -> usize {
        self.end.1 as usize
    }
}

#[cfg(test)]
mod tests {}
