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
  println!("\n0 1 2 3 4 5 6");
  for row in grid {
    for elem in row {
      print!("{} ", elem);
    }
    print!("\n");
  }
  print!("----------");
}

//main
fn main() {
  //build grid and set turn
  let mut grid = build_grid();
  let mut turn = String::from("X");
  
  //main gameloop
  loop {
    //PLACEHOLDER: Testing grid printing
    print_grid(&grid);
    break;
  }
}