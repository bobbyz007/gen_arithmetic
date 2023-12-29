use std::cmp::min;
use docx_rs::{Paragraph};
use rand::distributions::{Distribution, Uniform};
use rand::{Rng, thread_rng};
use crate::MissingNumberOpts;
use crate::utils::{add_paragraph, read_from_docx, write, write_to_docx};

const FONT_SIZE: usize = 36;
impl MissingNumberOpts {
    pub fn gen_missing_numbers_to_docx(&self) {
        let mut doc = read_from_docx("./resources/template.docx");

        for _i in 0..self.count {
            let line = &self.gen_single_missing_numbers();
            doc = add_paragraph(doc, FONT_SIZE, line);
            doc = doc.add_paragraph(Paragraph::new().size(FONT_SIZE))
        }

        write_to_docx(doc, "./output/missing-numbers.docx");
    }

    #[allow(dead_code)]
    pub fn gen_missing_numbers_to_txt(&self) {
        let mut lines = String::new();
        for _i in 0..self.count {
            lines.push_str(&self.gen_single_missing_numbers());
            lines.push_str("\n");
        }
        write(&lines.trim(), "./output/missing-numbers.txt").expect("Write error!");
        println!("Generate missing numbers successfully");
    }

    fn gen_single_missing_numbers(&self) -> String {
        let mut gaps = vec![];
        self.gen_gaps(&mut gaps);
        let min_numbers = gaps.iter().fold(0, |acc, &x| acc + x);

        // 生成数字
        let mut numbers: Vec<u16> = vec![];
        self.gen_numbers(&mut numbers, min_numbers + gaps.len() as u16 - 1);

        let mut line = String::new();
        // numbers中的插入gap的索引位置
        let mut number_pos: u16 = 0;
        let mut miss_numbers = min_numbers + gaps.len() as u16 - 1;
        // 在数字序列中插入gap
        for gap in gaps {
            // 随机数的范围
            let upper_bound = numbers.len() as u16 - miss_numbers + 1;
            let gap_start = thread_rng().gen_range(number_pos..upper_bound);

            // 填入数字
            for i in number_pos..gap_start {
                line.push_str(&numbers[i as usize].to_string());
                line.push(' ');
            }
            // 填入missing
            for i in gap_start..gap_start + gap {
                line.push_str(&"_".repeat(char_len(numbers[i as usize]) as usize));
                line.push(' ');
            }

            // 填入gap的间隔数字
            if gap_start + gap < numbers.len() as u16 {
                line.push_str(&numbers[(gap_start + gap) as usize].to_string());
                line.push(' ');
            }

            number_pos = gap_start + gap + 1;
            // 还剩余至少2个gap
            if miss_numbers > gap {
                miss_numbers -= gap + 1;
            }
        }
        // 填入剩余的number数字
        for i in number_pos..numbers.len() as u16 {
            line.push_str(&numbers[i as usize].to_string());
            line.push(' ');
        }
        line.trim().to_string()
    }

    // 根据行宽，确定数字序列
    fn gen_numbers(&self, numbers: &mut Vec<u16>, min_numbers: u16) {
        let mut upper_bound: u16 = self.number_max_inclusive - min_numbers * self.step;

        let mut number = self.number_max_inclusive;
        let mut width = char_len(number);
        while width <= self.line_width {
            number -= self.step;
            // 数字和间隔空格的宽度
            width += char_len(number) + 1;
        }
        // 既满足step要求， 又满足line width的要求
        upper_bound = min(upper_bound, number);
        if upper_bound < self.number_min_inclusive {
            panic!("Please shorten step or increase line width");
        }

        let mut rng = rand::thread_rng();
        let die = Uniform::from(self.number_min_inclusive..upper_bound);
        // 从start开始满足gap要求，每个gap都至少间隔了一个数字，而且line width不会超过数字范围
        let start = die.sample(&mut rng);

        // 从start开始截取不会超过line width的数字
        number = start;
        width = char_len(number);
        while width <= self.line_width {
            numbers.push(number);
            number += self.step;
            width += 1 + char_len(number); //空格间隔
        }
    }

    // 随机产生每个gap有多少个位置（数字）
    fn gen_gaps(&self, gaps: &mut Vec<u16>) {
        let mut rng = rand::thread_rng();
        let die = Uniform::from(1..self.miss_max_per_gap + 1);
        for _ in 0..self.gaps_per_line {
            gaps.push(die.sample(&mut rng));
        }
    }
}

// 数字长度， 也可以转换为string再计算， 但性能更差
fn char_len(mut number: u16) -> u16 {
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

