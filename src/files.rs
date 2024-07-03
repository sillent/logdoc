pub struct FileList;

impl FileList {
    pub fn walk<T>(paths: Vec<T>) -> FileList
    where
        T: AsRef<str>,
    {
        Self
    }
}

#[cfg(test)]
mod tests {}
