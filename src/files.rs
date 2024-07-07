use std::path::PathBuf;

pub(crate) struct FileList;

impl FileList {
    pub(crate) fn walk<T>(paths: Vec<T>) -> FileList
    where
        T: AsRef<str>,
    {
        Self
    }
}

pub(crate) fn proceed(
    files: &Option<Vec<String>>,
    dirs: &Option<Vec<String>>,
    recurse: bool,
) -> Vec<String> {
    let mut res = vec![];
    if let Some(dirs) = dirs {
        for dir in dirs {
            walkdir(&PathBuf::from(dir), &mut res, recurse);
        }
    }
    if let Some(files) = files {
        files.iter().map(|x| res.push(x.clone())).count();
    }
    return res;
}

fn walkdir(path: &std::path::PathBuf, result: &mut Vec<String>, recurse: bool) {
    if let Ok(entry) = std::fs::read_dir(path) {
        for e in entry {
            if let Ok(e) = e {
                let path = e.path();
                if path.is_dir() {
                    walkdir(&path, result, recurse);
                } else if path.is_file() {
                    result.push(path.into_os_string().into_string().unwrap());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn proceed_return_vec_of_all_files() {}
}
