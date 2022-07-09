use std::convert::TryInto;
use std::collections::VecDeque;
use std::collections::HashMap;


struct LRUCache {
  catch: VecDeque<i32>,
  map: HashMap<i32, i32>,
  cap: usize,
}
impl LRUCache {

  fn new(capacity: i32) -> Self {
    Self{
      catch:VecDeque::new(),
      map: HashMap::new(),
      cap:capacity as usize,
    }
  }

  fn get(&mut self, key: i32) -> i32 {
    if self.map.contains_key(&key){
      let pos = self.catch.iter().position(|&x| x == key).unwrap();
      self.catch.remove(pos);
      self.catch.push_front(key);
      return *self.map.get(&key).unwrap();
    }
    -1
  }

  fn put(&mut self, key: i32, value: i32) {
    match self.map.contains_key(&key){
      true=>{
        let pos = self.catch.iter().position(|&x| x == key).unwrap();
        self.catch.remove(pos);
        self.catch.push_front(key);
        self.map.insert(key,value);
      },
      false=>{
        if self.catch.len() == self.cap{
          let last = self.catch.pop_back().unwrap();
          self.map.remove(&last);
        }
        self.catch.push_front(key);
        self.map.insert(key,value);
      }
    }
  }
}