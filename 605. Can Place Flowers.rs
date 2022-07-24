impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        for i in 0..flowerbed.len() {
          if n == 0 {break;}
          if flowerbed[i] == 0 {
            if i == 0 && (flowerbed.len() == 1 || flowerbed[i+1] == 0) {
              flowerbed[i] = 1;
              n-=1;
            }
            else if i == flowerbed.len() - 1 &&  flowerbed[i-1] == 0{
              flowerbed[i] = 1;
              n-=1;
            }
            else if i > 0 && i < flowerbed.len() - 1 && flowerbed[i+1] == 0 && flowerbed[i-1] == 0 {
              flowerbed[i] = 1;
              n-=1;
            }
          }
        }
        match n == 0 {
          true => true,
          false => false
        }
    }
  }