use std::io;
use std::io::Write; //for stdout flushing

//create grid
//REMINDER: grid[y][x]. y coordinate goes from top to bottom
#[rustfmt::skip] //rustfmt mangles this, so it's being ignored
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

//check for wins
fn win_check(grid: &[[String; 7]; 6]) -> bool {
    //vertical wins
    for y in 0..grid.len() - 3 {
        for x in 0..grid[0].len() {
            let piece = &grid[y][x];
            if piece != "-"
                && &grid[y][x] == piece
                && &grid[y + 1][x] == piece
                && &grid[y + 2][x] == piece
                && &grid[y + 3][x] == piece
            {
                return true;
            }
        }
    }
    //horizontal wins
    for y in 0..grid.len() {
        for x in 0..grid[0].len() - 3 {
            let piece = &grid[y][x];
            if piece != "-"
                && &grid[y][x] == piece
                && &grid[y][x + 1] == piece
                && &grid[y][x + 2] == piece
                && &grid[y][x + 3] == piece
            {
                return true;
            }
        }
    }
    //diagonal down wins
    for y in 0..grid.len() - 3 {
        for x in 0..grid[0].len() - 3 {
            let piece = &grid[y][x];
            if piece != "-"
                && &grid[y][x] == piece
                && &grid[y + 1][x + 1] == piece
                && &grid[y + 2][x + 2] == piece
                && &grid[y + 3][x + 3] == piece
            {
                return true;
            }
        }
    }
    //diagonal up wins
    for y in grid.len() - 4..grid.len() {
        for x in 0..grid.len() - 3 {
            let piece = &grid[y][x];
            if piece != "-"
                && &grid[y][x] == piece
                && &grid[y - 1][x + 1] == piece
                && &grid[y - 2][x + 2] == piece
                && &grid[y - 3][x + 3] == piece
            {
                return true;
            }
        }
    }
    false //does not need to be "return false;" because it's the last expression that can be returned
}

//main
fn main() {
    //build grid and set turn
    let mut grid = build_grid();
    let mut turn = "X".to_string();

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
        if win_check(&grid) {
            print_grid(&grid);
            println!("WINNER: {}", turn);
            break;
        }

        //advancing turn order
        if turn == "X" {
            turn = "O".to_string();
        } else {
            turn = "X".to_string();
        }
    }
}
