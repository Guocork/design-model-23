
#[derive(Debug)]
struct Stack<T> {
    
    size: usize,

    data: Vec<T>
}

impl<T> Stack<T> {
    fn new() -> Self{
        Self { size: 0, data: vec![] }
    }

    // 判断栈是否为空
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    // 栈的长度
    fn len(&self) -> usize {
        self.size
    }

    // 清空栈
    fn clear(&mut self) {
        self.data.clear();
        self.size = 0;
    }

    // push添加元素
    fn push(&mut self, value: T) {
        self.data.push(value);
        self.size += 1;
    }

    // pop元素
    fn pop(&mut self) -> Option<T> {
        self.size -= 1;
        self.data.pop()
    }

    // 返回栈顶的不可变引用
    fn peek(&self) -> Option<&T> {
        self.data.get(self.size -1)
    }

    // 返回栈顶数据的可变引用
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.get_mut(self.size -1)
    }

    // 将集合转化成迭代器
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // 将集合转化为不可变引用迭代器
    fn iter(&self) -> Iter<T> {
        let mut iter = Iter(Vec::new());
        for item in self.data.iter() {
            iter.0.push(item);
        }
        iter
    }

    // 将集合转化成可变引用迭代器 
    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iter = IterMut(Vec::new());
        for item in self.data.iter_mut() {
            iter.0.push(item);
        }
        iter
    }

}

// 消费集合数据变成一个迭代器 
struct IntoIter<T>(Stack<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}

// 不可变引用迭代器
struct Iter<'a, T: 'a>(Vec<&'a T>);
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// 可变引用迭代器 
struct IterMut<'a, T: 'a>(Vec<&'a mut T>);
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it_work() {
        
    }
}