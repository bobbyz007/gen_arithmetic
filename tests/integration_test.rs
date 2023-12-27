// 集成测试, 对于二进制类型crate， 集成需要创建 lib.rs 并导出对应的方法
#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use csv_challenge::{load_csv, replace_column, write_csv};

    #[test]
    fn test_csv_challenge() {

    }
}