use std::io;

const FULL_GRID: u32 = 0b11101110111;

fn get_moves(grid: u32) -> [bool; 9] {
    let mut result = [false; 9];
    for y in 0..3 {
        for x in 0..3 {
            result[y * 3 + x] = grid & 1 << (y * 4 + x) == 0;
        }
    }
    result
}

fn _show_grid(player_1: u32, player_2: u32) {
    for y in 0..3 {
        let mut row = String::new();
        for x in 0..3 {
            let i = y * 4 + x;
            row += if player_1 & 1 << i != 0 {
                "1"
            } else if player_2 & 1 << i != 0 {
                "2"
            } else {
                "0"
            };
        }
        println!("{row}");
    }
}

fn make_move(grid: u32, r#move: usize) -> u32 {
    let offset = r#move / 3;
    grid | 1 << (r#move + offset)
}

fn is_winning(grid: u32) -> bool {
    // horizontal
    if grid & grid << 1 & grid << 2 != 0 {
        return true;
    }
    // vertical
    if grid & grid << 4 & grid << 8 != 0 {
        return true;
    }
    // top left -> bottom right
    if grid & grid << 5 & grid << 10 != 0 {
        return true;
    }
    // top right -> bottom left
    if grid & grid << 3 & grid << 6 != 0 {
        return true;
    }
    false
}

fn is_game_finished(grid: u32) -> bool {
    grid ^ FULL_GRID == 0
}

// return 42 when no move available
fn get_best_move(player_1: u32, player_2: u32, depth: i8) -> (usize, i8) {
    if is_winning(player_2) {
        return (42, depth - 9);
    }
    if is_game_finished(player_1 | player_2) {
        return (42, 0);
    }
    let mut best_score = -10;
    let mut best_move = 0;
    let moves = get_moves(player_1 | player_2);
    for i in 0..9 {
        if !moves[i] {
            continue;
        }

        let (_, score) = get_best_move(player_2, make_move(player_1, i), depth + 1);
        let score = -score;

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

fn get_player_move(player_1: u32, player_2: u32) -> usize {
    let available_moves = get_moves(player_1 | player_2);
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
    let mut player_1 = 0;
    let mut player_2 = 0;
    _show_grid(player_1, player_2);
    let result = loop {
        // check game status
        if is_winning(player_1) && is_bot_first || is_winning(player_2) && !is_bot_first {
            break "You lost";
        }
        if is_winning(player_2) && is_bot_first || is_winning(player_1) && !is_bot_first {
            break "You won";
        }
        if is_game_finished(player_1 | player_2) {
            break "Draw";
        }

        println!();

        // get move
        let r#move: usize;
        if is_bot_first && turn % 2 == 1 || !is_bot_first && turn % 2 == 0 {
            (r#move, _) = get_best_move(player_1, player_2, turn);
            let str_move = r#move + 1;
            println!("I play {str_move}");
        } else {
            println!("Play a move");
            r#move = get_player_move(player_1, player_2);
        }

        // make move
        if turn % 2 == 1 {
            player_1 = make_move(player_1, r#move);
        } else {
            player_2 = make_move(player_2, r#move);
        }
        turn += 1;
        _show_grid(player_1, player_2);
    };
    println!("{result}");
}

fn main() {
    launch_game();
}
