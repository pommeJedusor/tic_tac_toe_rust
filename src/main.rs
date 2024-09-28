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
    grid ^ 0b11101110111 == 0
}

fn get_score(player_1: u32, player_2: u32, depth: i8) -> i8 {
    if is_winning(player_2) {
        return depth - 9;
    }
    if is_game_finished(player_1 | player_2) {
        return 0;
    }
    let mut best_score = -10;
    let moves = get_moves(player_1 | player_2);
    for i in 0..9 {
        if !moves[i] {
            continue;
        }

        let score = -get_score(player_2, make_move(player_1, i), depth + 1);

        if score > best_score {
            best_score = score;
        }
    }
    return best_score;
}

fn main() {
    let player_1 = 0;
    let player_2 = 0;
    _show_grid(player_1, player_2);
    let score = get_score(player_1, player_2, 1);
    println!("the best score = {score}");
}
