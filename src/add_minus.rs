use rand::distributions::{Distribution, Uniform};
use crate::{AddMinusOpts, FOR_ROUND_Number};
use crate::utils::{add_paragraph, char_len, read_from_docx, write, write_to_docx};

pub fn gen_arithmetic_to_docx(args: &AddMinusOpts) {
    let mut doc = read_from_docx("./resources/template.docx");

    let mut line = String::new();
    for i in 1 ..= args.count {
        // 指定或随机生成算式
        line.push_str(&gen_arithmetic_expr(args));

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
fn gen_arithmetic_expr(args: &AddMinusOpts) -> String {
    let c = &args.category;
    if c.starts_with("+") {
        gen_add(args.number_min_inclusive, args.number_max_inclusive, FOR_ROUND_Number.get().unwrap())
    } else if c.starts_with("_") {
        gen_minus(args.number_min_inclusive, args.number_max_inclusive, args.allow_minus_result, FOR_ROUND_Number.get().unwrap())
    } else /*if c.starts_with("x")*/ {
        let is_round = c == "x0";
        if rand::random() {
            gen_add(args.number_min_inclusive, args.number_max_inclusive, FOR_ROUND_Number.get().unwrap())
        } else {
            gen_minus(args.number_min_inclusive, args.number_max_inclusive, args.allow_minus_result, FOR_ROUND_Number.get().unwrap())
        }
    }
}

fn gen_add(min: u16, max: u16, is_round: &bool) -> String {
    let is_valid = |_| true;
    let pair = if *is_round {
        let origin = gen_random(min / 10, max / 10, is_valid);
        (origin.0 * 10, origin.1 * 10)
    } else {
        gen_random(min, max, is_valid)
    };
    let width = char_len(max);
    if width < 2 {
        format!("{:>1} + {:<1}=", pair.0, pair.1)
    } else if width == 2 {
        format!("{:>2} + {:<2}=", pair.0, pair.1)
    } else {
        format!("{:>3} + {:<3}=", pair.0, pair.1)
    }
}
fn gen_minus(min: u16, max: u16, allow_result_minus: bool, is_round: &bool) -> String {
    let is_valid = |p: (u16, u16)| {
        if !allow_result_minus {
            return p.0 >= p.1;
        }
        true
    };
    let pair = if *is_round {
        let origin = gen_random(min / 10, max / 10, is_valid);
        (origin.0 * 10, origin.1 * 10)
    } else {
        gen_random(min, max, is_valid)
    };
    let width = char_len(max);
    if width < 2 {
        format!("{:>1} - {:<1}=", pair.0, pair.1)
    } else if width == 2 {
        format!("{:>2} - {:<2}=", pair.0, pair.1)
    } else {
        format!("{:>3} - {:<3}=", pair.0, pair.1)
    }
}

fn gen_random<F: Fn((u16, u16)) -> bool>(min: u16, max: u16, is_valid: F) -> (u16, u16) {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(min..max + 1);
    loop {
        let pair = (die.sample(&mut rng), die.sample(&mut rng));
        if !is_valid(pair) {  continue }
        return pair;
    }
}

#[allow(dead_code)]
fn gen_arithmetic_to_txt(args: &AddMinusOpts) {
    // let mut line: Vec<String> = vec![];
    let mut lines = String::new();
    for i in 1 ..= args.count {
        // 指定或随机生成加法
        lines.push_str(&gen_arithmetic_expr(args));

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
    use crate::add_minus::{gen_add, gen_arithmetic_to_txt, gen_minus};
    use crate::{AddMinusOpts};

    #[test]
    fn test_gen_arithmetic() {
        let args = AddMinusOpts {
            count: 40,
            column_per_page: 2,
            number_min_inclusive: 0,
            number_max_inclusive: 10,
            allow_minus_result: false,
            category: "+".to_string(),
            output_docx_font_size: 56
        };
        gen_arithmetic_to_txt(&args);
    }

    #[test]
    fn test_gen_add() {
        let s = gen_add(0, 50, &false);
        println!("{:?}", s);
        let s = gen_minus(0, 50, true, &false);
        println!("{:?}", s);
        let s = gen_minus(0, 50, false, &false);
        println!("{:?}", s);

        let s = gen_minus(0, 10, true, &false);
        println!("{:?}", s);
        let s = gen_minus(0, 10, false, &false);
        println!("{:?}", s);
    }
}