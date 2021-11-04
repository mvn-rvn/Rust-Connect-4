use std::io;

//create grid
fn build_grid() -> [[String; 7]; 6]  {
  let grid: [[String; 7]; 6] = [
    ["-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string()],
    ["-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string()],
    ["-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string()],
    ["-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string()],
    ["-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string()],
    ["-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string(), "-".to_string()],
  ];
  grid
}

//print grid to console
fn print_grid(grid: &[[String; 7]; 6]) {
  print!("\n0 1 2 3 4 5 6\n");
  for row in grid {
    for elem in row {
      print!("{} ", elem);
    }
    print!("\n");
  }
  print!("====================\n");
}

//main
fn main() {
  //build grid and set turn
  let mut grid = build_grid();
  let mut turn = String::from("X");
  
  //main gameloop
  loop {
    //print grid
    print_grid(&grid);

    //get player input
    print!("{}'s turn.\n", turn);
    let mut action = String::new();
    io::stdin()
      .read_line(&mut action)
      .expect("failed to read input");
    let action: usize = action.trim().parse::<usize>().expect("Your input was not a number");

    //setting up variables for piece-placing calculations
    let mut bottom = false;
    let mut row = 0;

    //placing piece
    while !bottom {
      //place piece at the bottom of the column
      if grid[0][action] == "-" && (row+1 == grid.len() || grid[row+1][action] != "-".to_string()) {
        grid[row][action] = turn.clone();
        bottom = true;
      } 
      //check if column is full
      else if grid[0][action] != "-" {
        print!("That column is full.\n");
        bottom = true;
      }
      //advance row
      row += 1;
    }
    
    //checking for wins
  }
}