use rand::distributions::{Distribution, Uniform};
use crate::AddMinusOpts;
use crate::read::{write};

pub fn gen_arithmetic(args: &AddMinusOpts) {
    // let mut line: Vec<String> = vec![];
    let mut lines = String::new();
    let add_only = add_only(args.category);
    let minus_only = minus_only(args.category);
    for i in 1 ..= args.count {
        // 指定或随机生成加法
        if add_only || (!minus_only && rand::random()) {
            lines.push_str(&gen_add(args.number_min_inclusive, args.number_max_inclusive));
        } else {
            lines.push_str(&gen_minus(args.number_min_inclusive, args.number_max_inclusive, args.allow_minus_result));
        }

        if i == args.count {
            break;
        }

        if i % args.column_per_page == 0 {
            lines.push_str("\n\n");
        } else {
            lines.push_str("    ");
        }
    }
    write(&lines.trim_end(), "./output/add-minus.txt").expect("Write error!");
    println!("Generate Add/Minus successfully")
}

fn add_only(category: char) -> bool {
    category == '+'
}
fn minus_only(category: char) -> bool {
    category == '-'
}

fn gen_add(min: u16, max: u16) -> String {
    let pair = gen_random(min, max, |_| true);
    format!("{:>2} + {:<2}=", pair.0, pair.1)
}
fn gen_minus(min: u16, max: u16, allow_result_minus: bool) -> String {
    let pair = gen_random(min, max, |p| {
        if !allow_result_minus {
            return p.0 >= p.1;
        }
        true
    });
    format!("{:>2} - {:<2}=", pair.0, pair.1)
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

#[cfg(test)]
mod test{
    use crate::add_minus::{gen_add, gen_arithmetic, gen_minus};
    use crate::{AddMinusOpts};

    #[test]
    fn test_gen_arithmetic() {
        let args = AddMinusOpts {
            count: 40,
            column_per_page: 2,
            number_min_inclusive: 0,
            number_max_inclusive: 10,
            allow_minus_result: false,
            category: '+'
        };
        gen_arithmetic(&args);
    }

    #[test]
    fn test_gen_add() {
        let s = gen_add(0, 50);
        println!("{:?}", s);
        let s = gen_minus(0, 50, true);
        println!("{:?}", s);
        let s = gen_minus(0, 50, false);
        println!("{:?}", s);

        let s = gen_minus(0, 10, true);
        println!("{:?}", s);
        let s = gen_minus(0, 10, false);
        println!("{:?}", s);
    }
}