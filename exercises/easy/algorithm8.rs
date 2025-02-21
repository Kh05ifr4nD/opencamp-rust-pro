/*
  queue
  This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
  data: Vec<T>,
}

impl<T> Queue<T> {
  pub fn new() -> Queue<T> {
    Queue { data: Vec::new() }
  }

  pub fn push(&mut self, val: T) {
    self.data.push(val);
  }

  pub fn pop(&mut self) -> Option<T> {
    if self.empty() {
      None
    } else {
      Some(self.data.remove(0))
    }
  }

  pub fn front(&self) -> Option<&T> {
    self.data.first()
  }

  pub fn back(&self) -> Option<&T> {
    self.data.last()
  }

  pub fn size(&self) -> usize {
    self.data.len()
  }

  pub fn empty(&self) -> bool {
    self.data.is_empty()
  }
}

impl<T> Default for Queue<T> {
  fn default() -> Queue<T> {
    Queue { data: Vec::new() }
  }
}

pub struct DummyStack<T> {
  q1: Queue<T>,
  q2: Queue<T>,
}
impl<T> DummyStack<T> {
  pub fn new() -> Self {
    Self { q1: Queue::new(), q2: Queue::new() }
  }
  pub fn push(&mut self, elem: T) {
    self.q1.push(elem);
  }
  pub fn pop(&mut self) -> Result<T, &str> {
    if self.empty() {
      return Err("Stack is empty");
    }

    while self.q1.size() > 1 {
      if let Some(val) = self.q1.pop() {
        self.q2.push(val);
      }
    }

    let rslt = self.q1.pop().ok_or("Stack is empty");

    std::mem::swap(&mut self.q1, &mut self.q2);

    rslt
  }

  pub fn empty(&self) -> bool {
    self.q1.empty() && self.q2.empty()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_queue() {
    let mut s = DummyStack::<i32>::new();
    assert_eq!(s.pop(), Err("Stack is empty"));
    s.push(1);
    s.push(2);
    s.push(3);
    assert_eq!(s.pop(), Ok(3));
    assert_eq!(s.pop(), Ok(2));
    s.push(4);
    s.push(5);
    assert_eq!(s.empty(), false);
    assert_eq!(s.pop(), Ok(5));
    assert_eq!(s.pop(), Ok(4));
    assert_eq!(s.pop(), Ok(1));
    assert_eq!(s.pop(), Err("Stack is empty"));
    assert_eq!(s.empty(), true);
  }
}
