use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::Range;
use std::path::{Path, PathBuf};
use docx_rs::{Docx, Paragraph, read_docx, Run, RunFonts};
use crate::err::Error;

pub fn read_from_docx(filepath: &str) -> Docx {
    let mut file = File::open(filepath).unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    read_docx(&buf).unwrap()
}

pub fn write_to_docx(docx: Docx, filepath: &str) {
    let path = Path::new(filepath);
    let output_file = File::create(path).unwrap();
    let pack_result = docx.build().pack(output_file);

    match pack_result {
        Ok(_) => println!("Generate docs successfully"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

pub fn add_paragraph(docx: Docx, font_size: usize, text: &str) -> Docx {
    docx.add_paragraph(
        Paragraph::new().size(font_size)
            .add_run(Run::new().size(font_size).fonts(RunFonts::new().ascii("Courier New")).add_text(text)))
}

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

// 数字长度， 也可以转换为string再计算， 但性能更差
pub fn char_len(mut number: u16) -> u16 {
    let mut len = 0;
    if number == 0 {
        len = 1;
    }
    while number != 0 {
        len += 1;
        number /= 10;
    }
    len
}

// number转换为最近的 multiple 的倍数
pub fn round_to(number: u16, multiple: u16, range: &Range<u16>) -> u16 {
    let mut times = number / multiple;
    let r = number % multiple;
    times = if r > multiple / 2 { times + 1 } else { times };
    let mut ans = times * multiple;
    if ans >= range.end {
        ans = (times - 1) * multiple;
    }
    if ans < range.start {
        ans = (times + 1) * multiple;
    }
    ans
}

// 单元测试
// 条件编译：只有执行cargo test时才编译下面的模块
#[cfg(test)]
mod test {
    use std::ops::Range;
    use std::path::PathBuf;
    use crate::utils::{char_len, read, round_to};

    #[test]
    fn test_round_to() {
        assert_eq!(round_to(19, 5, &(1u16..100u16)), 20);
        assert_eq!(round_to(18, 5, &(1u16..100u16)), 20);
        assert_eq!(round_to(17, 5, &(1u16..100u16)), 15);
        assert_eq!(round_to(16, 5, &(1u16..100u16)), 15);
        assert_eq!(round_to(15, 5, &(1u16..100u16)), 15);
    }

    #[test]
    fn test_valid_load_csv() {
        let filename = PathBuf::from("./output/output.txt");
        let csv_data = read(filename);
        assert!(csv_data.is_ok());
    }

    #[test]
    fn test_char_len() {
        assert_eq!(2, char_len(23));
        assert_eq!(1, char_len(0));
        assert_eq!(1, char_len(1));
        assert_eq!(1, char_len(9));
        assert_eq!(2, char_len(10));
        assert_eq!(2, char_len(99));
        assert_eq!(3, char_len(100));
        assert_eq!(5, char_len(10000));
    }
}
