use std::ops::Range;
use std::str::FromStr;
use rand::distributions::{Distribution, Uniform};
use rand::{Rng, thread_rng};
use crate::{AddMinusOpts, OperandConfig, OperandPattern, utils};
use crate::utils::{add_paragraph, char_len, read_from_docx, write, write_to_docx};

struct ParsedArgs<'a>{
    // 原始命令行参数
    origin: &'a AddMinusOpts,
    // 解析后的操作数配置
    operand_config: OperandConfig,
}

fn parse_args(args: &AddMinusOpts) -> ParsedArgs {
    let mut parsed_args = ParsedArgs { origin: args, operand_config: OperandConfig::Result(0) };

    let p_str = &args.operand_pattern;
    let patterns: Vec<&str> = p_str.split(",").collect();
    let mut operand_config;
    if patterns.len() == 2 {
        // L,R
        operand_config = OperandConfig::TwoOperand(
            parse_operand_pattern(patterns[0]), parse_operand_pattern(patterns[1]));
    } else if patterns[0].starts_with("=") {
        // =A
        operand_config = OperandConfig::Result(u16::from_str(&patterns[0][1..]).unwrap());
    } else {
        // L,L
        operand_config = OperandConfig::OneOperand(parse_operand_pattern(patterns[0]))
    }
    parsed_args.operand_config = operand_config;
    parsed_args
}

fn parse_operand_pattern(pattern: &str) -> OperandPattern {
    if pattern == "*" {
        OperandPattern::Wildcard
    } else if pattern.ends_with("*") {
        let p = &pattern[..pattern.len() - 1];
        let number = u16::from_str(p).unwrap();
        OperandPattern::NumberWildcard(number)
    } else if pattern.contains("~") || pattern.contains("-") {
        // 忽略ans，指定常数范围不受min~max范围限制
        let range: Vec<&str> = pattern.split(&['~','-']).collect();
        OperandPattern::ConstantRange(range[0].parse::<u16>().unwrap()..u16::from_str(range[1]).unwrap() + 1)
    } else {
        // 忽略ans，指定常数不受min~max范围限制
        OperandPattern::Constant(u16::from_str(pattern).unwrap())
    }
}

pub fn gen_arithmetic_to_docx(args: &AddMinusOpts) {
    let parsed_args = parse_args(args);
    let mut doc = read_from_docx("./resources/template.docx");

    let mut line = String::new();
    for i in 1 ..= args.count {
        // 指定或随机生成算式
        line.push_str(&gen_arithmetic_expr(&parsed_args));

        // 可能多个算式组合成一行，写入docx
        if i % args.column_per_page == 0 || i == args.count {
            // write paragraph
            doc = add_paragraph(doc, args.output_docx_font_size as usize, &line);

            line.clear();
        } else {
            // 算式之间添加分隔
            line.push_str("      ");
        }
    }
    write_to_docx(doc, "./output/add-minus.docx");
}

// 根据指定条件或随机生成算式
fn gen_arithmetic_expr(args: &ParsedArgs) -> String {
    let c = args.origin.category;
    if c == '+' {
        gen_add(args)
    } else if c == '-' {
        gen_minus(args)
    } else /*if c == 'x'*/ {
        if rand::random() {
            gen_add(args)
        } else {
            gen_minus(args)
        }
    }
}

enum Op {
    Add, Minus, Mul, Div
}

fn gen_add(args: &ParsedArgs) -> String {
    let is_valid = |p: (u16, u16)| {
        let ans = p.0 as i16 + p.1 as i16;
        args.origin.result_min_inclusive <= ans && ans <= args.origin.result_max_inclusive
    };
    let pair = gen_operands(args, Op::Add, is_valid);
    let width = char_len(args.origin.number_max_inclusive);
    if width < 2 {
        format!("{:>1} + {:<1}=", pair.0, pair.1)
    } else if width == 2 {
        format!("{:>2} + {:<2}=", pair.0, pair.1)
    } else {
        format!("{:>3} + {:<3}=", pair.0, pair.1)
    }
}
fn gen_minus(args: &ParsedArgs) -> String {
    let is_valid = |p: (u16, u16)| {
        let ans = p.0 as i16 - p.1 as i16;
        args.origin.result_min_inclusive <= ans && ans <= args.origin.result_max_inclusive
    };
    let pair = gen_operands(args, Op::Minus, is_valid);
    let width = char_len(args.origin.number_max_inclusive);
    if width < 2 {
        format!("{:>1} - {:<1}=", pair.0, pair.1)
    } else if width == 2 {
        format!("{:>2} - {:<2}=", pair.0, pair.1)
    } else {
        format!("{:>3} - {:<3}=", pair.0, pair.1)
    }
}

