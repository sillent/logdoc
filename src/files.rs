use std::{collections::hash_map::Entry, path::Path};

use crate::meta::Pos;
use walkdir;

pub trait WalkInPosition {
    fn line_start(&self) -> usize;
    fn line_end(&self) -> usize;
    fn pos_start(&self) -> usize;
    fn pos_end(&self) -> usize;
}

pub fn list_files_only<T>(
    paths: Vec<T>,
    recurse: bool,
) -> Result<Vec<String>, Box<dyn std::error::Error>>
where
    T: AsRef<Path>,
{
    // for path in paths {
    //     let wd = walkdir::WalkDir::new(path);
    //     for entry in wd.into_iter().filter_entry(|e| !is_hidden(e))
    //     // .filter_entry(|e| is_file(e))?
    //     {
    //         entry?;
    //         return Ok(vec![String::from("")]);
    //     }
    // }

    // if let Ok(entry) = std::fs::read_dir(pa)
    unimplemented!();
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn is_file(entry: &walkdir::DirEntry) -> bool {
    entry.file_type().is_file()
}

pub fn walk_dir(path: &std::path::Path, result: &mut Vec<String>, recurse: bool) {
    if let Ok(entry) = std::fs::read_dir(path) {
        for e in entry {
            if let Ok(e) = e {
                let path = e.path();
                if path.is_dir() {
                    if recurse {
                        walk_dir(&path, result, recurse);
                    }
                } else if path.is_file() {
                    if let Ok(path) = path.into_os_string().into_string() {
                        result.push(path);
                    }
                }
            }
        }
    }
}

pub fn search_in_file<T>(data: T, pos: &Pos) -> Vec<u8>
where
    T: AsRef<[u8]>,
{
    let mut lines: Vec<Vec<u8>> = vec![];
    let mut local_line: Vec<u8> = vec![];
    for byte in data.as_ref() {
        local_line.push(byte.clone());
        if byte.eq(&10) {
            lines.push(local_line.clone());
            local_line.clear();
            continue;
        }
    }
    let mut ret = vec![];
    for (line_num, line) in lines.iter().enumerate() {
        if line_num.ge(&(pos.start.0 as usize)) && line_num.le(&(pos.end.0 as usize)) {
            for (char_num, char) in line.iter().enumerate() {
                if char_num.ge(&(pos.start.1 as usize)) && char_num.le(&(pos.end.1 as usize)) {
                    ret.push(*char);
                }
            }
        }
    }
    ret
}

pub fn search_in_file_dyn<T, W>(data: T, pos: &W) -> Vec<u8>
where
    T: AsRef<[u8]>,
    W: WalkInPosition,
{
    let mut lines: Vec<Vec<u8>> = vec![];
    let mut local_line: Vec<u8> = vec![];
    for byte in data.as_ref() {
        local_line.push(byte.clone());
        if byte.eq(&10) {
            lines.push(local_line.clone());
            local_line.clear();
            continue;
        }
    }
    let mut ret = vec![];
    for (line_num, line) in lines.iter().enumerate() {
        if line_num.ge(&pos.line_start()) && line_num.le(&pos.line_end()) {
            for (char_num, char) in line.iter().enumerate() {
                if char_num.ge(&pos.pos_start()) && char_num.le(&pos.pos_end()) {
                    ret.push(*char);
                }
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use crate::{files::search_in_file_dyn, meta::Pos};

    use super::search_in_file;

    #[test]
    fn test_walk_file() {
        let data = r#"Hello,
December is a last month in the year
 When January comes
All gifts are gone
"#;
        let pos = Pos {
            typo: crate::meta::Typo::Level,
            start: (2, 1),
            end: (2, 4),
        };
        let result = search_in_file(data.as_bytes(), &pos);
        assert_eq!(vec![87u8, 104, 101, 110], result);
    }

    #[test]
    fn test_walk_file_dyn() {
        let data = r#"Hello,
December is a last month in the year
 When January comes
All gifts are gone
"#;
        let pos = Pos {
            typo: crate::meta::Typo::Level,
            start: (2, 1),
            end: (2, 4),
        };
        let result = search_in_file_dyn(data.as_bytes(), &pos);
        assert_eq!(vec![87u8, 104, 101, 110], result);
    }
}
