use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;
use rand::{rng, Rng};
use rand::distr::Uniform;
use rand::seq::{IteratorRandom, SliceRandom};
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

pub fn gen_arithmetic_to_docx_by_pattern2(args: &AddMinusOpts) {
    let mut map_pair: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();

    map_pair.insert(18, vec![(18, 9)]);
    map_pair.insert(17, vec![(17, 9), (17, 8)]);
    map_pair.insert(16, vec![(16, 9), (16, 8), (16, 7)]);
    map_pair.insert(15, vec![(15, 9), (15, 8), (15, 7), (15, 6)]);
    map_pair.insert(10, vec![(10, 9), (10, 8), (10, 7), (10, 6), (10, 5), (10, 4), (10, 3), (10, 2), (10, 1)]);
    map_pair.insert(8, vec![(8, 7), (8, 6), (8, 5), (8, 4), (8, 3), (8, 2), (8, 1)]);
    gen_arithmetic_to_docx_by_pattern2_3_4(args, &map_pair);
}
pub fn gen_arithmetic_to_docx_by_pattern3(args: &AddMinusOpts) {
    let mut map_pair: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();

    map_pair.insert(14, vec![(14, 9), (14, 8), (14, 7), (14, 6), (14, 5)]);
    map_pair.insert(13, vec![(13, 9), (13, 8), (13, 7), (13, 6), (13, 5), (13, 4)]);
    map_pair.insert(12, vec![(12, 9), (12, 8), (12, 7), (12, 6), (12, 5), (12, 4), (12, 3)]);
    map_pair.insert(11, vec![(11, 9), (11, 8), (11, 7), (11, 6), (11, 5), (11, 4), (11, 3), (11, 2)]);
    gen_arithmetic_to_docx_by_pattern2_3_4(args, &map_pair);
}
pub fn gen_arithmetic_to_docx_by_pattern4(args: &AddMinusOpts) {
    let mut map_pair: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();

    map_pair.insert(9, vec![(9, 8), (9, 7), (9, 6), (9, 5), (9, 4), (9, 3), (9, 2), (9, 1)]);
    map_pair.insert(7, vec![(7, 6), (7, 5), (7, 4), (7, 3), (7, 2), (7, 1)]);
    map_pair.insert(6, vec![(6, 5), (6, 4), (6, 3), (6, 2), (6, 1)]);
    map_pair.insert(5, vec![(5, 4), (5, 3), (5, 2), (5, 1)]);
    map_pair.insert(4, vec![(4, 3), (4, 2), (4, 1)]);

    gen_arithmetic_to_docx_by_pattern2_3_4(args, &map_pair);
}

pub fn gen_arithmetic_to_docx_by_pattern2_3_4(args: &AddMinusOpts, map_pair: &HashMap<u32, Vec<(u32, u32)>>) {
    let mut doc = read_from_docx("./resources/template.docx");

    let mut line = String::new();
    let mut i = 0;

    let mut keys: Vec<_> = map_pair.keys().collect();
    keys.sort_by(|a, b| b.cmp(a));

    while i < args.count {
        let mut result_pairs: Vec<String> = Vec::new();
        for key in &keys {
            for &pair in map_pair.get(key).unwrap().iter() {
                result_pairs.push(format!("{:>2} - {:<2}=", pair.0, pair.1));
            }
        }
        // 打乱顺序
        result_pairs.shuffle(&mut rand::rng());
        for pair in result_pairs.iter() {
            line.push_str(pair);

            i = i + 1;
            if i > args.count { break }
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
    }
    write_to_docx(doc, "./output/add-minus.docx");
}

pub fn gen_arithmetic_to_docx_by_pattern1(args: &AddMinusOpts) {
    let mut doc = read_from_docx("./resources/template.docx");

    let mut line = String::new();
    let mut i = 0;
    const NUMBER_PER_PAGE: u8 = 26;
    let mut map_pair: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    let mut map_freq: HashMap<u32, usize> = HashMap::new();

    map_pair.insert(16, vec![(9, 7), (8, 8)]);
    map_freq.insert(16, 2);

    map_pair.insert(15, vec![(9, 6), (8, 7)]);
    map_freq.insert(15, 2);

    map_pair.insert(14, vec![(9, 5), (8, 6), (7, 7)]);
    map_freq.insert(14, 3);

    map_pair.insert(13, vec![(9, 4), (8, 5), (7, 6)]);
    map_freq.insert(13, 3);

    map_pair.insert(12, vec![(9, 3), (8, 4), (7, 5), (6, 6)]);
    map_freq.insert(12, 4);

    map_pair.insert(11, vec![(9, 2), (8, 3), (7, 4), (6, 5)]);
    map_freq.insert(11, 4);

    map_pair.insert(10, vec![(9, 1), (8, 2), (7, 3), (6, 4), (5, 5)]);
    map_freq.insert(10, 1);

    map_pair.insert(9, vec![(8, 1), (7, 2), (6, 3), (5, 4)]);
    map_freq.insert(9, 2);

    map_pair.insert(8, vec![(7, 1), (6, 2), (5, 3), (4, 4)]);
    map_freq.insert(8, 2);

    map_pair.insert(7, vec![(6, 1), (5, 2), (4, 3), (4, 2), (9, 8)]);
    map_freq.insert(7, 2);

    map_pair.insert(6, vec![(5, 1), (3, 3), (4, 1), (3, 2), (9, 9)]);
    map_freq.insert(6, 1);

    let mut keys: Vec<_> = map_pair.keys().collect();
    keys.sort_by(|a, b| b.cmp(a));

    while i < args.count {
        let mut result_pairs: Vec<String> = Vec::new();
        for key in &keys {
            let pairs = map_pair.get(key).unwrap();
            let &expected_count = map_freq.get(key).unwrap();
            let selected_pairs = pairs.iter().choose_multiple(&mut rand::rng(), expected_count);
            for &pair in selected_pairs {
                // 随机决定前后顺序
                let rand_bool = rand::rng().gen_bool(0.5);
                if rand_bool {
                    result_pairs.push(format!("{:>1} + {:<1}=", pair.0, pair.1));
                } else {
                    result_pairs.push(format!("{:>1} + {:<1}=", pair.1, pair.0));
                }
            }
        }
        // 打乱顺序
        result_pairs.shuffle(&mut rand::rng());
        for pair in result_pairs.iter() {
            line.push_str(pair);

            i = i + 1;
            if i > args.count { break }
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
    }
    write_to_docx(doc, "./output/add-minus.docx");
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
    let c = &args.origin.category;
    if c.starts_with("+") {
        gen_add(args)
    } else if c.starts_with("_") {
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

    let mut rng = rng();
    let uniform = Uniform::new(min, max + 1).unwrap();

    loop {
        let mut l = rng.sample(uniform);
        let mut r = rng.sample(uniform);
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

        if !is_valid((l, r)) {  continue }
        return (l, r);
    }
}

fn parse_number_by_pattern(pattern: &OperandPattern, ans: u16, range: &Range<u16>) -> u16 {
    // let ans = rand::rng().gen_range(min..max + 1);
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
            rng().random_range(r.start..r.end)
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
            category: "+".to_string(),
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
            category: "+".to_string(),
            output_docx_font_size: 56,
            operand_pattern: "*,*".to_string(),
            result_max_inclusive: 99,
        };
        let s = gen_add(&parse_args(&args));
        println!("{:?}", s);
    }
}