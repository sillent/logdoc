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
    files: Option<Vec<String>>,
    dirs: Option<Vec<String>>,
    recurse: bool,
) -> Vec<String> {
    // let mut result_vec = Vec::new();
    unimplemented!("implement this");
}

#[cfg(test)]
mod tests {

    #[test]
    fn proceed_return_vec_of_all_files() {}
}
