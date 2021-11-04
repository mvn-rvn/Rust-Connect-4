use std::io;
use std::io::Write; //for stdout flushing

//create grid
#[rustfmt::skip] //rustfmt mangles this, so its being ignored
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
        println!();
    }
    println!("====================");
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
        print!("{}'s turn: ", turn);
        io::stdout().flush().unwrap(); //FOR SOME REASON, printing without \n doesn't show up without flushing stdout
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("failed to read input");
        //parse input to usize for indexing
        let action: usize = action
            .trim()
            .parse::<usize>()
            .expect("Your input was not a number");

        //setting up variables for piece-placing calculations
        let mut bottom = false;
        let mut row = 0;

        //placing piece
        while !bottom {
            //place piece at the bottom of the column
            if grid[0][action] == "-" && (row + 1 == grid.len() || grid[row + 1][action] != "-") {
                grid[row][action] = turn.clone();
                bottom = true;
            }
            //check if column is full
            else if grid[0][action] != "-" {
                println!("That column is full.");
                bottom = true;
            }
            //advance row
            row += 1;
        }

        //checking for wins
    }
}
