mod ai;
mod board;

use std::io::{self, Read, Write};

use crate::{
    ai::estimate,
    board::{get_other_player, Board, Player, TileCoord},
};

enum Prompt<T> {
    Valid(T),
    Invalid,
    Abort,
}

fn get_position_from_stdin(stdin: &mut io::StdinLock) -> Prompt<TileCoord> {
    let mut in_buf = [0, 0, 0]; // one for newline.
    stdin
        .read(&mut in_buf)
        .expect("failed reading into buffer.");
    if in_buf[0] == 'q' as u8 {
        return Prompt::Abort;
    }
    if in_buf[0] < 'a' as u8 {
        println!("Did not understand x coordinate. Must be a lower-case letter.");
        return Prompt::Invalid;
    }
    if in_buf[1] < '1' as u8 {
        println!("Did not understand y coordinate. Must be a digit.");
        return Prompt::Invalid;
    }
    let c = (
        (in_buf[0] - 'a' as u8).into(),
        (in_buf[1] - '1' as u8).into(),
    );
    return Prompt::Valid(c);
}

fn coord_to_position((x, y): TileCoord) -> String {
    let mut out = String::new();
    out.push(((x as u8) + 'a' as u8) as char);
    out.push(((y as u8) + '1' as u8) as char);
    out
}

fn get_next_valid_move_from_stdin(
    board: &Board,
    player_to_move: Player,
    stdin: &mut io::StdinLock,
) -> (TileCoord, TileCoord) {
    let mut from_pos = (0, 0);
    let mut piece_to_move = None;
    while piece_to_move.is_none() {
        println!("{}", board);
        print!("{} to move. Select your piece: ", player_to_move);
        io::stdout().flush().expect("failed writing to stdout.");

        match get_position_from_stdin(stdin) {
            Prompt::Valid(p) => {
                from_pos = p;
            }
            Prompt::Invalid => {
                println!("Not a valid coordinate! must be a1 to g9.");
                continue;
            }
            Prompt::Abort => {
                continue;
            }
        };

        match board.get_piece_at(from_pos) {
            None => println!("No player piece at coordinate."),
            Some((owner, p)) => {
                if owner != player_to_move {
                    println!("That is not your piece.")
                } else {
                    piece_to_move = Some(p)
                }
            }
        }
    }

    let possible_moves: Vec<TileCoord> = board
        .get_next_moves(player_to_move)
        .iter()
        .filter(|(p, _, _)| piece_to_move.unwrap() == *p)
        .flat_map(|(_, _, ms)| -> Vec<TileCoord> { ms.iter().map(|m| *m).collect() })
        .collect();
    if possible_moves.is_empty() {
        println!("Cannot move that piece! Select another.");
        return get_next_valid_move_from_stdin(board, player_to_move, stdin);
    }
    let mut to_pos = None;
    while to_pos.is_none() {
        println!(
            "Selected {}. Options are: {} or q to select another piece.",
            piece_to_move.unwrap(),
            possible_moves
                .iter()
                .map(|m| format!("{}", coord_to_position(*m)))
                .reduce(|accum, item| format!("{}, {}", accum, item))
                .unwrap()
        );
        print!("Where to move? ");
        io::stdout().flush().expect("failed writing to stdout.");

        match get_position_from_stdin(stdin) {
            Prompt::Valid(p) => {
                if possible_moves.iter().any(|m: &TileCoord| *m == p) {
                    to_pos = Some(p);
                }
            }
            Prompt::Invalid => {
                println!("Not a valid coordinate! must be a1 to g9.");
                continue;
            }
            Prompt::Abort => return get_next_valid_move_from_stdin(board, player_to_move, stdin),
        };
    }

    (from_pos, to_pos.unwrap())
}

fn main() {
    let mut b = Board::new();

    let mut stdin = io::stdin().lock();
    let mut player_to_move = Player::Player1;

    loop {
        dbg!(estimate(player_to_move, &b));
        let (from_pos, to_pos) = get_next_valid_move_from_stdin(&b, player_to_move, &mut stdin);
        let (new_b, winner, caps) = b
            .make_move(player_to_move, from_pos, to_pos)
            .expect("Somehow an invalid move got through.");

        println!(
            "{} {}",
            winner.map_or_else(|| "".to_string(), |p| format!("{} won the game!", p)),
            caps.map_or_else(
                || "".to_string(),
                |(p, c)| format!("captured {}'s {}.", p, c)
            )
        );
        if winner.is_some() {
            break;
        }
        b = new_b;

        player_to_move = get_other_player(player_to_move);
    }
}
