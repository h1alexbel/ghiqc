use crate::args::ignore_facts::parse_facts;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Ignore file.
/// `name` Ignore file name
#[derive(Clone)]
pub struct IgnoreFile {
    path: String,
}

impl IgnoreFile {
    /// New ignore file.
    pub fn new(path: String) -> IgnoreFile {
        IgnoreFile { path }
    }

    /// Is file exists?
    pub fn exists(&self) -> bool {
        Path::new(&self.path).exists()
    }

    /// Facts.
    pub fn facts(&self) -> HashMap<String, Vec<String>> {
        match File::open(Path::new(&self.path.clone())) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let facts: Vec<String> =
                    reader.lines().filter_map(Result::ok).collect();
                parse_facts(facts)
            }
            Err(err) => {
                panic!("Can not read facts from {}, due to: {}", self.path, err)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::args::ignore_file::IgnoreFile;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};
    use std::fs::File;
    use tempdir::TempDir;

    #[test]
    fn creates_new_ignore_file() -> Result<()> {
        let temp = TempDir::new("temp")?;
        let name = "ignore.ghiqc";
        let path = temp.path().join(name);
        File::create(path.clone()).expect("Can not create a file");
        let ignore = IgnoreFile::new(String::from(
            path.to_str().expect("Path does not exists"),
        ));
        assert_that!(ignore.path.is_empty(), is(equal_to(false)));
        Ok(())
    }

    #[test]
    fn returns_true_if_exists() -> Result<()> {
        let temp = TempDir::new("temp")?;
        let name = "ignore.ghiqc";
        let path = temp.path().join(name);
        File::create(path.clone()).expect("Can not create a file");
        let ignore = IgnoreFile::new(String::from(
            path.to_str().expect("Path does not exists"),
        ));
        assert_that!(ignore.exists(), is(equal_to(true)));
        Ok(())
    }

    #[test]
    fn returns_false_if_absent() -> Result<()> {
        let temp = TempDir::new("temp")?;
        let name = "ignore.ghiqc";
        let path = temp.path().join(name);
        let ignore = IgnoreFile::new(String::from(
            path.to_str().expect("Path does not exists"),
        ));
        assert_that!(ignore.exists(), is(equal_to(false)));
        Ok(())
    }
}
