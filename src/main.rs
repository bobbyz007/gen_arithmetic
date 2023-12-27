mod err;
mod read;
mod add_minus;

use clap::{Args, Parser, Subcommand};
use crate::add_minus::gen_arithmetic;
use crate::read::{create_dir_if_necessary};

fn main() {
    create_dir_if_necessary("./output");

    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::AddMinus(add_minus)) => {
            gen_arithmetic(add_minus);
        },
        Some(Commands::MissingNumber(missing_number)) => {
            println!("{:?}", missing_number);
        },
        None => {}
    }
}

// 利用clap处理命令行参数
#[derive(Debug, Parser)]
#[command(author,version)]
#[command(arg_required_else_help(true))] // 没有参数的时候打印帮助
#[command(about = "arithmetic - a simple CLI to auto-generate arithmetic expression")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

}
#[derive(Subcommand, Debug)]
enum Commands {
    /// 加减法
    AddMinus(AddMinus),
    /// 补充缺失的数字
    MissingNumber(MissingNumber),
}

#[derive(Args, Debug)]
struct AddMinus {
    // 生成多少个算式，默认40个
    #[arg(short='n', long, default_value_t=40)]
    count: u16,

    // 每页多少列，默认2列
    #[arg(short='o', long, default_value_t=2)]
    column_per_page: u8,

    // 类别：+： 全部加法， '-': 全部减法， 其他任何: 随机混合加减法
    #[arg(short, long, default_value_t='+')]
    category: char,

    // 参与运算的数的范围最小值，默认是0
    #[arg(short='l', long, default_value_t=0)]
    number_min_inclusive: u16,

    // 参与运算的数的范围最大值
    #[arg(short='r', long, default_value_t = 10)]
    number_max_inclusive: u16,

    // 允许负数结果，默认 false
    #[arg(short, long, default_value_t=false)]
    allow_minus_result: bool,
}

#[derive(Args, Debug)]
struct MissingNumber {
    // 一个gap包括的最大的缺失数字， 默认3
    #[arg(short, long, default_value_t=3)]
    miss_max_per_gap: u8,

    // 一行多少个gap，默认2个
    #[arg(short, long, default_value_t=2)]
    gaps_per_line: u8,

    // 一行多少个数字，默认20
    #[arg(short, long, default_value_t=20)]
    number_count_per_line: u8,

    // 参与数的范围最小值，默认是0
    #[arg(short='l', long, default_value_t=0)]
    number_min_inclusive: u16,

    // 参与数的范围最大值
    #[arg(short='r', long, default_value_t=100)]
    number_max_inclusive: u16,

}
