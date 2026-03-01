use std::{error::Error, io::Write, path::Path};

use crate::{
    args::{self},
    language::Language,
    meta::Level,
};

pub trait WalkInPosition {
    fn line_start(&self) -> usize;
    fn line_end(&self) -> usize;
    fn pos_start(&self) -> usize;
    fn pos_end(&self) -> usize;
}

pub fn form_list_files(arg: &args::Arg) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut total = vec![];
    let lang = &arg.language;
    if let Some(files) = &arg.files {
        total.extend(
            files
                .iter()
                .filter(|file| {
                    file.ends_with(lang.file_ending())
                        && std::fs::metadata(file).is_ok_and(|x| x.is_file())
                })
                .cloned(),
        );
    }
    let recurse = arg.recurse;
    let mut files = list_files_in_dir(arg.directories(), recurse, lang)?;
    total.append(&mut files);

    Ok(total)
}
fn list_files_in_dir<T>(
    dirs: &[T],
    recurse: bool,
    language: &Language,
) -> Result<Vec<String>, Box<dyn std::error::Error>>
where
    T: AsRef<Path>,
{
    let files_total = dirs
        .iter()
        .map(|dir| walk_path(dir.as_ref(), recurse, language))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok(files_total)
}

fn walk_path(
    path: &std::path::Path,
    recurse: bool,
    language: &Language,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut files = vec![];
    let entries = std::fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        if is_hidden(&entry) {
            continue;
        }
        let file_type = entry.file_type()?;
        if file_type.is_symlink() {
            continue;
        }
        if file_type.is_dir() {
            if recurse {
                let mut inner = walk_path(&entry.path(), recurse, language)?;
                files.append(&mut inner);
            }
            continue;
        }
        if let Ok(path) = entry.path().into_os_string().into_string() {
            if path.ends_with(language.file_ending()) {
                files.push(path);
            }
        }
    }
    Ok(files)
}

fn is_hidden(entry: &std::fs::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn search_in_file_dyn<T, W>(data: T, pos: &W) -> Vec<u8>
where
    T: AsRef<[u8]>,
    W: WalkInPosition,
{
    let lines = data.as_ref().split(|&byte| byte == b'\n');
    let mut ret = vec![];
    for (line_num, line) in lines.clone().enumerate() {
        if line_num.ge(&pos.line_start()) && line_num.le(&pos.line_end()) {
            for (char_num, char) in line.iter().enumerate() {
                if char_num.ge(&pos.pos_start()) && char_num.le(&(pos.pos_end() - 1)) {
                    ret.push(*char);
                }
            }
        }
    }
    ret
}

fn form_file_name(dir: &String, arg: &args::Arg, level: &Level) -> String {
    let path = std::path::Path::new(dir);
    let file_suf = arg.file_suffix();
    format!(
        "{}.{}",
        path.join(level.as_ref()).to_string_lossy(),
        file_suf
    )
}

pub fn save_string_to_file<T>(data: T, level: &Level, arg: &args::Arg) -> Result<(), Box<dyn Error>>
where
    T: AsRef<[u8]>,
{
    let save_path = form_file_name(&arg.save_path, arg, level);
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .append(false)
        .open(&save_path)?;
    file.write_all(data.as_ref())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{files::search_in_file_dyn, meta::Pos};

    #[test]
    fn test_walk_file_dyn() {
        let data = r#"Hello,
December is a last month in the year
Best month in year is "February"
All gifts are gone
"#;
        let pos = Pos {
            typo: crate::meta::Typo::Level,
            start: (1, 19),
            end: (1, 24),
        };
        let result = search_in_file_dyn(data.as_bytes(), &pos);
        assert_eq!("month", unsafe { String::from_utf8_unchecked(result) });

        let pos = Pos {
            typo: crate::meta::Typo::Content,
            start: (2, 23),
            end: (2, 31),
        };

        let result = search_in_file_dyn(data.as_bytes(), &pos);
        assert_eq!("February", unsafe { String::from_utf8_unchecked(result) });
    }
}
