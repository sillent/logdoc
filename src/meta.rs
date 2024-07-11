use tree_sitter::QueryCapture;

use crate::files;

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

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Default)]
pub enum Level {
    #[default]
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
