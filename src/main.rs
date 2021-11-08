use std::fmt::{Display, Formatter};
use std::io;
use std::io::Write;

const WIDTH: usize = 7;
const HEIGHT: usize = 6;

#[derive(Copy, Clone, Eq, PartialEq)]
enum State {
    Free,
    X,
    O,
}

impl Default for State {
    fn default() -> Self {
        State::Free
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Free => f.write_str("-"),
            State::X => f.write_str("X"),
            State::O => f.write_str("O"),
        }
    }
}

type Grid = [[State; WIDTH]; HEIGHT];

//print grid to console
fn print_grid(grid: &Grid) {
    for x in 0..WIDTH {
        print!("{} ", x); //printing numbers on top
    }

    println!();

    for row in grid {
        for elem in row {
            print!("{} ", elem);
        }

        println!();
    }

    println!("====================\n");
}

//check for wins
fn win_check(grid: &Grid) -> bool {
    //vertical wins
    for y in 0..HEIGHT - 3 {
        for x in 0..WIDTH {
            let piece = grid[y][x];
            if piece != State::Free
                && grid[y][x] == piece
                && grid[y + 1][x] == piece
                && grid[y + 2][x] == piece
                && grid[y + 3][x] == piece
            {
                return true;
            }
        }
    }
    //horizontal wins
    for y in 0..HEIGHT {
        for x in 0..WIDTH - 3 {
            let piece = grid[y][x];
            if piece != State::Free
                && grid[y][x] == piece
                && grid[y][x + 1] == piece
                && grid[y][x + 2] == piece
                && grid[y][x + 3] == piece
            {
                return true;
            }
        }
    }
    //diagonal down wins
    for y in 0..HEIGHT - 3 {
        for x in 0..WIDTH - 3 {
            let piece = grid[y][x];
            if piece != State::Free
                && grid[y][x] == piece
                && grid[y + 1][x + 1] == piece
                && grid[y + 2][x + 2] == piece
                && grid[y + 3][x + 3] == piece
            {
                return true;
            }
        }
    }
    //diagonal up wins
    for y in HEIGHT - 4..HEIGHT {
        for x in 0..HEIGHT - 3 {
            let piece = grid[y][x];
            if piece != State::Free
                && grid[y][x] == piece
                && grid[y - 1][x + 1] == piece
                && grid[y - 2][x + 2] == piece
                && grid[y - 3][x + 3] == piece
            {
                return true;
            }
        }
    }
    false //does not need to be "return false;" because it's the last expression that can be returned
}

//main
fn main() {
    // build grid and set turn
    // REMINDER: grid[y][x]. y coordinate goes from top to bottom
    let mut grid = Grid::default();
    let mut turn = State::X;

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
            if grid[0][action] == State::Free
                && (row + 1 == HEIGHT || grid[row + 1][action] != State::Free)
            {
                grid[row][action] = turn;
                bottom = true;
            }
            //check if column is full
            else if grid[0][action] != State::Free {
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
        turn = match turn {
            State::Free => panic!("how did we get here?"),
            State::X => State::O,
            State::O => State::X,
        };
    }
}