/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/
#[derive(Debug)]
struct Stack<T> {
	size: usize,
	data: Vec<T>,
}
impl<T> Stack<T> {
	fn new() -> Self {
		Self {
			size: 0,
			data: Vec::new(),
		}
	}
	fn is_empty(&self) -> bool {
		0 == self.size
	}
	fn len(&self) -> usize {
		self.size
	}
	fn clear(&mut self) {
		self.size = 0;
		self.data.clear();
	}
	fn push(&mut self, val: T) {
		self.data.push(val);
		self.size += 1;
	}
	// if let 活学活用,没用好
	// 但是对于pop的实现方法,他使用了unsafe获取地址实现
	fn pop(&mut self) -> Option<T> {
		// TODO
		if self.size > 0 {
            self.size -= 1;
            self.data.pop()
        } else {
            None
        }
	}
	fn peek(&self) -> Option<&T> {
		if 0 == self.size {
			return None;
		}
		self.data.get(self.size - 1)
	}
	fn peek_mut(&mut self) -> Option<&mut T> {
		if 0 == self.size {
			return None;
		}
		self.data.get_mut(self.size - 1)
	}
	fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
	fn iter(&self) -> Iter<T> {
		let mut iterator = Iter { 
			stack: Vec::new() 
		};
		for item in self.data.iter() {
			iterator.stack.push(item);
		}
		iterator
	}
	fn iter_mut(&mut self) -> IterMut<T> {
		let mut iterator = IterMut { 
			stack: Vec::new() 
		};
		for item in self.data.iter_mut() {
			iterator.stack.push(item);
		}
		iterator
	}
}
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.0.is_empty() {
			self.0.size -= 1;self.0.data.pop()
		} 
		else {
			None
		}
	}
}
struct Iter<'a, T: 'a> {
	stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}
struct IterMut<'a, T: 'a> {
	stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}

fn bracket_match(bracket: &str) -> bool
{


	//3. 使用双向链表
	// use std::collections::VecDeque;
    // let mut stack = VecDeque::new();
    // for c in bracket.chars() {
    //     match c {
    //         '(' | '[' | '{' => stack.push_back(c),
    //         ')' => {
    //             if stack.pop_back() != Some('(') {
    //                 return false;
    //             }
    //         },
    //         ']' => {
    //             if stack.pop_back() != Some('[') {
    //                 return false;
    //             }
    //         },
    //         '}' => {
    //             if stack.pop_back() != Some('{') {
    //                 return false;
    //             }
    //         },
    //         _ => {} // 忽略其他字符
    //     }
    // }
    // stack.is_empty()

	// 2.对于你的实现进行改进:对于后半部分直接匹配,不进行入栈
	let mut stack = Vec::new();
    for c in bracket.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            },
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            },
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            },
            _ => {} // 忽略其他字符
        }
    }
    stack.is_empty()
	//1. 嵌套就不能实现功能了TODO{([])}
	// if bracket.is_empty(){
	// 	return true;
	// }
	// let front = ['[','(','{'];
	// let back= [']',')','}'];
	// let mut stack_front = Stack::new();
	// let mut stack_back = Stack::new();
	// for i in bracket.chars(){
	// 	if front.contains(&i){
	// 		stack_front.push(i);
	// 	}
	// 	if back.contains(&i){
	// 		stack_back.push(i);
	// 	}
	// }
	// if stack_front.size != stack_back.size{
	// 	return false;
	// }
	// loop{
	// 	let Some(back) = stack_back.pop()else{break;};
	// 	match stack_front.pop(){
	// 		None=>break,
	// 		Some(a) => {
	// 			if a == '(' { if back != ')' {return false;} }
	// 			else if a == '[' { if back != ']'{return false;} }
	// 			else{ if back != '}'{return false;}}
	// 		}
	// 	}
	// }
	// true
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1(){
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_2(){
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_3(){
		let s = "{{([])}}";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_4(){
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_5(){
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_6(){
		let s = "";
		assert_eq!(bracket_match(s),true);
	}
}