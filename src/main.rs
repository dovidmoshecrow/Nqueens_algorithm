fn main() {
  let mut grid :[[bool;8];8] = [[false;8];8];
  if !minimax(&mut grid,0){
      println!("No solotion");  
  }else {
       for e in grid.iter(){
           println!("{:?}",e);
       }
  }
}

fn minimax(grid:&mut [[bool; 8]; 8],col:i32) -> bool{
   if col >= 8{
       return true;
   };
   for row in 0..8{
     if check_pos(grid,row,col) {
       grid[row as usize][col as usize] = true;
     if minimax(grid,col + 1){
        return true;
     };
     grid[row as usize][col as usize] = false;
   };
   };
   false
}



fn check_pos(grid:&[[bool;8];8],row:i32,col:i32)-> bool {
   for i in 0..8 {
       if grid[row as usize][i] {
          return false;
       }
   };
   for i in 0..8 {
       if grid[i][col as usize] {
           return false;
       }
   };
   let a = row + col;
   let b = row - col;
   for x in 0..8{
       for y in 0..8{
             if x + y == a || x - y == b{
                if grid[x as usize][y as usize]{
                    return false;
                }
             };
       };
   };
   true
}