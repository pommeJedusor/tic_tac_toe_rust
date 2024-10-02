use std::io;

type Grid = [[u32; 3]; 3];

fn get_grid() -> [[u32; 3]; 3] {
    return [[0; 3]; 3];
}

fn get_moves(grid: Grid) -> [bool; 9] {
    let mut result = [false; 9];
    for y in 0..3 {
        for x in 0..3 {
            result[y * 3 + x] = grid[y][x] == 0;
        }
    }

    result
}

fn _show_grid(grid: Grid) {
    for y in 0..3 {
        let mut row = String::new();
        for x in 0..3 {
            row.push(char::from_digit(grid[y][x], 10).unwrap());
        }
        println!("{row}");
    }
}

fn make_move(grid: &mut Grid, r#move: usize, player: u32) {
    let y: usize = r#move / 3;
    let x: usize = r#move % 3;
    grid[y][x] = player;
}

fn cancel_move(grid: &mut Grid, r#move: usize) {
    let y: usize = r#move / 3;
    let x: usize = r#move % 3;
    grid[y][x] = 0;
}

fn is_winning(grid: Grid) -> u32 {
    for i in 0..3 {
        // horizontal
        if grid[i][0] != 0 && grid[i][0] == grid[i][1] && grid[i][2] == grid[i][1] {
            return grid[i][0];
        }
        // vertical
        if grid[0][i] != 0 && grid[0][i] == grid[1][i] && grid[2][i] == grid[1][i] {
            return grid[0][i];
        }
    }
    // top left -> bottom right
    if grid[1][1] != 0 && grid[0][0] == grid[1][1] && grid[2][2] == grid[1][1] {
        return grid[1][1];
    }
    // top right -> bottom left
    if grid[1][1] != 0 && grid[0][2] == grid[1][1] && grid[2][0] == grid[1][1] {
        return grid[1][1];
    }

    0
}

fn is_game_finished(grid: Grid) -> bool {
    for y in 0..3 {
        for x in 0..3 {
            if grid[y][x] == 0 {
                return false;
            }
        }
    }

    true
}

// return 42 when no move available
fn get_best_move(grid: &mut Grid, depth: i8) -> (usize, i8) {
    if is_winning(*grid) != 0 {
        return (42, depth - 9);
    }
    if is_game_finished(*grid) {
        return (42, 0);
    }
    let mut best_score = -10;
    let mut best_move = 0;
    let moves = get_moves(*grid);
    for i in 0..9 {
        if !moves[i] {
            continue;
        }

        make_move(grid, i, u32::try_from((depth + 1) % 2 + 1).unwrap());
        let (_, score) = get_best_move(grid, depth + 1);
        let score = -score;
        cancel_move(grid, i);

        if score > best_score {
            best_score = score;
            best_move = i;
        }
    }
    return (best_move, best_score);
}

fn is_player_first() -> bool {
    loop {
        let mut result = String::new();
        io::stdin()
            .read_line(&mut result)
            .expect("Failed to read line");
        let result = result.trim();
        if result == "y" || result == "Y" {
            break true;
        } else if result == "n" || result == "N" {
            break false;
        } else {
            println!("invalid input '{result}'");
        }
    }
}

fn get_player_move(grid: Grid) -> usize {
    let available_moves = get_moves(grid);
    loop {
        // get input
        let mut result = String::new();
        io::stdin()
            .read_line(&mut result)
            .expect("Failed to read line");

        // str -> int
        let result: usize = match result.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Number's not valid");
                continue;
            }
        };

        // check move's inside the grid
        if result < 1 || result > 9 {
            println!("move's outside the grid");
            continue;
        }

        // trad the move
        let result = result - 1;

        // check move's validity
        if available_moves[result] {
            return result;
        }
        println!("Move's not a valid one");
    }
}

fn launch_game() {
    println!("-- Welcome to this tic-tac-toe game --");
    println!("-- Do you want to start? [y/n] --");
    let is_bot_first = !is_player_first();
    let mut turn = 1;
    let mut grid = get_grid();
    _show_grid(grid);
    let result = loop {
        // check game status
        if is_winning(grid) == 1 && is_bot_first || is_winning(grid) == 2 && !is_bot_first {
            break "You lost";
        }
        if is_winning(grid) == 2 && is_bot_first || is_winning(grid) == 1 && !is_bot_first {
            break "You won";
        }
        if is_game_finished(grid) {
            break "Draw";
        }

        println!();

        // get move
        let r#move: usize;
        if is_bot_first && turn % 2 == 1 || !is_bot_first && turn % 2 == 0 {
            (r#move, _) = get_best_move(&mut grid, turn);
            let str_move = r#move + 1;
            println!("I play {str_move}");
        } else {
            println!("Play a move");
            r#move = get_player_move(grid);
        }

        // make move
        if turn % 2 == 1 {
            make_move(&mut grid, r#move, 1);
        } else {
            make_move(&mut grid, r#move, 2);
        }
        turn += 1;
        _show_grid(grid);
    };
    println!("{result}");
}

fn main() {
    launch_game();
}
