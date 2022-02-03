use std::{fs, fs::File, io};

pub struct CFile {
    path: String,
    contents: String
}

impl CFile {
    pub fn new(path: String, contents: String) -> Self {
        CFile {
            path,
            contents
        }
    }
}

struct CPath {
    path: String,
}

impl CPath {
    pub fn new(path: &str) -> Result<Self, io::ErrorKind> {
        let max_index = path.len() - 1;
        let first = path.find(".").unwrap_or_default();
        if first == 0 || first == max_index || first != path.rfind(".").unwrap() {
            Err(io::ErrorKind::InvalidInput)
        } else {
            Ok(CPath {
                path: path.to_string(),
            })
        }
    }
}
pub fn create_file(path: &str) -> Result<File, io::ErrorKind> {
    let path = CPath::new(path)?.path;
    match File::create(path) {
        Ok(file) => Ok(file),
        Err(err) => Err(err.kind()),
    }
}

pub fn write_file(path: &str, contents: &str) -> Result<File, io::ErrorKind>{
    let path = CPath::new(path)?.path;
    fs::write(&path, contents).expect("cannot write the file");
    match File::open(path) {
        Ok(f) => Ok(f),
        Err(err) => Err(err.kind())
    }
}

#[cfg(test)]
mod test {
    use super::{create_file, write_file};

    #[test]
    fn test_create_file_fail() {
        create_file("hello.t.xt").unwrap_err();
        create_file("hello.").unwrap_err();
        create_file("hello").unwrap_err();
    }

    #[test]
    fn test_create_file_s1() {
        create_file("hello.txt").unwrap();
    }

    #[test]
    fn test_write_file_fail() {
        write_file("hello.t.xt", "hello world").unwrap_err();
    }
}
