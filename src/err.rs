use std::io;

// 定制错误
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Program(&'static str),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
impl From<&'static str> for Error {
    fn from(e: &'static str) -> Self {
        Error::Program(e)
    }
}

#[cfg(test)]
mod test{
    use std::fs::File;
    use crate::err::Error;

    #[test]
    fn test_str_convert() {
        let e: Error = "error in program".into();
        let e2 = <&str>::into("another error in program");
        match e {
            Error::Program(d) => assert_eq!(d, "error in program"),
            _ => ()
        }
        match e2 {
            Error::Program(d) => assert_eq!(d, "another error in program"),
            _ => ()
        }
    }

    #[test]
    fn test_io_error_convert() {
        let file = File::open("non exist");
        assert!(!file.is_ok());
        match file {
            Err(e) => {
                let custom_error: Error = e.into();
                let msg = format!("{:?}", custom_error);
                assert!(msg.eq("Io(Os { code: 2, kind: NotFound, message: \"No such file or directory\" })"));
            },
            _ => (),
        }
    }
}
