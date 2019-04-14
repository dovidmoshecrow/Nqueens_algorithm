const N: i32 = 20;
fn main() {
    let mut grid: [[bool; N as usize]; N as usize] = [[false; N as usize]; N as usize];
    if !find_nqueens(&mut grid, 0) {
        println!("No solotion");
    } else {
        for e in grid.iter() {
            println!("{:?}", e);
        }
    }
}

fn find_nqueens(grid: &mut [[bool; N as usize]; N as usize], col: i32) -> bool {
    if col >= N {
        return true;
    };
    for row in 0..N {
        if check_pos(grid, row, col) {
            grid[row as usize][col as usize] = true;
            if find_nqueens(grid, col + 1) {
                return true;
            };
            grid[row as usize][col as usize] = false;
        };
    }
    false
}

fn check_pos(grid: &[[bool; N as usize]; N as usize], row: i32, col: i32) -> bool {
    let a = row + col;
    let b = row - col;
    for x in 0..N {
        for y in 0..N {
            if x + y == a || x - y == b || x == row || y == col {
                if grid[x as usize][y as usize] {
                    return false;
                }
            };
        }
    }
    true
}
