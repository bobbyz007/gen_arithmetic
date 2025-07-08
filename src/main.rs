mod err;
mod utils;
mod add_minus;
mod missing_number;

use std::ops::Range;
use clap::{Args, Parser, Subcommand};
use crate::add_minus::{gen_arithmetic_to_docx, gen_arithmetic_to_docx_by_pattern1, gen_arithmetic_to_docx_by_pattern2, gen_arithmetic_to_docx_by_pattern3, gen_arithmetic_to_docx_by_pattern4};
use crate::utils::{create_dir_if_necessary};

// 全局初始化一次的变量
// static OPERAND_PATTERN: OnceLock<&str> = OnceLock::new();

fn main() {
    init();

    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::AddMinus(add_minus)) => {
            if add_minus.category.ends_with("p1") {
                // p1: add(result [6, 16])
                gen_arithmetic_to_docx_by_pattern1(add_minus)
            } else if add_minus.category.ends_with("p2") {
                // p2: minus(start with 8, 10, 15~18)
                gen_arithmetic_to_docx_by_pattern2(add_minus)
            } else if add_minus.category.ends_with("p3") {
                // p3: minus(start with 11~14)
                gen_arithmetic_to_docx_by_pattern3(add_minus)
            } else if add_minus.category.ends_with("p4") {
                // p4: minus(start with 4~9)
                gen_arithmetic_to_docx_by_pattern4(add_minus)
            } else {
                gen_arithmetic_to_docx(add_minus);
            }
        },
        Some(Commands::MissingNumber(missing_number)) => {
            missing_number.gen_missing_numbers_to_docx();
        },
        None => {}
    }
}

fn init() {
    create_dir_if_necessary("./output");
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
    // p1: 定制的模式
    #[arg(short, long, )]
    category: String,

    // 左/右操作数可以指定如下模式：
    // 受-l, -r参数约束：*表示范围内的任意数，C*表示数字是C的倍数。
    // 不受-l, -r参数约束：C~D表示[C,D]范围内的任意数，C表示指定的常数。 这两种相当于常数指定

    // L, R分别表示左右操作数
    // 1. L,R
    // 2. L  ：实际是L,L的简化，表示左右操作数相同
    // 3. =A 表示满足运算结果为A，此处A仅支持指定常数

    // 举例：
    // 10,*: 左操作数固定为10， 右操作数是范围内的任意随机数
    // 10*,5*：左操作数是10的倍数， 右操作数是5的倍数
    // *：左右操作数相同，是范围内的任意随机数
    // 5*：左右操作数相同，是5的倍数
    // =10：满足运算结果等于10
    #[arg(short='p', long, allow_hyphen_values=true, default_value="*,*")]
    operand_pattern: String,

    // 参与运算的数的范围最小值，默认是0
    #[arg(short='l', long, default_value_t=0)]
    number_min_inclusive: u16,

    // 参与运算的数的范围最大值
    #[arg(short='r', long, default_value_t=10)]
    number_max_inclusive: u16,

    // 允许的运算结果最小值，默认是0
    #[arg(short='b', long, default_value_t=0)]
    result_min_inclusive: i16,

    // 允许的运算结果最大值，默认是99
    #[arg(short='e', long, default_value_t = 99)]
    result_max_inclusive: i16,

    // 写入到docx中的字体大小
    #[arg(short='f', long, default_value_t = 56)]
    output_docx_font_size: u16,
}

// 操作数配置
enum OperandConfig {
    TwoOperand(OperandPattern, OperandPattern), // L,R
    OneOperand(OperandPattern), // L，是L,L简化
    Result(u16) // =A
}
enum OperandPattern {
    Wildcard,
    NumberWildcard(u16),
    Constant(u16),
    ConstantRange(Range<u16>)
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
    #[arg(short, long, allow_negative_numbers=true, default_value_t=1)]
    step: i16,

    // 随机产生数据起始是step的倍数，比如step是5， 则10,15,20符合, 11, 16, 21不符合，因为11不是5的倍数
    #[arg(short='t', long, default_value_t=false)]
    start_as_multiple_step: bool,

    // 一行多少个char，默认37
    #[arg(short='w', long, default_value_t=37)]
    line_width: u16,

    // 参与数的范围最小值，默认是0
    #[arg(short='l', long, default_value_t=0)]
    number_min_inclusive: u16,

    // 参与数的范围最大值，此处只是一个参考值，允许略微超过此上限
    #[arg(short='r', long, default_value_t=100)]
    number_max_inclusive: u16,

    // 写入到docx中的字体大小，需要与 line_width 配合，字体太大，则line_width需减少，否则一行容纳不下
    #[arg(short='f', long, default_value_t = 36)]
    output_docx_font_size: u16,
}

