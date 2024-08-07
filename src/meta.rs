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
                vec!["info:"]
            }
            Debug => {
                vec!["debug:"]
            }
            Trace => {
                vec!["trace:"]
            }
            Warn => {
                vec!["warn:"]
            }
            Fatal => {
                vec!["fatal:"]
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
// impl<T> From<(&String, &T)> for Message
// where
//     T: Display,
// {
//     fn from(value: (&String, &T)) -> Self {
//         let mut line = value.0.clone();
//         let level = Level::from((value.0, value.1));
//         // println!("level = {}", level);
//         if line
//             .to_lowercase()
//             .starts_with(format!("{}", value.1).as_str())
//         {
//             let l = format!("{}", value.1).len();
//             crop_letters(&mut line, l);
//             // println!("after crop = {line}");
//             delete_spaces_dotes(&mut line);
//             // delete_spaces(&mut line);

//             let level_variants = level.variants();
//             for variant in level_variants {
//                 if line.to_lowercase().starts_with(variant) {
//                     let len = variant.len();
//                     crop_letters(&mut line, len);
//                     break;
//                 }
//             }
//             delete_spaces_dotes(&mut line);
//             println!("line = {line}");
//             // delete_spaces(&mut line);
//         }
//         Message(line)
//     }
// }
impl<T> TryFrom<(&String, &T)> for Message
where
    T: Display,
{
    type Error = &'static str;
    fn try_from(value: (&String, &T)) -> Result<Self, Self::Error> {
        // fn from(value: (&String, &T)) -> Self {
        let mut line = value.0.clone();
        let level = Level::from((value.0, value.1));
        // println!("level = {}", level);
        if line
            .to_lowercase()
            .starts_with(format!("{}", value.1).as_str())
        {
            let l = format!("{}", value.1).len();
            crop_letters(&mut line, l);
            // println!("after crop = {line}");
            delete_spaces_dotes(&mut line);
            // delete_spaces(&mut line);

            let level_variants = level.variants();
            for variant in level_variants {
                if line.to_lowercase().starts_with(variant) {
                    let len = variant.len();
                    crop_letters(&mut line, len);
                    delete_spaces_dotes(&mut line);
                    return Ok(Message(line));
                    // break;
                }
            }
            // delete_spaces_dotes(&mut line);

            // println!("line = {line}");
            // delete_spaces(&mut line);
        }
        // Message(line)
        Err("unexpected")
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
            // delete_spaces_dotes(&mut line);
            delete_spaces(&mut line);
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
            // delete_spaces_dotes(&mut line);
            delete_spaces(&mut line);
        }
        Description(line)
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
        // delete_spaces_dotes(&mut line);
        delete_spaces(&mut line);
        if line.to_lowercase().starts_with("info") {
            crop_letters(&mut line, 4);
            delete_spaces(&mut line);
            if line.to_lowercase().starts_with(":") {
                return Level::Info;
            }
            // return Level::Info;
        }
        if line.to_lowercase().starts_with("debug") {
            crop_letters(&mut line, 5);
            delete_spaces(&mut line);
            if line.to_lowercase().starts_with(":") {
                return Level::Debug;
            }
            // return Level::Debug;
        }
        if line.to_lowercase().starts_with("trace") {
            crop_letters(&mut line, 5);
            delete_spaces(&mut line);
            if line.to_lowercase().starts_with(":") {
                return Level::Trace;
            }
            // return Level::Trace;
        }
        if line.to_lowercase().starts_with("fatal") {
            crop_letters(&mut line, 5);
            delete_spaces(&mut line);
            if line.to_lowercase().starts_with(":") {
                return Level::Fatal;
            }
            // return Level::Fatal;
        }
        if line.to_lowercase().starts_with("warn") {
            crop_letters(&mut line, 4);
            delete_spaces(&mut line);
            if line.to_lowercase().starts_with(":") {
                return Level::Warn;
            }
            // return Level::Warn;
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
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Typo::Level),
            1 => Ok(Typo::Subject),
            2 => Ok(Typo::Description),
            n @ _ => Err(format!("unsupported value {n}")),
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
        let comments = vec![Comment::Slash, Comment::Dash];
        let variants: Vec<(Level, Vec<&'static str>)> = vec![
            (Level::Info, Level::Info.variants()),
            (Level::Debug, Level::Debug.variants()),
            (Level::Trace, Level::Trace.variants()),
            (Level::Warn, Level::Warn.variants()),
            (Level::Fatal, Level::Fatal.variants()),
        ];
        // .into_iter()
        // .flatten()
        // .collect();
        for (l, vs) in variants {
            for comment in &comments {
                for v in &vs {
                    let st = format!("{} {} message", comment, v);
                    let cur_level = Level::from((&st, &comment));
                    assert_eq!(l, cur_level);
                }
            }
        }

        // let d1 = String::from("// DEBUG: debug message");
        // assert_eq!(Level::from((&d1, &Comment::Slash)), Level::Debug);
        // let d2 = String::from("// trace: trace message");
        // assert_eq!(Level::from((&d2, &Comment::Slash)), Level::Trace);
        // let d3 = String::from("// no level message");
        // assert_eq!(Level::from((&d3, &Comment::Slash)), Level::Info);
    }

    #[test]
    fn check_from() {
        let comments = Comment::variants();
        let variants: Vec<&'static str> = vec![
            Level::Info.variants(),
            Level::Debug.variants(),
            Level::Trace.variants(),
            Level::Warn.variants(),
            Level::Fatal.variants(),
        ]
        .into_iter()
        .flatten()
        .collect();

        let relevant_str = "test message";
        let relevant_message = Message(String::from(relevant_str));
        let relevant_subject = Subject(String::from(relevant_str));
        let relevant_desc = Description(String::from(relevant_str));

        for variant in variants {
            for comment in &comments {
                let s = format!("{} {} {}", comment, variant, relevant_str);
                let m = Message::try_from((&s, &comment)).unwrap();
                assert_eq!(relevant_message, m);
            }
        }
        for comment in &comments {
            let txt = format!("{} {}", comment, relevant_str);
            let s = Subject::from((&txt, &comment));
            let d = Description::from((&txt, &comment));
            assert_eq!(relevant_subject, s);
            assert_eq!(relevant_desc, d);
        }
    }

    #[test]
    fn check_message_from_string() {
        let relevant = "test Message";
        let msgrelevant = Message(relevant.to_owned());

        {
            let i1 = format!("// info: {}", relevant);
            // let i2 = format!("//info {}", relevant);
            let m1 = Message::try_from((&i1, &Comment::Slash)).unwrap();
            // let m2 = Message::from((&i2, &Comment::Slash));
            assert_eq!(msgrelevant, m1);
            // assert_eq!(msgrelevant, m2);
        }
        {
            let d1 = format!("//debug: {}", relevant);
            let d2 = format!("// Debug: {}", relevant);
            let m1 = Message::try_from((&d1, &Comment::Slash)).unwrap();
            let m2 = Message::try_from((&d2, &Comment::Slash)).unwrap();
            assert_eq!(msgrelevant, m1);
            assert_eq!(msgrelevant, m2);
        }
        {
            // let t1 = format!("// TRACE {}", relevant);
            let t2 = format!("//trace: {}", relevant);
            // let m1 = Message::from((&t1, &Comment::Slash));
            let m2 = Message::try_from((&t2, &Comment::Slash)).unwrap();
            // assert_eq!(msgrelevant, m1);
            assert_eq!(msgrelevant, m2);
        }
        {
            // let w1 = format!("// WaRN       {}", relevant);
            let w2 = format!("//warn:{}", relevant);
            // let m1 = Message::from((&w1, &Comment::Slash));
            let m2 = Message::try_from((&w2, &Comment::Slash)).unwrap();
            // assert_eq!(msgrelevant, m1);
            assert_eq!(msgrelevant, m2);
        }
        {
            let w1 = format!("# TRACE: {}", relevant);
            let m1 = Message::try_from((&w1, &Comment::Dash)).unwrap();
            assert_eq!(msgrelevant, m1);
        }
        {
            let f1 = format!("# FATAL: {}", relevant);
            let m1 = Message::try_from((&f1, &Comment::Dash)).unwrap();
            assert_eq!(msgrelevant, m1);
        }
        {
            let e1 = format!("# info : {}", relevant);
            let m = Message::try_from((&e1, &Comment::Dash));
            assert_eq!(Err("unexpected"), m);
        }
    }
}
