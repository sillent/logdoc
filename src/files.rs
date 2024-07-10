use crate::{
    args,
    meta::{MetaPos, Pos},
};
use std::io::{Seek, Write};

pub(crate) struct FileList {
    list: Vec<String>,
}

impl FileList {
    pub(crate) fn walk<T>(paths: Vec<T>) -> FileList
    where
        T: AsRef<str>,
    {
        Self { list: vec![] }
    }
}

pub fn walk_dir(path: &std::path::Path, result: &mut Vec<String>, recurse: bool) {
    if let Ok(entry) = std::fs::read_dir(path) {
        for e in entry {
            if let Ok(e) = e {
                let path = e.path();
                if path.is_dir() {
                    walk_dir(&path, result, recurse);
                } else if path.is_file() {
                    // path.into_os_string()
                    //     .into_string()
                    //     .ok()
                    //     .and_then(result.push);
                    if let Ok(path) = path.into_os_string().into_string() {
                        result.push(path);
                    }
                }
            }
        }
    }
}

pub fn walk_file<T>(data: T, pos: Pos) -> Vec<u8>
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

#[cfg(test)]
mod tests {
    use crate::meta::Pos;

    use super::walk_file;

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
        let result = walk_file(data.as_bytes(), pos);
        assert_eq!(vec![87u8, 104, 101, 110], result);
    }
}
