use std::path::PathBuf;

pub struct FileList;

impl FileList {
    pub fn walk<T>(paths: Vec<T>) -> FileList
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
    // walkdir(, )
    if let Some(dirs) = dirs {
        for dir in dirs {
            // TODO: unwrap behavior
            walkdir(&PathBuf::from(dir), &mut res, recurse).unwrap();
        }
    }
    if let Some(files) = files {
        files.iter().map(|x| res.push(x.clone())).count();
    }
    return res;
}

fn walkdir(
    path: &std::path::PathBuf,
    result: &mut Vec<String>,
    recurse: bool,
) -> Result<(), std::io::Error> {
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if recurse {
                walkdir(&path, result, recurse)?
            }
        } else if path.is_file() {
            let s = path.into_os_string().into_string().unwrap();
            result.push(s);
        }
    }
    Ok(())
}
fn walk_dir(path: &String, recurse: bool) -> Vec<String> {
    let mut result = vec![];
    // if let Ok(dir) = std::fs::read_dir(path) {
    //     // if std::
    //     return result;
    // }
    std::fs::read_dir(path).map(|de| de.map(|res| res.map(|entry| entry.file_type())));
    result
}

#[cfg(test)]
mod tests {

    #[test]
    fn proceed_return_vec_of_all_files() {}
}
