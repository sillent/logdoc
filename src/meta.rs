use std::fmt::Display;

use tree_sitter::QueryCapture;

use crate::files;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Message(pub String);
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Subject(pub String);
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Description(pub String);

#[derive(Debug, Default)]
pub struct Meta {
    pub level: Level,
    pub message: Message,
    pub subject: Subject,
    pub description: Description,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Typo {
    #[default]
    Level,
    Subject,
    Description,
}

#[derive(Debug, Clone, Default)]
pub struct Pos {
    pub typo: Typo,
    pub start: (u32, u32),
    pub end: (u32, u32),
}

#[derive(Debug, PartialEq, Eq, Default)]
pub enum Level {
    #[default]
    Info,
    Debug,
    Trace,
    Warn,
    Fatal,
}

impl Level {
    fn variants(&self) -> Vec<&'static str> {
        use Level::*;
        match self {
            Info => {
                vec!["info:", "info"]
            }
            Debug => {
                vec!["debug:", "debug"]
            }
            Trace => {
                vec!["trace:", "trace"]
            }
            Warn => {
                vec!["warn:", "warn"]
            }
            Fatal => {
                vec!["fatal:", "fatal"]
            }
        }
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let st = match self {
            Level::Info => "info",
            Level::Debug => "debug",
            Level::Trace => "trace",
            Level::Warn => "warn",
            Level::Fatal => "fatal",
        };
        write!(f, "{}", st)
    }
}
impl<T> From<(&String, &T)> for Message
where
    T: Display,
{
    fn from(value: (&String, &T)) -> Self {
        let mut line = value.0.clone();
        let level = Level::from((value.0, value.1));
        if line
            .to_lowercase()
            .starts_with(format!("{}", value.1).as_str())
        {
            let l = format!("{}", value.1).len();
            crop_letters(&mut line, l);
            delete_spaces_dotes(&mut line);

            let level_variants = level.variants();
            for variant in level_variants {
                if line.to_lowercase().starts_with(variant) {
                    let len = variant.len();
                    crop_letters(&mut line, len);
                    break;
                }
            }
            delete_spaces_dotes(&mut line);
        }
        Message(line)
    }
}

impl Message {
    pub fn format(&self) -> String {
        let mut msg = self.0.clone();
        if msg.ends_with("\n") {
            msg.pop();
            msg
        } else {
            msg
        }
    }
}

impl Subject {
    pub fn format(&self) -> String {
        let mut msg = self.0.clone();
        if msg.ends_with("\n") {
            msg.pop();
            msg
        } else {
            msg
        }
    }
}

impl Description {
    pub fn format(&self) -> String {
        let mut msg = self.0.clone();
        if msg.ends_with("\n") {
            msg.pop();
        }
        msg.replace("\n", "<br/>")
    }
}

impl<T> From<(&String, &T)> for Subject
where
    T: Display,
{
    fn from(value: (&String, &T)) -> Self {
        let mut line = value.0.clone();
        if line
            .to_lowercase()
            .starts_with(format!("{}", value.1).as_str())
        {
            let l = format!("{}", value.1).len();
            crop_letters(&mut line, l);
            delete_spaces_dotes(&mut line);
        }
        Subject(line)
    }
}
impl<T> From<(&String, &T)> for Description
where
    T: Display,
{
    fn from(value: (&String, &T)) -> Self {
        let mut line = value.0.clone();
        if line
            .to_lowercase()
            .starts_with(format!("{}", value.1).as_str())
        {
            let l = format!("{}", value.1).len();
            crop_letters(&mut line, l);
            delete_spaces_dotes(&mut line);
        }
        Description(line)
    }
}

fn delete_spaces_dotes(line: &mut String) {
    loop {
        if line.starts_with(" ") {
            crop_letters(line, 1);
        } else if line.starts_with(":") {
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

impl<T> From<(&String, &T)> for Level
where
    T: Display,
{
    fn from(value: (&String, &T)) -> Self {
        let comment = format!("{}", value.1);
        let comment_len = comment.len();
        let mut line = value.0.to_owned();
        crop_letters(&mut line, comment_len);
        delete_spaces_dotes(&mut line);
        if line.to_lowercase().starts_with("info:") || line.to_lowercase().starts_with("info") {
            return Level::Info;
        }
        if line.to_lowercase().starts_with("debug:") || line.to_lowercase().starts_with("debug") {
            return Level::Debug;
        }
        if line.to_lowercase().starts_with("trace:") || line.to_lowercase().starts_with("trace") {
            return Level::Trace;
        }
        if line.to_lowercase().starts_with("fatal:") || line.to_lowercase().starts_with("fatal") {
            return Level::Fatal;
        }
        if line.to_lowercase().starts_with("warn:") || line.to_lowercase().starts_with("warn") {
            return Level::Warn;
        }
        return Level::Info;
    }
}

impl From<u32> for Typo {
    fn from(value: u32) -> Self {
        match value {
            0 => Typo::Level,
            1 => Typo::Subject,
            2 => Typo::Description,
            _ => Typo::default(),
        }
    }
}

impl TryFrom<u8> for Typo {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Typo::Level),
            1 => Ok(Typo::Subject),
            2 => Ok(Typo::Description),
            _ => Err("unsupported value"),
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
mod tests {
    use crate::language::Comment;

    use super::*;

    #[test]
    fn check_level_from_string() {
        let d1 = String::from("// DEBUG: debug message");
        assert_eq!(Level::from((&d1, &Comment::Slash)), Level::Debug);
        let d2 = String::from("// trace: trace message");
        assert_eq!(Level::from((&d2, &Comment::Slash)), Level::Trace);
        let d3 = String::from("// no level message");
        assert_eq!(Level::from((&d3, &Comment::Slash)), Level::Info);
    }

    #[test]
    fn check_message_from_string() {
        let relevant = String::from("test Message");
        let msgrelevant = Message(relevant.clone());

        {
            let i1 = format!("// info  :{}", relevant);
            let i2 = format!("//info {}", relevant);
            let m1 = Message::from((&i1, &Comment::Slash));
            let m2 = Message::from((&i2, &Comment::Slash));
            assert_eq!(msgrelevant, m1);
            assert_eq!(msgrelevant, m2);
        }
        {
            let d1 = format!("//debug: {}", relevant);
            let d2 = format!("// Debug : {}", relevant);
            let m1 = Message::from((&d1, &Comment::Slash));
            let m2 = Message::from((&d2, &Comment::Slash));
            assert_eq!(msgrelevant, m1);
            assert_eq!(msgrelevant, m2);
        }
        {
            let t1 = format!("// TRACE {}", relevant);
            let t2 = format!("//trace  : {}", relevant);
            let m1 = Message::from((&t1, &Comment::Slash));
            let m2 = Message::from((&t2, &Comment::Slash));
            assert_eq!(msgrelevant, m1);
            assert_eq!(msgrelevant, m2);
        }
        {
            let w1 = format!("// WaRN       {}", relevant);
            let w2 = format!("//warn:{}", relevant);
            let m1 = Message::from((&w1, &Comment::Slash));
            let m2 = Message::from((&w2, &Comment::Slash));
            assert_eq!(msgrelevant, m1);
            assert_eq!(msgrelevant, m2);
        }
        {
            let w1 = format!("# TRACE {}", relevant);
            let m1 = Message::from((&w1, &Comment::Dash));
            assert_eq!(msgrelevant, m1);
        }
    }
}
