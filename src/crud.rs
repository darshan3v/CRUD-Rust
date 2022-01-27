use std::{fs::File, io};

struct CPath {
    path: String,
}

impl CPath {
    pub fn new(path: &str) -> Result<Self, io::ErrorKind> {
        let n = path.len();
        let i = path.find(".").unwrap_or_default();
        if i == 0 || i == n - 1 || i != path.rfind(".").unwrap() {
            Err(io::ErrorKind::InvalidInput)
        } else {
            Ok(CPath {
                path: path.to_string(),
            })
        }
    }
}

pub fn create_file(file_name: &str) -> Result<File, io::ErrorKind> {
    let path = CPath::new(file_name)?;
    match File::create(path.path) {
        Ok(file) => Ok(file),
        Err(err) => Err(err.kind()),
    }
}

#[cfg(test)]
mod test {
    use super::create_file;

    #[test]
    #[should_panic]
    fn test_create_file_f1() {
        // f1 is fail - 1
        create_file("hello.t.xt").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_create_file_f2() {
        create_file("hello.").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_create_file_f3() {
        create_file("hello").unwrap();
    }

    #[test]
    fn test_create_file_s1() {
        create_file("hello.txt").unwrap();
    }
}