fn gen_operands<F: Fn((u16, u16)) -> bool>(args: &ParsedArgs, op: Op, is_valid: F) -> (u16, u16) {
    let min = args.origin.number_min_inclusive;
    let max = args.origin.number_max_inclusive;
    let range = min..max + 1;

    let mut rng = thread_rng();
    let die = Uniform::from(min..max+1);

    loop {
        let mut l = die.sample(&mut rng);
        let mut r = die.sample(&mut rng);
        match &args.operand_config {
            OperandConfig::TwoOperand(pattern_l, pattern_r) => {
                // L, R
                l = parse_number_by_pattern(pattern_l, l, &range);
                r = parse_number_by_pattern(pattern_r, r, &range);
            }
            OperandConfig::OneOperand(pattern_l) => {
                // L,L
                l = parse_number_by_pattern(pattern_l, l, &range);
                r = l;
            }
            OperandConfig::Result(ans) => {
                // =A
                match op {
                    // min<= r=ans-l <=max
                    Op::Add => {
                        if l + max < *ans || l + min > *ans { continue }
                        r = ans - l
                    },
                    // min <= r=l-ans <= max
                    Op::Minus => {
                        if ans + max < l || ans + min > l  { continue }
                        r = l - ans;
                    }
                    _ => {
                    }
                }
            }
        }

        // let pair = (die.sample(&mut rng), die.sample(&mut rng));
        if !is_valid((l, r)) {  continue }
        return (l, r);
    }
}

fn parse_number_by_pattern(pattern: &OperandPattern, ans: u16, range: &Range<u16>) -> u16 {
    // let ans = rand::thread_rng().gen_range(min..max + 1);
    match pattern {
        OperandPattern::Wildcard => { ans }
        OperandPattern::NumberWildcard(number) => {
            if ans % number == 0 {
                ans
            } else {
                utils::round_to(ans, *number, range)
            }
        }
        OperandPattern::Constant(c) => {
            //忽略ans, 指定常数不受min~max范围限制
            *c
        }
        OperandPattern::ConstantRange(r) => {
            //忽略ans, 指定常数范围不受min~max范围限制
            thread_rng().gen_range(r.start..r.end)
        }
    }
}

#[allow(dead_code)]
fn gen_arithmetic_to_txt(args: &AddMinusOpts) {
    // let mut line: Vec<String> = vec![];
    let mut lines = String::new();
    for i in 1 ..= args.count {
        // 指定或随机生成加法
        lines.push_str(&gen_arithmetic_expr(&parse_args(args)));

        if i == args.count {
            break;
        }

        if i % args.column_per_page == 0 {
            lines.push_str("\n");
        } else {
            lines.push_str("      ");
        }
    }
    write(&lines.trim_end(), "./output/add-minus.txt").expect("Write error!");
    println!("Generate Add/Minus successfully")
}

#[cfg(test)]
mod test{
    use crate::add_minus::{gen_add, gen_arithmetic_to_txt, parse_args};
    use crate::{AddMinusOpts};

    #[test]
    fn test_gen_arithmetic() {
        let args = AddMinusOpts {
            count: 40,
            column_per_page: 2,
            number_min_inclusive: 0,
            number_max_inclusive: 10,
            result_min_inclusive: 0,
            category: '+',
            output_docx_font_size: 56,
            operand_pattern: "*,*".to_string(),
            result_max_inclusive: 99,
        };
        gen_arithmetic_to_txt(&args);
    }

    #[test]
    fn test_gen_add() {
        let args = AddMinusOpts {
            count: 40,
            column_per_page: 2,
            number_min_inclusive: 0,
            number_max_inclusive: 10,
            result_min_inclusive: 0,
            category: '+',
            output_docx_font_size: 56,
            operand_pattern: "*,*".to_string(),
            result_max_inclusive: 99,
        };
        let s = gen_add(&parse_args(&args));
        println!("{:?}", s);
    }
}