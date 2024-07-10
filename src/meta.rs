use tree_sitter::QueryCapture;

#[derive(Debug)]
pub struct Meta {
    pub file: String,
    pub level: Level,
    pub message: String,
    pub subject: String,
    pub description: String,
}

#[derive(Debug)]
pub struct MetaPos {
    pub comment1: Pos,
    pub comment2: Pos,
    pub comment3: Vec<Pos>,
}

#[derive(Debug, Clone)]
pub enum Typo {
    Level,
    Subject,
    Description,
}

#[derive(Debug, Clone)]
pub struct Pos {
    pub typo: Typo,
    pub start: (u32, u32),
    pub end: (u32, u32),
}

#[derive(Debug)]
pub enum Level {
    Info,
    Debug,
    Trace,
    Warn,
    Fatal,
}

impl From<u32> for Typo {
    fn from(value: u32) -> Self {
        match value {
            0 => Typo::Level,
            1 => Typo::Subject,
            2 => Typo::Description,
            _ => unimplemented!("shouldn't happened"),
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

pub fn form_meta(pos: Vec<Pos>) -> MetaPos {
    let mut mpos = MetaPos::default();
    for p in pos {
        match p.typo {
            Typo::Level => mpos.comment1 = p,
            Typo::Subject => mpos.comment2 = p,
            Typo::Description => mpos.comment3.push(p),
        }
    }
    mpos
}

impl Default for MetaPos {
    fn default() -> Self {
        MetaPos {
            comment1: Pos {
                typo: Typo::Level,
                start: (0, 0),
                end: (0, 0),
            },
            comment2: Pos {
                typo: Typo::Subject,
                start: (0, 0),
                end: (0, 0),
            },
            comment3: vec![],
        }
    }
}
