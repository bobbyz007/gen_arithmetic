mod err;
mod utils;
mod add_minus;
mod missing_number;

use std::sync::OnceLock;
use clap::{Args, Parser, Subcommand};
use crate::add_minus::{gen_arithmetic_to_docx};
use crate::utils::{create_dir_if_necessary};

// 全局初始化一次的变量
static FOR_ROUND_Number: OnceLock<bool> = OnceLock::new();

fn main() {
    create_dir_if_necessary("./output");

    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::AddMinus(add_minus)) => {
            FOR_ROUND_Number.get_or_init(|| {
                add_minus.category.ends_with("0")
            });
            gen_arithmetic_to_docx(add_minus);
        },
        Some(Commands::MissingNumber(missing_number)) => {
            missing_number.gen_missing_numbers_to_docx();
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
    AddMinus(AddMinusOpts),
    /// 补充缺失的数字
    MissingNumber(MissingNumberOpts),
}

#[derive(Args, Debug)]
struct AddMinusOpts {
    // 生成多少个算式，默认40个
    #[arg(short='n', long, default_value_t=40)]
    count: u16,

    // 每页多少列，默认2列
    #[arg(short='o', long, default_value_t=2)]
    column_per_page: u16,

    // 类别有如下
    // +： 全部加法，
    // +0： 整十加法
    // _: 全部减法， - 与命令行符号冲突，选择_
    // _0: 整十减法
    // 其他任何: 随机混合加减法
    #[arg(short, long, )]
    category: String,

    // 参与运算的数的范围最小值，默认是0
    #[arg(short='l', long, default_value_t=0)]
    number_min_inclusive: u16,

    // 参与运算的数的范围最大值
    #[arg(short='r', long, default_value_t = 10)]
    number_max_inclusive: u16,

    // 允许负数结果，默认 false
    #[arg(short, long, default_value_t=false)]
    allow_minus_result: bool,

    // 写入到docx中的字体大小
    #[arg(short='f', long, default_value_t = 56)]
    output_docx_font_size: u16,
}

#[derive(Args, Debug)]
struct MissingNumberOpts {
    // 生成多少个，默认10个
    #[arg(short='n', long, default_value_t=10)]
    count: u16,

    // 一个gap包括的最大的缺失个数， 默认3
    #[arg(short, long, default_value_t=3)]
    miss_max_per_gap: u16,

    // 一行多少个gap，默认2个
    #[arg(short, long, default_value_t=2)]
    gaps_per_line: u16,

    // 递进
    #[arg(short, long, default_value_t=1)]
    step: u16,

    // 一行多少个char，默认38
    #[arg(short='w', long, default_value_t=37)]
    line_width: u16,

    // 参与数的范围最小值，默认是0
    #[arg(short='l', long, default_value_t=0)]
    number_min_inclusive: u16,

    // 参与数的范围最大值
    #[arg(short='r', long, default_value_t=100)]
    number_max_inclusive: u16,

    // 写入到docx中的字体大小，需要与 line_width 配合，字体太大，则line_width需减少，否则一行容纳不下
    #[arg(short='f', long, default_value_t = 36)]
    output_docx_font_size: u16,
}

