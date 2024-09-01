/// 数字生成器
pub struct NumberGenerator {

    // 数字开始时
    begin: usize,

    // 数字结束时
    end: usize
}

impl NumberGenerator {
    fn new(begin: usize, end: usize) -> Self {
        Self {
            begin,
            end
        }
    }
}

impl Iterator for NumberGenerator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.begin < self.end {
            let res = self.begin;
            self.begin += 1;
            Some(res)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it_work() {
        let nums = NumberGenerator::new(90, 100);
        for i in nums {
            println!("i = {}",i);
        }
    }
}
