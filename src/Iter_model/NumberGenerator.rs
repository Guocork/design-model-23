use std::ops::Range;

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

fn count_iterator<T: Iterator>(range: T) -> i32 {
    let mut a = 0;
    for _ in range {
        a += 1;
    }
    a
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_it_work1() {
        let nums = NumberGenerator::new(90, 100);
        for i in nums {
            println!("i = {}",i);
        }
    }

    #[test]
    fn test_it_work2() {
        let nums1 = NumberGenerator::new(10, 100).filter(|num| num % 3 == 0 && num % 7 ==0);

        for n in nums1 {
            println!("n = {n}");
        }
    }

    #[test]
    fn test_it_work3() {
        (0..5).filter(|i| i % 3 == 0).for_each(|e| println!("{e}"));
    }

    #[test]
    fn test_it_work4() {
        (0..5).map(|i| i as f64).map(|n| n.sqrt()).for_each(|n| println!("{n}"));
    }

    #[test]
    fn test_it_work5() {
        let a = count_iterator(0..=500);
        println!("{a}");
    }

    #[test]
    fn test_it_work6() {
        println!("sum = {}", (1..=100).sum::<usize>());
    }

    #[test]
    fn test_it_work7() {
        let a = (0..5).collect::<Vec<usize>>();
        assert_eq!(a, vec![0, 1, 2, 3, 4]);
        let res = (0..50).map(|n| if n <= 3 {
            Some(n)
        } else { None} ).collect::<Option<Vec<usize>>>();
        assert_eq!(res, None);
    }

    #[test]
    fn test_it_work8() {
        (0..5).map(|n| if n > 3  {
            3
        } else {
            n
        }).collect::<HashSet<usize>>().iter().for_each(|i| println!("{i}"));
    }

    #[test]
    fn test_it_work9() {
        vec![Some(1), None, Some(2)]
            .into_iter()
            .flatten()
            .collect::<Vec<usize>>()
            .into_iter()
            .for_each(|n| println!("{n}"));
    }
}
