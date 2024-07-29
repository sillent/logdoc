use std::{fs::File, io::Write, path::Path};

use crate::{
    args::{self, SaveType},
    meta::{Level, Meta},
};

pub trait WalkInPosition {
    fn line_start(&self) -> usize;
    fn line_end(&self) -> usize;
    fn pos_start(&self) -> usize;
    fn pos_end(&self) -> usize;
}

pub fn form_list_files(arg: &args::Arg) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut total = vec![];
    if let Some(files) = &arg.files {
        for file in files {
            if std::fs::metadata(&file)?.is_file() {
                total.push(file.clone());
            }
        }
    }
    let recurse = arg.recurse;
    let mut files = list_files_in_dir(&arg.directories(), recurse)?;
    total.append(&mut files);

    Ok(total)
}
fn list_files_in_dir<T>(
    dirs: &Vec<T>,
    recurse: bool,
) -> Result<Vec<String>, Box<dyn std::error::Error>>
where
    T: AsRef<Path>,
{
    let mut files_total = vec![];
    for dir in dirs {
        let mut files = walk_path(dir.as_ref(), recurse)?;
        files_total.append(&mut files);
    }
    Ok(files_total)
}

fn walk_path(
    path: &std::path::Path,
    recurse: bool,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut files = vec![];
    let entries = std::fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        if is_hidden(&entry) {
            continue;
        }
        if entry.file_type()?.is_symlink() {
            continue;
        }
        if entry.file_type()?.is_dir() {
            if recurse {
                let mut files_in_entry = walk_path(path.as_ref(), recurse)?;
                files.append(&mut files_in_entry);
            }
            continue;
        }
        if let Ok(path) = entry.path().into_os_string().into_string() {
            files.push(path);
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

pub fn write_to_file(meta: Meta, arg: &args::Arg) -> Result<(), Box<dyn std::error::Error>> {
    let save_path = form_file_name(&arg.save_path, arg, &meta.level);

    let mut file = create_new(&save_path, &arg, &meta)?;
    if arg.save_type == SaveType::MD {
        write_description(&mut file, &arg, &meta)?;
        write_markdown_table_header(&mut file, &arg)?;
        write_markdown_data(&mut file, &meta)?;
    }

    Ok(())
}

fn form_file_name(dir: &String, arg: &args::Arg, level: &Level) -> String {
    let path = std::path::Path::new(dir);
    return match level {
        Level::Info => format!(
            "{}.{}",
            path.join("info").display().to_string(),
            arg.file_suffix()
        ),
        Level::Debug => format!(
            "{}.{}",
            path.join("debug").display().to_string(),
            arg.file_suffix()
        ),
        Level::Trace => format!(
            "{}.{}",
            path.join("trace").display().to_string(),
            arg.file_suffix()
        ),
        Level::Warn => format!(
            "{}.{}",
            path.join("warn").display().to_string(),
            arg.file_suffix()
        ),
        Level::Fatal => format!(
            "{}.{}",
            path.join("fatal").display().to_string(),
            arg.file_suffix()
        ),
    };
}

fn create_new(
    path: &String,
    arg: &args::Arg,
    meta: &Meta,
) -> Result<std::fs::File, Box<dyn std::error::Error>> {
    let project = arg.project_name.clone();
    let mut file = std::fs::File::create(path)?;
    file.write(format!("# {} - {} logs\n\n", project, meta.level).as_bytes())?;
    Ok(file)
}

fn write_description(
    mut file: &File,
    arg: &args::Arg,
    meta: &Meta,
) -> Result<(), Box<dyn std::error::Error>> {
    match meta.level {
        Level::Info => {
            if let Some(ref desc) = arg.info_desc {
                if desc.len() > 1 {
                    write_file_to_end(&mut file, desc)?;
                }
            }
        }
        Level::Debug => {
            if let Some(ref desc) = arg.debug_desc {
                if desc.len() > 1 {
                    write_file_to_end(&mut file, desc)?;
                }
            }
        }
        Level::Trace => {
            if let Some(ref desc) = arg.trace_desc {
                if desc.len() > 1 {
                    write_file_to_end(&mut file, desc)?;
                }
            }
        }
        Level::Warn => {
            if let Some(ref desc) = arg.warn_desc {
                if desc.len() > 1 {
                    write_file_to_end(&mut file, desc)?;
                }
            }
        }
        Level::Fatal => {
            if let Some(ref desc) = arg.fatal_desc {
                if desc.len() > 1 {
                    write_file_to_end(&mut file, desc)?;
                }
            }
        }
    }

    Ok(())
}

fn write_file_to_end(
    mut file: &std::fs::File,
    data: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = format!("{data}\n\n");
    file.write(data.as_bytes())?;
    Ok(())
}

fn write_markdown_table_header(
    mut file: &File,
    arg: &args::Arg,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut msg_header = "error message";
    let mut subject_header = "subject";
    let mut description_header = "description";
    if let Some(ref msg) = arg.message_table_header {
        msg_header = msg;
    }
    if let Some(ref subj) = arg.subject_table_header {
        subject_header = subj;
    }
    if let Some(ref desc) = arg.description_table_header {
        description_header = desc;
    }
    let data = format!(
        "|{}|{}|{}|\n|---|---|---|\n",
        msg_header, subject_header, description_header
    );
    file.write(data.as_bytes())?;
    Ok(())
}

fn write_markdown_data(mut file: &File, meta: &Meta) -> Result<(), Box<dyn std::error::Error>> {
    let message = meta.message.format();
    let subject = meta.subject.format();
    let description = meta.description.format();
    let data = format!("|{}|{}|{}|\n", message, subject, description);
    file.write(data.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{files::search_in_file_dyn, meta::Pos};

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
