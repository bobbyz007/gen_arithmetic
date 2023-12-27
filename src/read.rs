use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use crate::err::Error;

pub fn create_dir_if_necessary(path: &str) {
    match File::open(PathBuf::from(path)) {
        Err(_) => { fs::create_dir_all(path).expect("TODO: panic message"); },
        _ => {}
    }
}

#[allow(dead_code)]
pub fn read(path: PathBuf) -> Result<String, Error> {
    let mut buffer = String::new();
    let mut input_file = open(path)?;
    input_file.read_to_string(&mut buffer)?;
    if buffer.is_empty() {
        return Err("input file missing")?; // ? 配合From 实现自动转换为定制错误Error
    }
    Ok(buffer)
}

#[allow(dead_code)]
pub fn open(path: PathBuf) -> Result<File, Error> {
    let file = File::open(path)?;
    Ok(file)
}

#[allow(dead_code)]
pub fn write(content: &str, filename: &str) -> Result<(), Error> {
    let mut output_file = File::create(filename)?;
    output_file.write_all(content.as_bytes())?;
    Ok(())
}

// 单元测试
// 条件编译：只有执行cargo test时才编译下面的模块
#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use crate::read::read;

    #[test]
    fn test_valid_load_csv() {
        let filename = PathBuf::from("./output/output.txt");
        let csv_data = read(filename);
        assert!(csv_data.is_ok());
    }
}
