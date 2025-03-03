use super::container::Container;

pub struct Stack<T>{
    items: Vec<T>,
}
impl<T> Stack<T> {
    pub fn new(items: Vec<T>) -> Self {
        Stack { items }
     }
 }
impl <T> Container<T> for Stack<T> {
    
      fn put(&mut self, item: T) {
        self.items.push(item);
    }
      fn get(&mut self) -> Option<T> {
        self.items.pop()
    }
      fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}