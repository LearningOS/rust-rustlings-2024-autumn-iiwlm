/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }
    pub fn clear(&mut self) {
        while !self.is_empty(){
            self.elements.pop();
        }
    }
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

// 基于两个队列的栈 (myStack)，我们需要设计一种机制来保证最后进入队列的元素能够最先被弹出
#[derive(Debug)]
pub struct myStack<T>
{
	//TODO
    top :Option<usize>,
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
            top:None,
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    
    pub fn push(&mut self, elem: T) {
        if let Some(a) = self.top{
            self.top=Some(a-1);
        };
        std::mem::swap(&mut self.q1, &mut self.q2);
        self.q1.enqueue(elem);
        while !self.q2.is_empty(){
            match self.q2.dequeue(){
                Ok(a) =>self.q1.enqueue(a),
                _other => (),
            }

        } 
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        if self.q1.is_empty() {return Err("Stack is empty");}
        if let Some(a) = self.top{
        self.top=Some(a-1);

        };
        self.q1.dequeue()
    }

    pub fn is_empty(&self) -> bool {
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        println!("{:?}", s);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}