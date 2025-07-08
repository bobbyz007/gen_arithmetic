use std::cmp::{max, min};
use docx_rs::{Paragraph};
use rand::{rng, Rng};
use rand::distr::Uniform;
use crate::MissingNumberOpts;
use crate::utils::{add_paragraph, char_len, read_from_docx, write, write_to_docx};

impl MissingNumberOpts {
    pub fn gen_missing_numbers_to_docx(&self) {
        let mut doc = read_from_docx("./resources/template.docx");

        for _i in 0..self.count {
            let line = &self.gen_single_missing_numbers();
            let font_size = self.output_docx_font_size as usize;
            doc = add_paragraph(doc, font_size, line);
            doc = doc.add_paragraph(Paragraph::new().size(font_size))
        }

        write_to_docx(doc, "./output/missing-numbers.docx");
    }

    fn gen_single_missing_numbers(&self) -> String {
        let mut gaps = vec![];
        self.gen_gaps(&mut gaps);
        // 所有的gap对应的number数量
        let all_gap_numbers = gaps.iter().fold(0, |acc, &x| acc + x);
        // 满足能插入所有gap 所需要的最少number数量（每个gap至少间隔一个数字）
        let min_numbers = all_gap_numbers + gaps.len() as u16 - 1;

        // 生成数字
        let mut numbers: Vec<u16> = vec![];
        self.gen_numbers(&mut numbers, min_numbers);

        let mut line = String::new();
        // numbers中的插入gap的索引位置
        let mut number_pos: u16 = 0;
        let mut miss_numbers = min_numbers;
        if numbers.len() < (miss_numbers as usize) {
            eprintln!("numbers generated: {:?} with min count: {:?}", numbers, miss_numbers);
            panic!("The count of numbers generated is too small");
        }
        // 在数字序列中插入gap
        for gap in gaps {
            // 随机数的范围
            let upper_bound = numbers.len() as u16 - miss_numbers + 1;
            let gap_start = rng().random_range(number_pos..upper_bound);

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
        // 从start开始截取不会超过line width的数字
        let mut number = self.gen_num_start(min_numbers);
        let mut width = char_len(number);
        let step = i16::unsigned_abs(self.step);
        while width <= self.line_width {
            numbers.push(number);
            number = if self.step > 0 {
                number + step
            } else {
                if number < step {
                    break;
                }
                number - step
            };
            width += 1 + char_len(number); //空格间隔
        }
    }

    fn gen_num_start(&self, min_numbers: u16) -> u16 {
        let (mut lower_bound, mut upper_bound) = (self.number_min_inclusive, self.number_max_inclusive);
        let step = i16::unsigned_abs(self.step);
        if self.step > 0 {
            upper_bound = self.number_max_inclusive - min_numbers * step;

            let mut number = self.number_max_inclusive;
            let mut width = char_len(number);
            while width <= self.line_width {
                if number < step {
                    break;
                }
                number -= step;
                // 数字和间隔空格的宽度
                width += char_len(number) + 1;
            }
            number += step;
            // 既满足step要求， 又满足line width的要求
            upper_bound = min(upper_bound, number);
            if upper_bound < self.number_min_inclusive {
                panic!("Please shorten step or increase line width");
            }
        } else {
            lower_bound = self.number_min_inclusive + min_numbers * step;

            let mut number = self.number_min_inclusive;
            let mut width = char_len(number);
            while width <= self.line_width {
                number += step;
                // 数字和间隔空格的宽度
                width += char_len(number) + 1;
            }
            number -= step;
            // 既满足step要求， 又满足line width的要求
            lower_bound = max(lower_bound, number);

            // 上限只是一个参考值，允许超过上限
        }

        let mut rng = rand::rng();
        upper_bound = if lower_bound == upper_bound { upper_bound + 1} else { upper_bound };
        let die = Uniform::new(lower_bound, upper_bound).unwrap();
        // 从start开始满足gap要求，每个gap都至少间隔了一个数字，而且line width不会超过数字范围
        let mut start = rng.sample(die);
        if self.start_as_multiple_step {
            if start % step == 0 {
                return start;
            } else {
                let mut times = start / step;
                let r = start % step;
                times = if r > step / 2 { times + 1 } else { times };
                start = times * step;
                // 如果超过下限，则直接向上
                if start < self.number_min_inclusive {
                    start = (times + 1) * step;
                }
            }
        }
        start
    }

    // 随机产生每个gap有多少个位置（数字）
    fn gen_gaps(&self, gaps: &mut Vec<u16>) {
        let mut rng = rng();
        let die = Uniform::new(1, self.miss_max_per_gap + 1).unwrap();
        for _ in 0..self.gaps_per_line {
            gaps.push(rng.sample(die));
        }
    }

    #[allow(dead_code)]
    fn gen_missing_numbers_to_txt(&self) {
        let mut lines = String::new();
        for _i in 0..self.count {
            lines.push_str(&self.gen_single_missing_numbers());
            lines.push_str("\n");
        }
        write(&lines.trim(), "./output/missing-numbers.txt").expect("Write error!");
        println!("Generate missing numbers successfully");
    }
}

