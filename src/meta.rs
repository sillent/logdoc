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
        let mut variants = vec![];
        use Level::*;
        match self {
            Info => {
                variants.push("info:");
                variants.push("info");
            }
            Debug => {
                variants.push("debug:");
                variants.push("debug");
            }
            Trace => {
                variants.push("trace:");
                variants.push("trace");
            }
            Warn => {
                variants.push("warn:");
                variants.push("warn");
            }
            Fatal => {
                variants.push("fatal:");
                variants.push("fatal");
            }
        }
        variants
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
impl From<(&String, &str)> for Message {
    fn from(value: (&String, &str)) -> Self {
        let mut line = value.0.clone();
        let level = Level::from(value.0);
        if line.to_lowercase().starts_with(value.1) {
            let l = value.1.len();
            crop_letters(&mut line, l);
            delete_spaces_dotes(&mut line);
            // let variants = vec![
            //     ("info:", 5),
            //     ("info", 4),
            //     ("debug:", 6),
            //     ("debug", 5),
            //     ("trace:", 6),
            //     ("trace", 5),
            //     ("warn:", 5),
            //     ("warn", 4),
            //     ("fatal:", 6),
            //     ("fatal", 5),
            // ];
            // for (variant, local) in variants {
            //     if line.to_lowercase().starts_with(variant) {
            //         crop_letters(&mut line, local);
            //         break;
            //     }
            // }
            let level_variants = level.variants();
            for variant in level_variants {
                if line.to_lowercase().starts_with(variant) {
                    let len = line.len();
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

impl From<(&String, &str)> for Subject {
    fn from(value: (&String, &str)) -> Self {
        let mut line = value.0.clone();
        if line.to_lowercase().starts_with(value.1) {
            let l = value.1.len();
            crop_letters(&mut line, l);
            delete_spaces_dotes(&mut line);
        }
        Subject(line)
    }
}
impl From<(&String, &str)> for Description {
    fn from(value: (&String, &str)) -> Self {
        let mut line = value.0.clone();
        if line.to_lowercase().starts_with(value.1) {
            let l = value.1.len();
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

impl From<&String> for Level {
    fn from(value: &String) -> Self {
        let v = value.clone();
        if v.to_lowercase().starts_with("// info") || v.to_lowercase().starts_with("//info") {
            return Level::Info;
        }
        if v.to_lowercase().starts_with("// debug") || v.to_lowercase().starts_with("//debug") {
            return Level::Debug;
        }
        if v.to_lowercase().starts_with("// trace") || v.to_lowercase().starts_with("//trace") {
            return Level::Trace;
        }
        if v.to_lowercase().starts_with("// fatal") || v.to_lowercase().starts_with("//fatal") {
            return Level::Fatal;
        }
        if v.to_lowercase().starts_with("// warn") || v.to_lowercase().starts_with("//warn") {
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
    use super::*;

    #[test]
    fn check_level_from_string() {
        let d1 = String::from("// DEBUG: debug message");
        assert_eq!(Level::from(&d1), Level::Debug);
        let d2 = String::from("// trace: trace message");
        assert_eq!(Level::from(&d2), Level::Trace);
        let d3 = String::from("// no level message");
        assert_eq!(Level::from(&d3), Level::Info);
    }

    #[test]
    fn check_message_from_string() {
        let relevant = String::from("test Message");
        let msgrelevant = Message(relevant.clone());

        {
            let i1 = format!("// info  :{}", relevant);
            let i2 = format!("//info {}", relevant);
            let m1 = Message::from((&i1, "//"));
            let m2 = Message::from((&i2, "//"));
            assert_eq!(msgrelevant, m1);
            assert_eq!(msgrelevant, m2);
        }
        {
            let d1 = format!("//debug: {}", relevant);
            let d2 = format!("// Debug : {}", relevant);
            let m1 = Message::from((&d1, "//"));
            let m2 = Message::from((&d2, "//"));
            assert_eq!(msgrelevant, m1);
            assert_eq!(msgrelevant, m2);
        }
        {
            let t1 = format!("// TRACE {}", relevant);
            let t2 = format!("//trace  : {}", relevant);
            let m1 = Message::from((&t1, "//"));
            let m2 = Message::from((&t2, "//"));
            assert_eq!(msgrelevant, m1);
            assert_eq!(msgrelevant, m2);
        }
        {
            let w1 = format!("// WaRN       {}", relevant);
            let w2 = format!("//warn:{}", relevant);
            let m1 = Message::from((&w1, "//"));
            let m2 = Message::from((&w2, "//"));
            assert_eq!(msgrelevant, m1);
            assert_eq!(msgrelevant, m2);
        }
    }
}
